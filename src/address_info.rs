use std::convert::TryFrom;

use crate::extended_enum;
use crate::hardware_address::HardwareAddress;

use crate::{error::HciError, error::HciErrorKind, pack::UnpackFixed, Error};

// Basic Rate / Enhanced Data Rate (BR/EDR)
//
// Low Energy (LE)

extended_enum!(
    AddressType, u8,
    BrEdr => 0x00,
    LePublic => 0x01,
    LeRandom => 0x02,
);

#[derive(Clone, Debug, PartialEq)]
pub struct AddressInfo {
    pub address: HardwareAddress,
    pub address_type: AddressType,
}

impl UnpackFixed<AddressInfo, Error> for AddressInfo {
    fn unpack(data: &[u8]) -> Result<AddressInfo, Error> {
        if data.len() != 7 {
            return Err(Error::from(HciError::new(HciErrorKind::NotEnoughData)));
        }
        let address = HardwareAddress::from(&data[0..6]);
        let address_type = AddressType::try_from(data[6])?;
        Ok(AddressInfo {
            address,
            address_type,
        })
    }
}
