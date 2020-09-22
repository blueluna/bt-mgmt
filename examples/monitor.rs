use std::os::unix::io::AsRawFd;

use mio::{unix::SourceFd, Events, Interest, Poll, Token};

use byteorder::{ByteOrder, LittleEndian};

use bt_mgmt::{
    self,
    eir::{self, EirEntry},
    events::Event,
    pack::UnpackFixed,
    Error, Socket,
};

const MGMT_EVENTS: Token = Token(0);

fn main() -> Result<(), Error> {
    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(128);
    let mut mgmt = Socket::new()?;
    poll.registry().register(
        &mut SourceFd(&mgmt.as_raw_fd()),
        MGMT_EVENTS,
        Interest::READABLE,
    )?;
    let mut buffer = [0u8; 1024];
    loop {
        poll.poll(&mut events, None)?;
        for event in events.iter() {
            match event.token() {
                MGMT_EVENTS => {
                    let (size, event, index) = mgmt.receive_event(&mut buffer)?;
                    let (event, _) = Event::unpack(event, &buffer[..size])?;
                    let hex: String = buffer[..size]
                        .iter()
                        .map(|i| format!("{:02x}", i))
                        .collect();
                    match event {
                        Event::Discovering(event) => {
                            println!("Event {} {:?}", index, event);
                        }
                        Event::DeviceFound(event) => {
                            print!(
                                "Event {} Device found {} {:4} {:08x}",
                                index, event.address_info, event.rssi, event.flags
                            );
                            let length = event.data.len();
                            let mut offset = 0usize;
                            while offset < length {
                                let (eir, used) = EirEntry::unpack(&event.data[offset..])?;
                                match eir.data_type {
                                    eir::DataType::Flags => {
                                        print!(" flags {:02x}", eir.data[0]);
                                    }
                                    eir::DataType::TxPowerLevel => {
                                        print!(" power {}", eir.data[0] as i8);
                                    }
                                    eir::DataType::ManufacturerData => {
                                        print!(" mfg");
                                    }
                                    eir::DataType::Appearance => {
                                        let appearance = LittleEndian::read_u16(&eir.data[0..1]);
                                        let appearance = bt_mgmt::Appearance::from(appearance);
                                        print!(" appearance {:?}", appearance);
                                    }
                                    eir::DataType::ClassOfDevice => {
                                        let cod = bt_mgmt::ClassOfDevice::unpack(&eir.data[..3])?;
                                        print!(" class {:?}", cod.device_class());
                                    }
                                    eir::DataType::ShortenedLocalName
                                    | eir::DataType::CompleteLocalName => {
                                        match std::str::from_utf8(eir.data) {
                                            Ok(name) => {
                                                print!(" name {:?}", name);
                                            }
                                            Err(_) => {
                                                print!(" invalid name");
                                            }
                                        }
                                    }
                                    _ => {
                                        print!(" {:?} ({})", eir.data_type, eir.data.len());
                                    }
                                }
                                offset += used;
                            }
                            println!();
                        }
                        Event::ClassOfDeviceChanged(event) => {
                            let device_class = event.device_class();
                            println!("Event {} device class changed {:?}", index, device_class);
                        }
                        _ => {
                            println!("Event {} {:?} ({}) {}", index, event, size, hex);
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
    }
}
