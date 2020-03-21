use crate::extended_enum_other;
/// Extended Inquiry Response (EIR)
///
///
// use byteorder::{ByteOrder, LittleEndian};
use crate::{error::HciError, error::HciErrorKind, Error};

extended_enum_other!(DataType, u8,
    Flags => 0x01,
    IncompleteListOf16bitServiceClassUUIDs => 0x02,
    CompleteListOf16bitServiceClassUUIDs => 0x03,
    IncompleteListOf32bitServiceClassUUIDs => 0x04,
    CompleteListOf32bitServiceClassUUIDs => 0x05,
    IncompleteListOf128bitServiceClassUUIDs => 0x06,
    CompleteListOf128bitServiceClassUUIDs => 0x07,
    ShortenedLocalName => 0x08,
    CompleteLocalName => 0x09,
    TxPowerLevel => 0x0a,
    ServiceData16BitUUIDs => 0x16,
    Appearance => 0x19,
    ManufacturerData => 0xff,
);

pub struct EirEntry<'a> {
    pub data_type: DataType,
    pub data: &'a [u8],
}

impl<'a> EirEntry<'a> {
    pub fn unpack(data: &'a [u8]) -> Result<(EirEntry<'a>, usize), Error> {
        if data.len() < 2 {
            return Err(Error::from(HciError::new(HciErrorKind::NotEnoughData)));
        }
        let length = usize::from(data[0]);
        if data.len() < length + 1 {
            return Err(Error::from(HciError::new(HciErrorKind::NotEnoughData)));
        }
        let data_type = DataType::from(data[1]);
        let data = if length > 1 { &data[2..=length] } else { &[] };
        Ok((EirEntry { data_type, data }, length + 1))
    }
}
