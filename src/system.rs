use libc;
use std::io;
use std::mem::size_of;
use std::os::unix::io::RawFd;

macro_rules! ccall {
    ( $x:expr ) => {{
        let value = unsafe { $x };
        if value < 0 {
            return Err(io::Error::last_os_error());
        }
        value
    }};
}

#[repr(C)]
pub(crate) struct Address {
    pub family: u16,
    pub device: u16,
    pub channel: u16,
}

const BTPROTO_HCI: i32 = 1;

pub(crate) fn hci_socket() -> io::Result<RawFd> {
    Ok(ccall!(libc::socket(
        libc::AF_BLUETOOTH,
        libc::SOCK_RAW | libc::SOCK_NONBLOCK | libc::SOCK_CLOEXEC,
        BTPROTO_HCI
    )))
}

const HCI_CHANNEL_RAW: u16 = 0;
const HCI_CHANNEL_USER: u16 = 1;
const HCI_CHANNEL_MONITOR: u16 = 2;
const HCI_CHANNEL_CONTROL: u16 = 3;
const HCI_CHANNEL_LOGGING: u16 = 4;

pub(crate) fn bind(socket: RawFd, address: &Address) -> io::Result<()> {
    let addr_ptr: *const Address = address;
    let _ = ccall!(libc::bind(
        socket,
        addr_ptr as *const libc::sockaddr,
        size_of::<Address>() as u32
    ));
    Ok(())
}

const MGMT_INDEX_NONE: u16 = 0xffff;

pub(crate) fn bind_device(socket: RawFd, device_identifier: u16) -> io::Result<()> {
    let address = Address {
        family: libc::AF_BLUETOOTH as u16,
        device: device_identifier,
        channel: if device_identifier == MGMT_INDEX_NONE {
            HCI_CHANNEL_CONTROL
        } else {
            0
        },
    };
    bind(socket, &address)
}

pub(crate) fn bind_mgmn(socket: RawFd) -> io::Result<()> {
    bind_device(socket, MGMT_INDEX_NONE)
}

pub(crate) fn socket_write(socket: RawFd, buffer: &[u8]) -> io::Result<usize> {
    let buffer_ptr: *const [u8] = buffer;
    let buffer_ptr: *const core::ffi::c_void = buffer_ptr as *const core::ffi::c_void;
    let bytes = ccall!(libc::write(socket, buffer_ptr, buffer.len()));
    Ok(bytes as usize)
}

pub(crate) fn socket_read(socket: RawFd, buffer: &mut [u8]) -> io::Result<usize> {
    let buffer_ptr: *mut [u8] = buffer;
    let buffer_ptr: *mut core::ffi::c_void = buffer_ptr as *mut core::ffi::c_void;
    let bytes = ccall!(libc::read(socket, buffer_ptr, buffer.len()));
    Ok(bytes as usize)
}
