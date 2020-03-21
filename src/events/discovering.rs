use crate::{error::HciError, error::HciErrorKind, pack::UnpackFixed, Error};
use bitflags;

bitflags!(
    struct Type: u8 {
        const BE_EDR = 0b0001;
        const LE_PUBLIC = 0b0010;
        const LE_RANDOM = 0b0100;
        const LE = Self::LE_PUBLIC.bits | Self::LE_RANDOM.bits;
        const INTERLEAVED = Self::BE_EDR.bits | Self::LE_PUBLIC.bits | Self::LE_RANDOM.bits;
    }
);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Discovering {
    discovery_type: Type,
    discovering: bool,
}

impl UnpackFixed<Discovering, Error> for Discovering {
    fn unpack(data: &[u8]) -> Result<Discovering, Error> {
        assert!(data.len() == 2);
        let discovery_type = match Type::from_bits(data[0]) {
            Some(v) => v,
            None => return Err(Error::from(HciError::new(HciErrorKind::InvalidValue))),
        };
        let discovering = match data[1] {
            0x00 => false,
            0x01 => true,
            _ => return Err(Error::from(HciError::new(HciErrorKind::InvalidValue))),
        };
        Ok(Discovering {
            discovery_type,
            discovering,
        })
    }
}
