use byteorder::{ByteOrder, LittleEndian};

use crate::{error::HciError, error::HciErrorKind, pack::UnpackFixed, AddressInfo, Error};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DeviceFound<'a> {
    pub address_info: AddressInfo,
    pub rssi: i8,
    pub flags: u32,
    pub data: &'a [u8],
}

impl<'a> DeviceFound<'a> {
    pub fn unpack(data: &'a [u8]) -> Result<(DeviceFound<'a>, usize), Error> {
        if data.len() < 14 {
            return Err(Error::from(HciError::new(HciErrorKind::NotEnoughData)));
        }
        let address_info = AddressInfo::unpack(&data[0..7])?;
        let mut offset = 7;
        let rssi = data[offset] as i8;
        offset += 1;
        let flags = LittleEndian::read_u32(&data[offset..offset + 4]);
        offset += 4;
        let length = LittleEndian::read_u16(&data[offset..offset + 2]);
        offset += 2;
        let end = offset + (length as usize);
        if end > data.len() {
            return Err(Error::from(HciError::new(HciErrorKind::NotEnoughData)));
        }
        Ok((
            DeviceFound {
                address_info,
                rssi,
                flags,
                data: &data[offset..end],
            },
            end,
        ))
    }
}
