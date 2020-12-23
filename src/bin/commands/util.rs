use crate::commands::util::ColorSpec::{NamedColor, NumericColor};
use clap::Values;
use either::Either;
use either::Either::{Left, Right};
use log::error;
use luxafor_usb::device::{BitFlags, Lights, RgbColor};
use std::convert::TryFrom;

impl<'a> TryFrom<&'a str> for ColorSpec<'a> {
    type Error = ();

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        if value.len() > 0 {
            if value.starts_with("#") {
                match value.len() {
                    4 | 7 => Ok(NumericColor(&value)),
                    _ => Err(()),
                }
            } else if value.match_indices(",").collect::<Vec<_>>().len() == 2 {
                Ok(NumericColor(&value))
            } else {
                Ok(NamedColor(&value))
            }
        } else {
            Err(())
        }
    }
}

pub struct ColorSpecParseError(String);
impl From<ColorSpecParseError> for String {
    fn from(e: ColorSpecParseError) -> Self {
        e.0
    }
}

pub(crate) fn colorspec_to_rgb(
    &color_spec: &Either<&str, &ColorSpec>,
) -> Result<RgbColor, ColorSpecParseError> {
    match color_spec {
        Left(str) => match str.to_lowercase().as_str() {
            "red" => Ok(RgbColor::red()),
            "green" => Ok(RgbColor::green()),
            "blue" => Ok(RgbColor::blue()),
            "cyan" => Ok(RgbColor::cyan()),
            "magenta" => Ok(RgbColor::magenta()),
            "yellow" => Ok(RgbColor::yellow()),
            "white" => Ok(RgbColor::white()),
            "off" => Ok(RgbColor::off()),
            s => match RgbColor::try_from(s) {
                Ok(color) => Ok(color),
                Err(e) => Err(ColorSpecParseError(e.to_string())),
            },
        },
        Right(color_spec) => match *color_spec {
            ColorSpec::NamedColor(name) => match name.to_lowercase().as_str() {
                s => {
                    error!("Unrecognized color name \"{}\"", s);
                    Err(ColorSpecParseError(format!("Invalid color: \"{}\"", s)))
                }
            },
            ColorSpec::NumericColor(num) => match RgbColor::try_from(num) {
                Ok(color) => Ok(color),
                Err(e) => Err(ColorSpecParseError(e.to_string())),
            },
        },
    }
}

pub(crate) fn parse_lights(values: Option<Values>) -> Option<BitFlags<Lights>> {
    if values.is_none() {
        None
    } else {
        let lights = values
            .unwrap()
            .map(|v| v.to_lowercase())
            .collect::<Vec<String>>();

        if lights.contains(&"all".to_string()) {
            Some(Lights::all())
        } else if lights.is_empty() {
            // Should always be false, but...
            None
        } else {
            let mut flags = BitFlags::<Lights>::empty();

            lights.iter().for_each(|l| match l.to_lowercase().as_str() {
                // "all","flag","back","flag-bottom","flag-middle","flag-top","back-bottom","back-middle","back-top"
                "all" => flags |= Lights::all(),
                "f" | "flag" => flags |= Lights::flag(),
                "b" | "back" => flags |= Lights::back(),
                "1" | "flag-bottom" => flags |= Lights::FlagBottom,
                "2" | "flag-middle" => flags |= Lights::FlagMiddle,
                "3" | "flag-top" => flags |= Lights::FlagTop,
                "4" | "back-bottom" => flags |= Lights::BackBottom,
                "5" | "back-middle" => flags |= Lights::BackMiddle,
                "6" | "back_top" => flags |= Lights::BackTop,
                _ => (),
            });

            Some(flags)
        }
    }
}

#[derive(Debug)]
pub(crate) enum ColorSpec<'a> {
    NamedColor(&'a str),
    NumericColor(&'a str),
}
