mod command;
mod device_found;
mod discovering;

use byteorder::{ByteOrder, LittleEndian};

use crate::{
    error::{HciError, HciErrorKind},
    extended_enum_other,
    pack::{Unpack, UnpackFixed},
    ClassOfDevice, Error,
};

pub use command::{CommandComplete, CommandStatus};
pub use device_found::DeviceFound;
pub use discovering::Discovering;

use bitflags;

bitflags!(
    pub struct Settings: u32 {
        const POWERED = 0x0000_0001;
        const CONNECTABLE = 0x0000_0002;
        const FAST_CONNECTABLE = 0x0000_0004;
        const DISCOVERABLE = 0x0000_0008;
        const BONDABLE = 0x0000_0010;
        const LINK_SECURITY = 0x0000_0020;
        const SECURE_SIMPLE_PAIRING = 0x0000_0040;
        const BASIC_RATE_ENHANCED_DATA_RATE = 0x0000_0080;
        const HIGH_SPEED = 0x0000_0100;
        const LOW_ENERGY = 0x0000_0200;
        const ADVERTISING = 0x0000_0400;
        const SECURE_CONN = 0x0000_0800;
        const DEBUG_KEYS = 0x0000_1000;
        const PRIVACY = 0x0000_2000;
        const CONFIGURATION = 0x0000_4000;
        const STATIC_ADDRESS = 0x0000_8000;
        const PHY_CONFIGURATION = 0x0001_0000;
    }
);

extended_enum_other!(EventId, u16,
    CommandComplete => 0x0001,
    CommandStatus => 0x0002,
    ControllerError => 0x0003,
    IndexAdded => 0x0004,
    IndexRemoved => 0x0005,
    NewSettings => 0x0006,
    ClassOfDeviceChanged => 0x0007,
    LocalNameChanged => 0x0008,
    NewLinkKey => 0x0009,
    NewLongTermKey => 0x000a,
    DeviceConnected => 0x000b,
    DeviceDisconnected => 0x000c,
    ConnectFailed => 0x000d,
    PinCodeRequest => 0x000e,
    UserConfirmRequest => 0x000f,
    UserPasskeyRequest => 0x0010,
    AuthenticatonFailed => 0x0011,
    DeviceFound => 0x0012,
    Discovering => 0x0013,
    DeviceBlocked => 0x0014,
    DeviceUnblocked => 0x0015,
    DeviceUnpaired => 0x0016,
    PasskeyNotify => 0x0017,
    NewIdentityResolvingKey => 0x0018,
    NewConnectionSignatureResolvingKey => 0x0019,
    DeviceAdded => 0x001a,
    DeviceRemoved => 0x001b,
    NewConnectionParameters => 0x001c,
    UnconfirmedIndexAdded => 0x001d,
    UnconfirmedIndexRemoved => 0x001e,
    NewConfigurationOptions => 0x001f,
    ExtendedIndexAdded => 0x0020,
    ExtendedIndexRemoved => 0x0021,
    LocalOutOfBandDataUpdated => 0x0022,
    AdvertisingAdded => 0x0023,
    AdvertisingRemoved => 0x0024,
    ExtendedInformationChanged => 0x0025,
    PhyConfigurationChanged => 0x0026,
);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Event<'a> {
    CommandComplete(CommandComplete<'a>),
    CommandStatus(CommandStatus),
    ControllerError(u8),
    IndexAdded,
    IndexRemoved,
    ClassOfDeviceChanged(ClassOfDevice),
    NewSettings(Settings),
    DeviceFound(DeviceFound<'a>),
    Discovering(Discovering),
    Other((EventId, &'a [u8])),
}

impl<'a> Event<'a> {
    pub fn unpack(event_id: EventId, data: &'a [u8]) -> Result<(Event<'a>, usize), Error> {
        match event_id {
            EventId::CommandComplete => {
                let (event, used) = CommandComplete::unpack(data)?;
                Ok((Event::CommandComplete(event), used))
            }
            EventId::CommandStatus => {
                let (event, used) = CommandStatus::unpack(data)?;
                Ok((Event::CommandStatus(event), used))
            }
            EventId::ControllerError => {
                if data.is_empty() {
                    return Err(Error::from(HciError::new(HciErrorKind::NotEnoughData)));
                }
                Ok((Event::ControllerError(data[0]), 1))
            }
            EventId::IndexAdded => Ok((Event::IndexAdded, 0)),
            EventId::IndexRemoved => Ok((Event::IndexRemoved, 0)),
            EventId::ClassOfDeviceChanged => {
                if data.len() < 3 {
                    return Err(Error::from(HciError::new(HciErrorKind::NotEnoughData)));
                }
                let cod = ClassOfDevice::unpack(&data[..3])?;
                Ok((Event::ClassOfDeviceChanged(cod), 3))
            }
            EventId::NewSettings => {
                if data.len() < 4 {
                    return Err(Error::from(HciError::new(HciErrorKind::NotEnoughData)));
                }
                let settings = LittleEndian::read_u32(&data[0..4]);
                let settings = match Settings::from_bits(settings) {
                    Some(v) => v,
                    None => return Err(Error::from(HciError::new(HciErrorKind::InvalidValue))),
                };
                Ok((Event::NewSettings(settings), 4))
            }
            EventId::DeviceFound => {
                let (event, used) = DeviceFound::unpack(data)?;
                Ok((Event::DeviceFound(event), used))
            }
            EventId::Discovering => {
                let event = Discovering::unpack(&data[..2])?;
                Ok((Event::Discovering(event), 2))
            }
            _ => Ok((Event::Other((event_id, &data)), data.len())),
        }
    }
}
