/// Extended Inquiry Response (EIR)
/// 
/// 


use crate::{Error, error::HciError, error::HciErrorKind, pack::Unpack};
use crate::extended_enum_other;

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
);

struct EirEntry<'a> {
    data_type: DataType,
    data: &'a[u8]
}

impl<'a> EirEntry<'a> {
    fn unpack(data: &'a[u8]) -> Result<(EirEntry<'a>, usize), Error> {
        if data.len() < 2 {
            return Err(Error::from(HciError::new(HciErrorKind::NotEnoughData)));
        }
        let length = usize::from(data[0]);
        if data.len() < length + 2 {
            return Err(Error::from(HciError::new(HciErrorKind::NotEnoughData)));
        }
        let data_type = DataType::from(data[1]);
        Ok((EirEntry{ data_type, data: &data[..length]}, length + 2))
    }
}