use std::fmt;
use std::os::unix::io::AsRawFd;
use std::time::Duration;

use mio::{unix::SourceFd, Events, Interest, Poll, Token};

use timerfd::{ClockId, SetTimeFlags, TimerFd, TimerState};

use byteorder::{ByteOrder, LittleEndian};

use bt_mgmt::{
    self,
    eir::{self, EirEntry},
    events::{self, EventId},
    pack::{Unpack, UnpackFixed},
    ClassOfDevice, Error, HardwareAddress, OperationId, Socket, Status,
};

const MGMT_EVENTS: Token = Token(0);
const TIMER_EVENTS: Token = Token(1);

struct Scanner {
    poll: mio::Poll,
    mgmt: Socket,
    timer: TimerFd,
    mgmt_index: u16,
    scanning: bool,
}

impl Scanner {
    pub fn new() -> Result<Self, Error> {
        let poll = Poll::new()?;
        let mgmt = Socket::new()?;
        let mut timer = TimerFd::new_custom(ClockId::Monotonic, true, true)?;

        timer.set_state(
            TimerState::Periodic {
                current: Duration::from_secs(1),
                interval: Duration::from_secs(10),
            },
            SetTimeFlags::Default,
        );

        poll.registry().register(
            &mut SourceFd(&mgmt.as_raw_fd()),
            MGMT_EVENTS,
            Interest::READABLE,
        )?;
        poll.registry().register(
            &mut SourceFd(&timer.as_raw_fd()),
            TIMER_EVENTS,
            Interest::READABLE,
        )?;
        Ok(Scanner {
            poll,
            mgmt,
            timer,
            mgmt_index: 0xffff,
            scanning: false,
        })
    }

    pub fn run(&mut self) -> Result<(), Error> {
        if self.mgmt_index == 0xffff {
            self.send_command(bt_mgmt::OperationId::ReadIndexList, &[])?;
        } else {
            self.send_command(bt_mgmt::OperationId::ReadInformation, &[])?;
        }
        let mut events = Events::with_capacity(128);
        loop {
            self.poll.poll(&mut events, None)?;
            for event in events.iter() {
                match event.token() {
                    MGMT_EVENTS => {
                        self.mgmt_read()?;
                    }
                    TIMER_EVENTS => {
                        let _ = self.timer.read();
                        self.time()?;
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    fn mgmt_read(&mut self) -> Result<(), Error> {
        let mut buffer = [0u8; 1024];
        let (size, event, index) = self.mgmt.receive_event(&mut buffer)?;
        match event {
            EventId::CommandComplete => {
                let (complete, _) = events::CommandComplete::unpack(&buffer[..size])?;
                if complete.status == Status::Success {
                    self.event_command_complete(index, complete.operation, complete.data)?;
                } else {
                    println!(
                        "Command failed {} {:?} {:?}",
                        index, complete.operation, complete.status
                    );
                }
            }
            EventId::ClassOfDeviceChanged => {
                let device_class = ClassOfDevice::unpack(&buffer[..size])?;
                println!("Event {} device class changed {:?}", index, device_class);
            }
            EventId::DeviceFound => {
                self.event_device_found(index, &buffer[..size])?;
            }
            EventId::Discovering => {
                let discovering = events::Discovering::unpack(&buffer[..size])?;
                self.scanning = discovering.discovering;
                println!(
                    "Event {} Discovering {} {:?}",
                    index,
                    if discovering.discovering { "yes" } else { "no" },
                    discovering.discovery_type
                );
            }
            _ => (),
        }
        Ok(())
    }

    fn time(&mut self) -> Result<(), Error> {
        if self.mgmt_index < 0xffff {
            let discovering_type = (events::DiscoveringType::BR_EDR
                | events::DiscoveringType::LE_PUBLIC
                | events::DiscoveringType::LE_RANDOM)
                .bits();
            if self.scanning {
                println!("Stop Discovering {:02x}", discovering_type);
                self.send_command(OperationId::StopDiscovery, &[discovering_type])?;
                self.scanning = false;
            } else {
                println!("Start Discovering {:02x}", discovering_type);
                self.send_command(OperationId::StartDiscovery, &[discovering_type])?;
                self.scanning = true;
            }
        }
        Ok(())
    }

    fn send_command(&mut self, operation: bt_mgmt::OperationId, data: &[u8]) -> Result<(), Error> {
        self.mgmt.send_command(operation, self.mgmt_index, data)?;
        Ok(())
    }

    fn event_command_complete(
        &mut self,
        index: u16,
        operation: OperationId,
        data: &[u8],
    ) -> Result<(), Error> {
        match operation {
            OperationId::ReadIndexList => {
                let (count, offset) = if data.len() >= 2 {
                    (LittleEndian::read_u16(&data[..2]), 2)
                } else {
                    (0, 0)
                };
                let octets = (count * 2) as usize;
                if (data.len() - offset) >= octets {
                    let indicies: Vec<u16> = data[offset..offset + octets]
                        .chunks_exact(2)
                        .map(|c| LittleEndian::read_u16(c))
                        .collect();
                    println!("Index List, {:?}", indicies);
                    if self.mgmt_index > 0x7fff && indicies.len() == 1 {
                        self.mgmt_index = indicies[0];
                        self.send_command(bt_mgmt::OperationId::ReadInformation, &[])?;
                    }
                }
            }
            OperationId::ReadInformation => {
                if data.len() >= 280 {
                    let address = HardwareAddress::from(&data[0..6]);
                    let version = data[6];
                    let manufacturer = LittleEndian::read_u16(&data[7..9]);
                    let supported_settings = LittleEndian::read_u32(&data[9..13]);
                    let current_settings = LittleEndian::read_u32(&data[13..17]);
                    let class_of_device = bt_mgmt::ClassOfDevice::unpack(&data[17..20])?;
                    println!(
                        "Information, {} {} {:04x} {:08x} {:08x} {:?}",
                        address,
                        version,
                        manufacturer,
                        supported_settings,
                        current_settings,
                        class_of_device
                    );
                }
            }
            _ => {
                println!(
                    "Command succeded {} {:?} ({})",
                    index,
                    operation,
                    data.len()
                );
            }
        }
        Ok(())
    }

    fn event_device_found(&mut self, index: u16, data: &[u8]) -> Result<(), Error> {
        let (device_found, _) = events::DeviceFound::unpack(data)?;
        print!(
            "Event {} Device found {} {:4} {:08x}",
            index, device_found.address_info, device_found.rssi, device_found.flags
        );
        let length = device_found.data.len();
        let mut offset = 0usize;
        while offset < length {
            let (eir, used) = EirEntry::unpack(&device_found.data[offset..])?;
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
                    if eir.data.len() == 2 {
                        let appearance_identifier = LittleEndian::read_u16(&eir.data[0..2]);
                        let appearance = bt_mgmt::Appearance::from(appearance_identifier);
                        match appearance {
                            bt_mgmt::Appearance::Reserved => {
                                fmt::format(format_args!("Hello, {}!", "world"));
                                print!(" appearance reserved {:04x}", appearance_identifier);
                            }
                            _ => {
                                print!(" appearance {:?}", appearance);
                            }
                        }
                    } else {
                        print!(" appearance {}", eir.data.len());
                    }
                }
                eir::DataType::ClassOfDevice => {
                    let cod = bt_mgmt::ClassOfDevice::unpack(&eir.data[..3])?;
                    print!(" class {:?}", cod.device_class());
                }
                eir::DataType::ShortenedLocalName | eir::DataType::CompleteLocalName => {
                    match std::str::from_utf8(eir.data) {
                        Ok(name) => {
                            print!(" name {:?}", name);
                        }
                        Err(_) => {
                            print!(" invalid name");
                        }
                    }
                }
                eir::DataType::IncompleteServiceClassUUIDs16
                | eir::DataType::CompleteServiceClassUUIDs16 => {
                    print!(" UUID");
                    for chunk in eir.data.chunks_exact(2) {
                        let uuid = LittleEndian::read_u16(chunk);
                        print!(" {:04x}", uuid);
                    }
                }
                eir::DataType::IncompleteServiceClassUUIDs32
                | eir::DataType::CompleteServiceClassUUIDs32 => {
                    print!(" UUID");
                    for chunk in eir.data.chunks_exact(4) {
                        let uuid = LittleEndian::read_u32(chunk);
                        print!(" {:08x}", uuid);
                    }
                }
                _ => {
                    print!(" {:?} ({})", eir.data_type, eir.data.len());
                }
            }
            offset += used;
        }
        println!();
        Ok(())
    }
}

fn main() -> Result<(), Error> {
    let mut scanner = Scanner::new()?;
    scanner.run()
}
