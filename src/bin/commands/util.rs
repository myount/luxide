use clap::Values;
use log::{debug, error, trace};
use luxafor_usb::device::{BitFlags, Lights, RgbColor};
use std::str::FromStr;

pub(crate) trait Parse<T> {
    fn parse(str: &str) -> Option<T>;
}

impl Parse<RgbColor> for RgbColor {
    fn parse(str: &str) -> Option<RgbColor> {
        trace!("attempting to parse \"{}\" to RGB color", str);

        if str.starts_with("#") {
            trace!("found possible hex color");

            // I didn't want to make these mutable, I wanted to just initialize them in the match
            // below, but rustc is (by design) not clever enough to see that they're only used if
            // they get initialized.
            let mut r: u8 = 0;
            let mut g: u8 = 0;
            let mut b: u8 = 0;

            let res = match str.len() {
                4 => {
                    trace!("found possible CSS shorthand-style color");
                    r = (u8::from_str_radix(&str[1..2], 16).ok()? * 16)
                        + u8::from_str_radix(&str[1..2], 16).ok()?;
                    g = (u8::from_str_radix(&str[2..3], 16).ok()? * 16)
                        + u8::from_str_radix(&str[2..3], 16).ok()?;
                    b = (u8::from_str_radix(&str[3..4], 16).ok()? * 16)
                        + u8::from_str_radix(&str[3..4], 16).ok()?;
                    Some(())
                }
                7 => {
                    trace!("found possible HTML color");
                    r = u8::from_str_radix(&str[1..3], 16).ok()?;
                    g = u8::from_str_radix(&str[3..5], 16).ok()?;
                    b = u8::from_str_radix(&str[5..7], 16).ok()?;
                    Some(())
                }
                l => {
                    trace!(
                        "\"{}\" is an invalid length ({}) for a valid hex color",
                        str,
                        l
                    );
                    None
                }
            };

            if res.is_some() {
                let color = Some(RgbColor(r, g, b));
                debug!("parsed \"{}\" to {:?}", str, color.unwrap());
                color
            } else {
                debug!("couldn't parse \"{}\" to RgbColor", str);
                None
            }
        } else {
            trace!("not a hex color - attempting to parse as r,g,b");
            let parts = str.split(',').collect::<Vec<&str>>();
            if parts.len() != 3 {
                debug!(
                    "couldn't parse \"{}\" as RgbColor - {} parts ({})",
                    str,
                    if parts.len() < 3 {
                        "not enough"
                    } else {
                        "too many"
                    },
                    parts.len()
                );
                None
            } else {
                let rgb = parts
                    .iter()
                    .filter_map(|s| u8::from_str(&s.trim()).ok())
                    .collect::<Vec<u8>>();
                if rgb.len() == 3 {
                    let res = Some(RgbColor(rgb[0], rgb[1], rgb[2]));
                    debug!("parsed \"{}\" to {:?}", str, res.unwrap());
                    res
                } else {
                    debug!(
                        "couldn't parse \"{}\" to RgbColor - parsed {} segments, expected 3",
                        str,
                        rgb.len()
                    );
                    None
                }
            }
        }
    }
}

pub(crate) fn colorspec_to_rgb(color_spec: &ColorSpec) -> Result<RgbColor, String> {
    match color_spec {
        ColorSpec::NamedColor(name) => match name.to_lowercase().as_str() {
            "red" => Ok(RgbColor::red()),
            "green" => Ok(RgbColor::green()),
            "blue" => Ok(RgbColor::blue()),
            "cyan" => Ok(RgbColor::cyan()),
            "magenta" => Ok(RgbColor::magenta()),
            "yellow" => Ok(RgbColor::yellow()),
            "white" => Ok(RgbColor::white()),
            "off" => Ok(RgbColor::off()),
            s => {
                error!("Unrecognized color name \"{}\"", s);
                Err(format!("Invalid color: \"{}\"", s))
            }
        },
        ColorSpec::NumericColor(num) => {
            RgbColor::parse(num).ok_or(format!("Couldn't parse RGB color: \"{}\"", num))
        }
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
