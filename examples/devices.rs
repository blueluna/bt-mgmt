use std::os::unix::io::AsRawFd;

use mio::{unix::SourceFd, Events, Interest, Poll, Token};

use bt_mgmt::{
    eir::EirEntry,
    events::{DeviceFound, Discovering},
    pack::UnpackFixed,
    Error, Event, Socket,
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
                    let hex: String = buffer[..size]
                        .iter()
                        .map(|i| format!("{:02x}", i))
                        .collect();
                    match event {
                        Event::Discovering => {
                            let event = Discovering::unpack(&buffer[..size])?;
                            println!("Event {} {:?}", index, event);
                        }
                        Event::DeviceFound => {
                            let (event, _) = DeviceFound::unpack(&buffer[..size])?;
                            print!(
                                "Event {} Device found {} {:08x}",
                                index, event.rssi, event.flags
                            );
                            let length = event.data.len();
                            let mut offset = 0usize;
                            while offset < length {
                                let (eir, used) = EirEntry::unpack(&event.data[offset..])?;
                                print!(" {:?} ({})", eir.data_type, eir.data.len());
                                offset += used;
                            }
                            println!();
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
