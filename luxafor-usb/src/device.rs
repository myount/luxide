/* This file is part of luxafor-usb, a Rust library for communicating with Luxafor Flags.
  Copyright © 2020 Mike Yount

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU Lesser General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU Lesser General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use crate::usb::{LUXAFOR_PID, LUXAFOR_VID};
pub use enumflags2::BitFlags;
use hidapi::{HidApi, HidDevice};
use log::{debug, trace};
use std::{
    convert::TryFrom,
    fmt,
    fmt::{Debug, Formatter},
    str::FromStr,
};
use thiserror::Error;

pub struct Luxafor {
    pub(crate) hid_device: HidDevice,
}

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub(crate) enum Target {
    All = 0xFF,
    Back = 'B' as u8,
    Tab = 'A' as u8,
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

#[derive(Error, Debug)]
pub enum RgbColorParseError {
    #[error("The hex color was invalid. Either it was the incorrect length or contained non-hex digits.")]
    InvalidHexColor(String),
    #[error("The R,G,B color was invalid. Either it was composed of the wrong number of parts, or contained a value over 255.")]
    InvalidNumericColor(String),
}

impl TryFrom<&str> for RgbColor {
    type Error = RgbColorParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        trace!("attempting to parse \"{}\" to RGB color", value);

        if value.starts_with("#") {
            trace!("found possible hex color");

            // I didn't want to make these mutable, I wanted to just initialize them in the match
            // below, but rustc is (by design) not clever enough to see that they're only used if
            // they get initialized.
            let mut r: u8 = 0;
            let mut g: u8 = 0;
            let mut b: u8 = 0;

            let from_hex = |s: &str| -> Result<u8, RgbColorParseError> {
                match u8::from_str_radix(s, 16) {
                    Ok(i) => Ok(i),
                    Err(e) => Err(RgbColorParseError::InvalidHexColor(e.to_string())),
                }
            };

            let res = match value.len() {
                4 => {
                    trace!("found possible CSS shorthand-style color");
                    r = from_hex(&value[1..2])? * 16 + from_hex(&value[1..2])?;
                    g = from_hex(&value[2..3])? * 16 + from_hex(&value[2..3])?;
                    b = from_hex(&value[3..4])? * 16 + from_hex(&value[3..4])?;
                    Ok(())
                }
                7 => {
                    trace!("found possible HTML color");
                    r = from_hex(&value[1..3])?;
                    g = from_hex(&value[3..5])?;
                    b = from_hex(&value[5..7])?;
                    Ok(())
                }
                l => {
                    let msg = format!("Invalid length for a hex color ({})", l);

                    trace!("{}", msg);
                    Err(msg)
                }
            };

            if res.is_ok() {
                let color = RgbColor(r, g, b);
                debug!("parsed \"{}\" to {:?}", value, color);
                Ok(color)
            } else {
                debug!("couldn't parse \"{}\" to RgbColor", value);
                Err(RgbColorParseError::InvalidHexColor(res.unwrap_err()))
            }
        } else {
            let from_dec = |s: &str| -> Result<u8, RgbColorParseError> {
                match u8::from_str(s) {
                    Ok(i) => Ok(i),
                    Err(e) => Err(RgbColorParseError::InvalidNumericColor(e.to_string())),
                }
            };

            trace!("not a hex color - attempting to parse as r,g,b");
            let parts = value.split(',').collect::<Vec<&str>>();
            if parts.len() != 3 {
                let msg = format!(
                    "{} parts ({})",
                    if parts.len() < 3 {
                        "not enough"
                    } else {
                        "too many"
                    },
                    parts.len()
                );
                debug!("couldn't parse \"{}\" as r,g,b: {}", value, msg);
                Err(RgbColorParseError::InvalidNumericColor(msg))
            } else {
                let rgb = parts
                    .iter()
                    .filter_map(|s| from_dec(&s.trim()).ok())
                    .collect::<Vec<u8>>();
                if rgb.len() == 3 {
                    let res = RgbColor(rgb[0], rgb[1], rgb[2]);
                    debug!("parsed \"{}\" to {:?}", value, res);
                    Ok(res)
                } else {
                    let msg = format!(
                        "couldn't parse \"{}\" to RgbColor - parsed {} segments, expected 3",
                        value,
                        rgb.len()
                    );
                    debug!("{}", msg);
                    Err(RgbColorParseError::InvalidNumericColor(msg))
                }
            }
        }
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
