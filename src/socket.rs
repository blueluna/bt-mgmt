use std::os::unix::io::{AsRawFd, RawFd};

use crate::error::{Error, HciError, HciErrorKind, Result};
use crate::extended_enum_other;
use crate::system;

use byteorder::{ByteOrder, LittleEndian};

const MGMT_BUFFER_SIZE: usize = 512;
const MGMT_HEADER_SIZE: usize = 6;

extended_enum_other!(Event, u16,
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

extended_enum_other!(Status, u8,
    Success => 0x00,
    UnknownCommand => 0x01,
    NotConnected => 0x02,
    Failed => 0x03,
    ConnectFailed => 0x04,
    AuthenticationFailed => 0x05,
    NotPaired => 0x06,
    NoResources => 0x07,
    Timeout => 0x08,
    AlreadyConnected => 0x09,
    Busy => 0x0a,
    Rejected => 0x0b,
    NotSupported => 0x0c,
    InvalidParameters => 0x0d,
    Disconnected => 0x0e,
    NotPowered => 0x0f,
    Cancelled => 0x10,
    InvalidIndex => 0x11,
    Rfkilled => 0x12,
    AlreadyPaired => 0x13,
    PermissionDenied => 0x14,
);

/// HCI Socket can be used to communicate with the Linux kernel using the
/// HCI protocol.
pub struct Socket {
    socket: RawFd,
}

impl Socket {
    /// Create a new Socket
    pub fn new() -> Result<Socket> {
        let socket = system::hci_socket()?;
        system::bind_mgmn(socket)?;
        Ok(Socket { socket })
    }

    pub fn send_command(&mut self, opcode: u16, index: u16, data: &[u8]) -> Result<usize> {
        assert!(data.len() < u16::max_value() as usize);
        let end = MGMT_HEADER_SIZE + data.len();
        assert!(end < MGMT_BUFFER_SIZE);
        let mut buffer = [0u8; MGMT_BUFFER_SIZE];
        buffer[MGMT_HEADER_SIZE..end].copy_from_slice(data);
        LittleEndian::write_u16(&mut buffer[0..2], opcode);
        LittleEndian::write_u16(&mut buffer[2..4], index);
        let size = data.len() as u16;
        LittleEndian::write_u16(&mut buffer[4..MGMT_BUFFER_SIZE], size);
        match system::socket_write(self.socket, &buffer[..end]) {
            Ok(size) => Ok(size),
            Err(err) => Err(err.into()),
        }
    }

    pub fn receive_event(&mut self, data: &mut [u8]) -> Result<(usize, Event, u16)> {
        let mut buffer = [0u8; MGMT_BUFFER_SIZE];
        let read = system::socket_read(self.socket, &mut buffer)?;
        if read < MGMT_HEADER_SIZE {
            return Err(Error::Hci(HciError::new(HciErrorKind::NotEnoughData)));
        }
        let event = Event::from(LittleEndian::read_u16(&buffer[0..2]));
        let index = LittleEndian::read_u16(&buffer[2..4]);
        let size = LittleEndian::read_u16(&buffer[4..6]) as usize;
        data[..size].copy_from_slice(&buffer[MGMT_HEADER_SIZE..MGMT_HEADER_SIZE + size]);
        Ok((size, event, index))
    }
}

impl AsRawFd for Socket {
    fn as_raw_fd(&self) -> RawFd {
        self.socket
    }
}

#[cfg(test)]
mod tests {
    use super::Socket;

    #[test]
    fn test_socket() {
        let _socket = Socket::new().unwrap();
    }
}
