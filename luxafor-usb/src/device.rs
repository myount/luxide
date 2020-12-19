use crate::usb::{LUXAFOR_PID, LUXAFOR_VID};
pub use enumflags2::BitFlags;
use hidapi::{HidApi, HidDevice};
use std::fmt;
use std::fmt::{Debug, Formatter};

pub struct Luxafor {
    pub(crate) hid_device: HidDevice,
}

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub(crate) enum Target {
    All = 0xFF,
    Back = 'A' as u8,
    Tab = 'B' as u8,
    TabBottomLed = 1,
    TabMiddleLed = 2,
    TabTopLed = 3,
    BackBottomLed = 4,
    BackMiddleLed = 5,
    BackTopLed = 6,
}

#[derive(BitFlags, Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum Lights {
    FlagBottom = 0b000001,
    FlagMiddle = 0b000010,
    FlagTop = 0b000100,
    BackBottom = 0b001000,
    BackMiddle = 0b010000,
    BackTop = 0b100000,
}

impl Lights {
    pub fn all() -> BitFlags<Lights> {
        Lights::FlagBottom
            | Lights::FlagMiddle
            | Lights::FlagTop
            | Lights::BackBottom
            | Lights::BackMiddle
            | Lights::BackTop
    }

    pub fn flag() -> BitFlags<Lights> {
        Lights::FlagBottom | Lights::FlagMiddle | Lights::FlagTop
    }

    pub fn back() -> BitFlags<Lights> {
        Lights::BackBottom | Lights::BackMiddle | Lights::BackTop
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum SimpleColor {
    Red = 'R' as u8,
    Green = 'G' as u8,
    Blue = 'B' as u8,
    Cyan = 'C' as u8,
    Magenta = 'M' as u8,
    Yellow = 'Y' as u8,
    White = 'W' as u8,
    Off = 'O' as u8,
}

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum WaveType {
    Short = 1,
    Long = 2,
    OverlappingShort = 3,
    OverlappingLong = 4,
}

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum PatternType {
    Luxafor = 1,
    Police = 5,
    Random1 = 2,
    Random2 = 3,
    Random3 = 4,
    Random4 = 6,
    Random5 = 7,
    RainbowWave = 8,
}

#[derive(Clone, Copy, Debug)]
pub struct RgbColor(pub u8, pub u8, pub u8);

impl RgbColor {
    pub fn red() -> RgbColor {
        RgbColor(255, 0, 0)
    }

    pub fn green() -> RgbColor {
        RgbColor(0, 255, 0)
    }

    pub fn blue() -> RgbColor {
        RgbColor(0, 0, 255)
    }

    pub fn cyan() -> RgbColor {
        RgbColor(0, 255, 255)
    }

    pub fn magenta() -> RgbColor {
        RgbColor(255, 0, 255)
    }

    pub fn yellow() -> RgbColor {
        RgbColor(255, 255, 0)
    }

    pub fn white() -> RgbColor {
        RgbColor(255, 255, 255)
    }

    pub fn off() -> RgbColor {
        RgbColor(0, 0, 0)
    }
}

impl Luxafor {
    pub fn new() -> Result<Self, String> {
        match HidApi::new() {
            // TODO: Theoretically a user could have multiple Luxafors...
            Ok(api) => match api.open(LUXAFOR_VID, LUXAFOR_PID) {
                Ok(hid_device) => match hid_device.get_product_string() {
                    Ok(result) => match result {
                        Some(str) => {
                            if !str.to_lowercase().contains("luxafor") {
                                Err(format!("Unexpected product string: {}", str))
                            } else {
                                Ok(Self { hid_device })
                            }
                        }
                        None => Ok(Self { hid_device }),
                    },
                    Err(e) => Err(String::from(e.to_string())),
                },
                Err(e) => {
                    let err = e.to_string();
                    Err(err)
                }
            },
            Err(e) => {
                let err = e.to_string();
                Err(err)
            }
        }
    }
}

impl Debug for Luxafor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match write!(f, "Luxafor device") {
            Ok(_) => (),
            Err(e) => return Err(e),
        };

        // I have a feeling these "match match ..."es are, shall we say, not
        // exactly best practice.  But it strikes me as preferable (however
        // minimally) to having only the inner matches with each arm having
        // its own match write!(...) {...}.
        match match self.hid_device.get_manufacturer_string() {
            Ok(Some(mfr)) => write!(f, ", manufacturer \"{}\"", mfr),
            Ok(None) => write!(f, " <manufacturer unknown>"),
            Err(_) => write!(f, " <error getting manufacturer string>"),
        } {
            Ok(_) => (),
            Err(e) => return Err(e),
        };

        match match self.hid_device.get_product_string() {
            Ok(Some(prod)) => write!(f, ", product \"{}\"", prod),
            Ok(None) => write!(f, " <product unknown>"),
            Err(_) => write!(f, " <error getting product string>"),
        } {
            Ok(_) => (),
            Err(e) => return Err(e),
        };

        match match self.hid_device.get_serial_number_string() {
            Ok(Some(ser)) => write!(f, ", s/n: {}", ser),
            Ok(None) => write!(f, ", <serial number unknown>"),
            Err(_) => write!(f, ""),
        } {
            Ok(_) => (),
            Err(e) => return Err(e),
        };

        Ok(())
    }
}
