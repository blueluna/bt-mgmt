use std::os::unix::io::{AsRawFd, RawFd};

use crate::error::{Error, HciError, HciErrorKind, Result};
use crate::events::EventId;
use crate::system;

use byteorder::{ByteOrder, LittleEndian};

const MGMT_BUFFER_SIZE: usize = 512;
const MGMT_HEADER_SIZE: usize = 6;

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

    pub fn send_command<T: Into<u16>>(
        &mut self,
        opcode: T,
        index: u16,
        data: &[u8],
    ) -> Result<usize> {
        assert!(data.len() < u16::max_value() as usize);
        let end = MGMT_HEADER_SIZE + data.len();
        assert!(end < MGMT_BUFFER_SIZE);
        let mut buffer = [0u8; MGMT_BUFFER_SIZE];
        buffer[MGMT_HEADER_SIZE..end].copy_from_slice(data);
        LittleEndian::write_u16(&mut buffer[0..2], opcode.into());
        LittleEndian::write_u16(&mut buffer[2..4], index);
        let size = data.len() as u16;
        LittleEndian::write_u16(&mut buffer[4..MGMT_BUFFER_SIZE], size);
        match system::socket_write(self.socket, &buffer[..end]) {
            Ok(size) => Ok(size),
            Err(err) => Err(err.into()),
        }
    }

    pub fn receive_event(&mut self, data: &mut [u8]) -> Result<(usize, EventId, u16)> {
        let mut buffer = [0u8; MGMT_BUFFER_SIZE];
        let read = system::socket_read(self.socket, &mut buffer)?;
        if read < MGMT_HEADER_SIZE {
            return Err(Error::Hci(HciError::new(HciErrorKind::NotEnoughData)));
        }
        let event = EventId::from(LittleEndian::read_u16(&buffer[0..2]));
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
