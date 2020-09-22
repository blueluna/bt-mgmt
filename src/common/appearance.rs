use core::convert::From;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Watch {
    Generic,
    Sport,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Thermometer {
    Generic,
    Ear,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HeartRateSensor {
    Generic,
    Belt,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BloodPressure {
    Generic,
    Arm,
    Wrist,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HumanInterfaceDevice {
    Generic,
    Keyboard,
    Mouse,
    Joystick,
    Gamepad,
    DigitizerTablet,
    CardReader,
    DigitalPen,
    BarcodeScanner,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RunningWalkingSensor {
    Generic,
    InShoe,
    OnShoe,
    OnHip,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Cycling {
    Generic,
    Computer,
    SpeedSensor,
    CadenceSensor,
    PowerSensor,
    SpeedAndCadenceSensor,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GenericControlDevice {
    Generic,
    Switch,
    MultiSwitch,
    Button,
    Slider,
    Rotary,
    TouchPanel,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GenericNetworkDevice {
    Generic,
    AccessPoint,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Sensor {
    Generic,
    Motion,
    AirQuality,
    Temperature,
    Humidity,
    Leak,
    Smoke,
    Occupancy,
    Contact,
    CarbonMonoxide,
    CarbonDioxide,
    AmbientLight,
    Energy,
    ColorLight,
    Rain,
    Fire,
    Wind,
    Proximity,
    MultiSensor,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LightFixture {
    Generic,
    WallLight,
    CeilingLight,
    FloorLight,
    CabinetLight,
    DeskLight,
    TrofferLight,
    PendandtLight,
    InGroundLight,
    FloodLight,
    UnderwaterLight,
    BollardLight,
    PathwatLight,
    GardenLight,
    PoleTopLight,
    Spotlight,
    LinearLight,
    StreetLight,
    ShelvesLight,
    HighBayLowBayLight,
    EmergencyExitLight,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Fan {
    Generic,
    Ceiling,
    Axial,
    Exhaust,
    Pedestal,
    Desk,
    Wall,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Appearance {
    Unknown,
    Phone,
    Computer,
    Watch(Watch),
    Clock,
    Display,
    RemoteControl,
    EyeGlasses,
    Tag,
    Keyring,
    MediaPlayer,
    BarcodeScanner,
    Thermometer(Thermometer),
    HeartRateSensor(HeartRateSensor),
    BloodPressure(BloodPressure),
    HumanInterfaceDevice(HumanInterfaceDevice),
    GlucoseMeter,
    RunningWalkingSensor(RunningWalkingSensor),
    Cycling(Cycling),
    GenericControlDevice(GenericControlDevice),
    GenericNetworkDevice(GenericNetworkDevice),
    Sensor(Sensor),
    LightFixture(LightFixture),
    Fan(Fan),
}

impl From<u16> for Appearance {
    fn from(value: u16) -> Appearance {
        let category = (0b1111_1111_1100_0000 & value) >> 6;
        let sub_category = 0b0000_0000_0011_1111 & value;
        match (category, sub_category) {
            (1, _) => Appearance::Phone,
            (2, _) => Appearance::Computer,
            (3, 1) => Appearance::Watch(Watch::Sport),
            (3, _) => Appearance::Watch(Watch::Generic),
            (4, _) => Appearance::Clock,
            (5, _) => Appearance::Display,
            (6, _) => Appearance::RemoteControl,
            (7, _) => Appearance::EyeGlasses,
            (8, _) => Appearance::Tag,
            (9, _) => Appearance::Keyring,
            (10, _) => Appearance::MediaPlayer,
            (11, _) => Appearance::BarcodeScanner,
            (12, 1) => Appearance::Thermometer(Thermometer::Ear),
            (12, _) => Appearance::Thermometer(Thermometer::Generic),
            (13, 1) => Appearance::HeartRateSensor(HeartRateSensor::Belt),
            (13, _) => Appearance::HeartRateSensor(HeartRateSensor::Generic),
            (14, 1) => Appearance::BloodPressure(BloodPressure::Arm),
            (14, 2) => Appearance::BloodPressure(BloodPressure::Wrist),
            (14, _) => Appearance::BloodPressure(BloodPressure::Generic),
            (15, 1) => Appearance::HumanInterfaceDevice(HumanInterfaceDevice::Keyboard),
            (15, 2) => Appearance::HumanInterfaceDevice(HumanInterfaceDevice::Mouse),
            (15, 3) => Appearance::HumanInterfaceDevice(HumanInterfaceDevice::Joystick),
            (15, 4) => Appearance::HumanInterfaceDevice(HumanInterfaceDevice::Gamepad),
            (15, 5) => Appearance::HumanInterfaceDevice(HumanInterfaceDevice::DigitizerTablet),
            (15, 6) => Appearance::HumanInterfaceDevice(HumanInterfaceDevice::CardReader),
            (15, 7) => Appearance::HumanInterfaceDevice(HumanInterfaceDevice::DigitalPen),
            (15, 8) => Appearance::HumanInterfaceDevice(HumanInterfaceDevice::BarcodeScanner),
            (15, _) => Appearance::HumanInterfaceDevice(HumanInterfaceDevice::Generic),
            (16, _) => Appearance::GlucoseMeter,
            (17, 1) => Appearance::RunningWalkingSensor(RunningWalkingSensor::InShoe),
            (17, 2) => Appearance::RunningWalkingSensor(RunningWalkingSensor::OnShoe),
            (17, 3) => Appearance::RunningWalkingSensor(RunningWalkingSensor::OnHip),
            (17, _) => Appearance::RunningWalkingSensor(RunningWalkingSensor::Generic),
            (18, 1) => Appearance::Cycling(Cycling::Computer),
            (18, 2) => Appearance::Cycling(Cycling::SpeedSensor),
            (18, 3) => Appearance::Cycling(Cycling::CadenceSensor),
            (18, 4) => Appearance::Cycling(Cycling::PowerSensor),
            (18, 5) => Appearance::Cycling(Cycling::SpeedAndCadenceSensor),
            (18, _) => Appearance::Cycling(Cycling::Generic),
            (19, 1) => Appearance::GenericControlDevice(GenericControlDevice::Switch),
            (19, 2) => Appearance::GenericControlDevice(GenericControlDevice::MultiSwitch),
            (19, 3) => Appearance::GenericControlDevice(GenericControlDevice::Button),
            (19, 4) => Appearance::GenericControlDevice(GenericControlDevice::Slider),
            (19, 5) => Appearance::GenericControlDevice(GenericControlDevice::Rotary),
            (19, 6) => Appearance::GenericControlDevice(GenericControlDevice::TouchPanel),
            (19, _) => Appearance::GenericControlDevice(GenericControlDevice::Generic),
            (20, 1) => Appearance::GenericNetworkDevice(GenericNetworkDevice::AccessPoint),
            (20, _) => Appearance::GenericNetworkDevice(GenericNetworkDevice::Generic),
            (21, 1) => Appearance::Sensor(Sensor::Motion),
            (21, 2) => Appearance::Sensor(Sensor::AirQuality),
            (21, 3) => Appearance::Sensor(Sensor::Temperature),
            (21, 4) => Appearance::Sensor(Sensor::Humidity),
            (21, 5) => Appearance::Sensor(Sensor::Leak),
            (21, 6) => Appearance::Sensor(Sensor::Smoke),
            (21, 7) => Appearance::Sensor(Sensor::Occupancy),
            (21, 8) => Appearance::Sensor(Sensor::Contact),
            (21, 9) => Appearance::Sensor(Sensor::CarbonMonoxide),
            (21, 10) => Appearance::Sensor(Sensor::CarbonDioxide),
            (21, 11) => Appearance::Sensor(Sensor::AmbientLight),
            (21, 12) => Appearance::Sensor(Sensor::Energy),
            (21, 13) => Appearance::Sensor(Sensor::ColorLight),
            (21, 14) => Appearance::Sensor(Sensor::Rain),
            (21, 15) => Appearance::Sensor(Sensor::Fire),
            (21, 16) => Appearance::Sensor(Sensor::Wind),
            (21, 17) => Appearance::Sensor(Sensor::Proximity),
            (21, 18) => Appearance::Sensor(Sensor::MultiSensor),
            (21, _) => Appearance::Sensor(Sensor::Generic),
            (22, 1) => Appearance::LightFixture(LightFixture::WallLight),
            (22, 2) => Appearance::LightFixture(LightFixture::CeilingLight),
            (22, 3) => Appearance::LightFixture(LightFixture::FloorLight),
            (22, 4) => Appearance::LightFixture(LightFixture::CabinetLight),
            (22, 5) => Appearance::LightFixture(LightFixture::DeskLight),
            (22, 6) => Appearance::LightFixture(LightFixture::TrofferLight),
            (22, 7) => Appearance::LightFixture(LightFixture::PendandtLight),
            (22, 8) => Appearance::LightFixture(LightFixture::InGroundLight),
            (22, 9) => Appearance::LightFixture(LightFixture::FloodLight),
            (22, 10) => Appearance::LightFixture(LightFixture::UnderwaterLight),
            (22, 11) => Appearance::LightFixture(LightFixture::BollardLight),
            (22, 12) => Appearance::LightFixture(LightFixture::PathwatLight),
            (22, 13) => Appearance::LightFixture(LightFixture::GardenLight),
            (22, 14) => Appearance::LightFixture(LightFixture::PoleTopLight),
            (22, 15) => Appearance::LightFixture(LightFixture::Spotlight),
            (22, 16) => Appearance::LightFixture(LightFixture::LinearLight),
            (22, 17) => Appearance::LightFixture(LightFixture::StreetLight),
            (22, 18) => Appearance::LightFixture(LightFixture::ShelvesLight),
            (22, 19) => Appearance::LightFixture(LightFixture::HighBayLowBayLight),
            (22, 20) => Appearance::LightFixture(LightFixture::EmergencyExitLight),
            (22, _) => Appearance::Fan(Fan::Generic),
            (23, 1) => Appearance::Fan(Fan::Ceiling),
            (23, 2) => Appearance::Fan(Fan::Axial),
            (23, 3) => Appearance::Fan(Fan::Exhaust),
            (23, 4) => Appearance::Fan(Fan::Pedestal),
            (23, 5) => Appearance::Fan(Fan::Desk),
            (23, 6) => Appearance::Fan(Fan::Wall),
            (23, _) => Appearance::Fan(Fan::Generic),
            (_, _) => Appearance::Unknown,
        }
    }
}
