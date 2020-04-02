// Reference, https://www.bluetooth.com/specifications/assigned-numbers/baseband/

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassOfDevice(u32);

use crate::{error::HciError, error::HciErrorKind, pack::UnpackFixed, Error};

bitflags!(
    pub struct MajorServiceClass: u32 {
        const LIMITED_DISCOVERABLE = 0b0000_0000_0010_0000_0000_0000;
        const RESERVED1            = 0b0000_0000_0100_0000_0000_0000;
        const RESERVED2            = 0b0000_0000_1000_0000_0000_0000;
        const POSITIONING          = 0b0000_0001_0000_0000_0000_0000;
        const NETWORKING           = 0b0000_0010_0000_0000_0000_0000;
        const RENDERING            = 0b0000_0100_0000_0000_0000_0000;
        const CAPTURING            = 0b0000_1000_0000_0000_0000_0000;
        const OBJECT_TRANSFER      = 0b0001_0000_0000_0000_0000_0000;
        const AUDIO                = 0b0010_0000_0000_0000_0000_0000;
        const TELEFONY             = 0b0100_0000_0000_0000_0000_0000;
        const INFORMATION          = 0b1000_0000_0000_0000_0000_0000;
    }
);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MajorDeviceClass {
    Miscellaneous,
    Computer,
    Phone,
    Network,
    AudioVideo,
    Peripheral,
    Imaging,
    Wearable,
    Toy,
    Health,
    Uncategorized,
    Reserved,
}

impl From<u8> for MajorDeviceClass {
    fn from(value: u8) -> Self {
        match value {
            0b0_0000 => MajorDeviceClass::Miscellaneous,
            0b0_0001 => MajorDeviceClass::Computer,
            0b0_0010 => MajorDeviceClass::Phone,
            0b0_0011 => MajorDeviceClass::Network,
            0b0_0100 => MajorDeviceClass::AudioVideo,
            0b0_0101 => MajorDeviceClass::Peripheral,
            0b0_0110 => MajorDeviceClass::Imaging,
            0b0_0111 => MajorDeviceClass::Wearable,
            0b0_1000 => MajorDeviceClass::Toy,
            0b0_1001 => MajorDeviceClass::Health,
            0b1_1111 => MajorDeviceClass::Uncategorized,
            _ => MajorDeviceClass::Reserved,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DeviceClassComputer {
    Uncategorized,
    Workstation,
    Server,
    Laptop,
    Handheld,
    PalmSized,
    Wearable,
    Tablet,
    Reserved,
}

impl From<u8> for DeviceClassComputer {
    fn from(value: u8) -> Self {
        match value {
            0b00_0000 => DeviceClassComputer::Uncategorized,
            0b00_0001 => DeviceClassComputer::Workstation,
            0b00_0010 => DeviceClassComputer::Server,
            0b00_0011 => DeviceClassComputer::Laptop,
            0b00_0100 => DeviceClassComputer::Handheld,
            0b00_0101 => DeviceClassComputer::PalmSized,
            0b00_0110 => DeviceClassComputer::Wearable,
            0b00_0111 => DeviceClassComputer::Tablet,
            _ => DeviceClassComputer::Reserved,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DeviceClassPhone {
    Uncategorized,
    Cellular,
    Cordless,
    Smartphone,
    ModemOrVoiceGateway,
    Isdn,
    Reserved,
}

impl From<u8> for DeviceClassPhone {
    fn from(value: u8) -> Self {
        match value {
            0b00_0000 => DeviceClassPhone::Uncategorized,
            0b00_0001 => DeviceClassPhone::Cellular,
            0b00_0010 => DeviceClassPhone::Cordless,
            0b00_0011 => DeviceClassPhone::Smartphone,
            0b00_0100 => DeviceClassPhone::ModemOrVoiceGateway,
            0b00_0101 => DeviceClassPhone::Isdn,
            _ => DeviceClassPhone::Reserved,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DeviceClassNetwork {
    Uncategorized(u8),
    Reserved,
}

impl From<u8> for DeviceClassNetwork {
    fn from(value: u8) -> Self {
        let utilization = match value & 0b11_1000 {
            0b000 => 100,
            0b001 => 1,
            0b010 => 17,
            0b011 => 33,
            0b100 => 50,
            0b101 => 67,
            0b110 => 99,
            _ => 0,
        };
        match value & 0b111 {
            0b000 => DeviceClassNetwork::Uncategorized(utilization),
            _ => DeviceClassNetwork::Reserved,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DeviceClassAudioVideo {
    Uncategorized,
    Headset,
    HandsFree,
    Microphone,
    Loudspeaker,
    Headphones,
    PortableAudio,
    CarAudio,
    SetTopBox,
    HiFiAudioDevice,
    VCR,
    VideoCamera,
    Camcorder,
    VideoMonitor,
    VideoDisplayAndLoudspeaker,
    VideoConferencing,
    GamingToy,
    Reserved,
}

impl From<u8> for DeviceClassAudioVideo {
    fn from(value: u8) -> Self {
        match value {
            0b00_0000 => DeviceClassAudioVideo::Uncategorized,
            0b00_0001 => DeviceClassAudioVideo::Headset,
            0b00_0010 => DeviceClassAudioVideo::HandsFree,
            0b00_0100 => DeviceClassAudioVideo::Microphone,
            0b00_0101 => DeviceClassAudioVideo::Loudspeaker,
            0b00_0110 => DeviceClassAudioVideo::Headphones,
            0b00_0111 => DeviceClassAudioVideo::PortableAudio,
            0b00_1000 => DeviceClassAudioVideo::CarAudio,
            0b00_1001 => DeviceClassAudioVideo::SetTopBox,
            0b00_1010 => DeviceClassAudioVideo::HiFiAudioDevice,
            0b00_1011 => DeviceClassAudioVideo::VCR,
            0b00_1100 => DeviceClassAudioVideo::VideoCamera,
            0b00_1101 => DeviceClassAudioVideo::Camcorder,
            0b00_1110 => DeviceClassAudioVideo::VideoMonitor,
            0b00_1111 => DeviceClassAudioVideo::VideoDisplayAndLoudspeaker,
            0b01_0000 => DeviceClassAudioVideo::VideoConferencing,
            0b01_0010 => DeviceClassAudioVideo::GamingToy,
            _ => DeviceClassAudioVideo::Reserved,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DeviceClassKeyboardPointingDevice {
    NotKeyboardNotPointingDevice,
    Keyboard,
    PointingDevice,
    ComboDevice,
}

impl From<u8> for DeviceClassKeyboardPointingDevice {
    fn from(value: u8) -> Self {
        match value {
            0b01 => DeviceClassKeyboardPointingDevice::Keyboard,
            0b10 => DeviceClassKeyboardPointingDevice::PointingDevice,
            0b11 => DeviceClassKeyboardPointingDevice::ComboDevice,
            _ => DeviceClassKeyboardPointingDevice::NotKeyboardNotPointingDevice,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DeviceClassPeripheral {
    Uncategorized,
    Joystick,
    Gamepad,
    RemoteControl,
    SensingDevice,
    DigitizerTablet,
    CardReader,
    DigitalPen,
    HandheldScanner,
    HandheltGesture,
    Reserved,
}

impl From<u8> for DeviceClassPeripheral {
    fn from(value: u8) -> Self {
        match value {
            0b0000 => DeviceClassPeripheral::Uncategorized,
            0b0001 => DeviceClassPeripheral::Joystick,
            0b0010 => DeviceClassPeripheral::Gamepad,
            0b0011 => DeviceClassPeripheral::RemoteControl,
            0b0100 => DeviceClassPeripheral::SensingDevice,
            0b0101 => DeviceClassPeripheral::DigitizerTablet,
            0b0110 => DeviceClassPeripheral::CardReader,
            0b0111 => DeviceClassPeripheral::DigitalPen,
            0b1000 => DeviceClassPeripheral::HandheldScanner,
            0b1001 => DeviceClassPeripheral::HandheltGesture,
            _ => DeviceClassPeripheral::Reserved,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DeviceClassWearable {
    Wristwatch,
    Pager,
    Jacket,
    Helmet,
    Glasses,
    Reserved,
}

impl From<u8> for DeviceClassWearable {
    fn from(value: u8) -> Self {
        match value {
            0b0001 => DeviceClassWearable::Wristwatch,
            0b0010 => DeviceClassWearable::Pager,
            0b0011 => DeviceClassWearable::Jacket,
            0b0100 => DeviceClassWearable::Helmet,
            0b0101 => DeviceClassWearable::Glasses,
            _ => DeviceClassWearable::Reserved,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DeviceClassToy {
    Robot,
    Vehicle,
    Doll,
    Controller,
    Game,
    Reserved,
}

impl From<u8> for DeviceClassToy {
    fn from(value: u8) -> Self {
        match value {
            0b0001 => DeviceClassToy::Robot,
            0b0010 => DeviceClassToy::Vehicle,
            0b0011 => DeviceClassToy::Doll,
            0b0100 => DeviceClassToy::Controller,
            0b0101 => DeviceClassToy::Game,
            _ => DeviceClassToy::Reserved,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DeviceClassHealth {
    Undefined,
    BloodPressureMonitor,
    Thermometer,
    WeighingScale,
    GlucoseMeter,
    PulseOximeter,
    HeartRateMonitor,
    HealthDataDisplay,
    StepCounter,
    BodyCompositionAnalyzer,
    PeakFlowMonitor,
    MedicationMonitor,
    KneeProsthesis,
    AnkleProsthesis,
    GenericHealthManager,
    PersonalMobilityDevice,
    Reserved,
}

impl From<u8> for DeviceClassHealth {
    fn from(value: u8) -> Self {
        match value {
            0b00_0000 => DeviceClassHealth::Undefined,
            0b00_0001 => DeviceClassHealth::BloodPressureMonitor,
            0b00_0010 => DeviceClassHealth::Thermometer,
            0b00_0011 => DeviceClassHealth::WeighingScale,
            0b00_0100 => DeviceClassHealth::GlucoseMeter,
            0b00_0101 => DeviceClassHealth::PulseOximeter,
            0b00_0110 => DeviceClassHealth::HeartRateMonitor,
            0b00_0111 => DeviceClassHealth::HealthDataDisplay,
            0b00_1000 => DeviceClassHealth::StepCounter,
            0b00_1001 => DeviceClassHealth::BodyCompositionAnalyzer,
            0b00_1010 => DeviceClassHealth::PeakFlowMonitor,
            0b00_1011 => DeviceClassHealth::MedicationMonitor,
            0b00_1110 => DeviceClassHealth::KneeProsthesis,
            0b00_1111 => DeviceClassHealth::AnkleProsthesis,
            0b01_0000 => DeviceClassHealth::GenericHealthManager,
            0b01_0001 => DeviceClassHealth::PersonalMobilityDevice,
            _ => DeviceClassHealth::Reserved,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DeviceClass {
    Miscellaneous(u8),
    Computer(DeviceClassComputer),
    Phone(DeviceClassPhone),
    Network(DeviceClassNetwork),
    AudioVideo(DeviceClassAudioVideo),
    Peripheral((DeviceClassKeyboardPointingDevice, DeviceClassPeripheral)),
    Imaging,
    Wearable(DeviceClassWearable),
    Toy(DeviceClassToy),
    Health(DeviceClassHealth),
    Uncategorized(u8),
    Reserved(u8),
}

impl ClassOfDevice {
    pub fn format_code(&self) -> u8 {
        (self.0 & 0b0000_0000_0000_0000_0000_0011) as u8
    }

    pub fn major_device_class(&self) -> MajorDeviceClass {
        let major_device_class = ((self.0 & 0b0000_0000_0001_1111_0000_0000) >> 8) as u8;
        MajorDeviceClass::from(major_device_class)
    }

    pub fn device_class(&self) -> DeviceClass {
        let major = self.major_device_class();
        let minor = ((self.0 & 0b0000_0000_0000_0000_1111_1100) >> 2) as u8;
        match major {
            MajorDeviceClass::Miscellaneous => DeviceClass::Miscellaneous(minor),
            MajorDeviceClass::Computer => DeviceClass::Computer(DeviceClassComputer::from(minor)),
            MajorDeviceClass::Phone => DeviceClass::Phone(DeviceClassPhone::from(minor)),
            MajorDeviceClass::Network => DeviceClass::Network(DeviceClassNetwork::from(minor)),
            MajorDeviceClass::AudioVideo => {
                DeviceClass::AudioVideo(DeviceClassAudioVideo::from(minor))
            }
            MajorDeviceClass::Peripheral => DeviceClass::Peripheral((
                DeviceClassKeyboardPointingDevice::from(minor >> 4),
                DeviceClassPeripheral::from(minor & 0b1111),
            )),
            MajorDeviceClass::Imaging => DeviceClass::Imaging,
            MajorDeviceClass::Wearable => DeviceClass::Wearable(DeviceClassWearable::from(minor)),
            MajorDeviceClass::Toy => DeviceClass::Toy(DeviceClassToy::from(minor)),
            MajorDeviceClass::Health => DeviceClass::Health(DeviceClassHealth::from(minor)),
            MajorDeviceClass::Uncategorized => DeviceClass::Uncategorized(minor),
            MajorDeviceClass::Reserved => DeviceClass::Reserved(minor),
        }
    }

    pub fn major_service_class(&self) -> MajorServiceClass {
        MajorServiceClass::from_bits_truncate(self.0)
    }
}

impl<'a> UnpackFixed<'a, ClassOfDevice, Error> for ClassOfDevice {
    fn unpack(data: &'a [u8]) -> Result<ClassOfDevice, Error> {
        if data.len() != 3 {
            return Err(Error::from(HciError::new(HciErrorKind::NotEnoughData)));
        }
        let class_of_device = data[0] as u32 | (data[1] as u32) << 8 | (data[2] as u32) << 16;
        Ok(ClassOfDevice(class_of_device))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unpack() {
        let cod = ClassOfDevice::unpack(&[0x00, 0x00, 0x00]).unwrap();
        assert_eq!(cod.format_code(), 0);
        assert_eq!(cod.major_device_class(), MajorDeviceClass::Miscellaneous);
        assert_eq!(cod.major_service_class(), MajorServiceClass::empty());

        let cod = ClassOfDevice::unpack(&[0x04, 0x01, 0x14]).unwrap();
        assert_eq!(cod.format_code(), 0);
        assert_eq!(cod.major_device_class(), MajorDeviceClass::Computer);
        assert_eq!(
            cod.device_class(),
            DeviceClass::Computer(DeviceClassComputer::Workstation)
        );
        assert_eq!(
            cod.major_service_class(),
            MajorServiceClass::RENDERING | MajorServiceClass::OBJECT_TRANSFER
        );

        let cod = ClassOfDevice::unpack(&[0x04, 0x01, 0x1c]).unwrap();
        assert_eq!(cod.format_code(), 0);
        assert_eq!(cod.major_device_class(), MajorDeviceClass::Computer);
        assert_eq!(
            cod.device_class(),
            DeviceClass::Computer(DeviceClassComputer::Workstation)
        );
        assert_eq!(
            cod.major_service_class(),
            MajorServiceClass::RENDERING
                | MajorServiceClass::CAPTURING
                | MajorServiceClass::OBJECT_TRANSFER
        );
    }
}
