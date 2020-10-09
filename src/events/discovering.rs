use crate::{error::HciError, error::HciErrorKind, pack::UnpackFixed, Error};
use bitflags;

bitflags!(
    pub struct DiscoveringType: u8 {
        const BR_EDR = 0b0001;
        const LE_PUBLIC = 0b0010;
        const LE_RANDOM = 0b0100;
    }
);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Discovering {
    pub discovery_type: DiscoveringType,
    pub discovering: bool,
}

impl<'a> UnpackFixed<'a, Discovering, Error> for Discovering {
    fn unpack(data: &'a [u8]) -> Result<Discovering, Error> {
        assert!(data.len() == 2);
        let discovery_type = match DiscoveringType::from_bits(data[0]) {
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
