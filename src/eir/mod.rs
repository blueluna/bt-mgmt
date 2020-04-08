use crate::extended_enum_other;
/// Extended Inquiry Response (EIR)
///
///
// use byteorder::{ByteOrder, LittleEndian};

// Reference, https://www.bluetooth.com/specifications/assigned-numbers/generic-access-profile/
use crate::{error::HciError, error::HciErrorKind, Error};

extended_enum_other!(DataType, u8,
    Flags => 0x01,
    IncompleteServiceClassUUIDs16 => 0x02,
    CompleteServiceClassUUIDs16 => 0x03,
    IncompleteServiceClassUUIDs32 => 0x04,
    CompleteServiceClassUUIDs32 => 0x05,
    IncompleteServiceClassUUIDs128 => 0x06,
    CompleteServiceClassUUIDs128 => 0x07,
    ShortenedLocalName => 0x08,
    CompleteLocalName => 0x09,
    TxPowerLevel => 0x0a,
    ClassOfDevice => 0x0d,
    SimplePairingHash192 => 0x0e,
    SimplePairingRandomizer192 => 0x0f,
    SecurityManagerTemporaryKey => 0x10,
    SecurityManagerOutOfBandFlags => 0x11,
    SlaveConnectionIntervalRange => 0x12,
    ServiceSolicitationUUIDs16 => 0x14,
    ServiceSolicitationUUIDs128 => 0x15,
    ServiceDataUUIDs16 => 0x16,
    PublicTargetAddress => 0x17,
    RandomTargetAddress => 0x18,
    Appearance => 0x19,
    AdvertisingInterval => 0x1a,
    LowEnergyBluetoothDeviceAddress => 0x1b,
    LowEnergyRole => 0x1c,
    SimplePairingHash256 => 0x1d,
    SimplePairingRandomizer256 => 0x1e,
    ServiceSolicitationUUIDs32 => 0x1f,
    ServiceDataUUIDs32 => 0x20,
    ServiceDataUUIDs128 => 0x21,
    LowEnergySecureConnectionsConfirmation => 0x22,
    LowEnergySecureConnectionsRandom => 0x23,
    Uri => 0x24,
    IndoorPositioning => 0x25,
    TransportDiscoveryData => 0x26,
    LowEnergySupportedFeatures => 0x27,
    ChannelMapUpdateIndication => 0x28,
    ProvisioningBearerAdvertisement => 0x29,
    MeshMessage => 0x2a,
    MeshBeacon => 0x2b,
    BroadcastIsochronousGroupInformation => 0x2c,
    BroadcastCode => 0x2d,
    InformationData3D => 0x3d,
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
