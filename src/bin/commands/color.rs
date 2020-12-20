use clap::{App, Arg, ArgGroup, ArgMatches, SubCommand, Values};
use log::{debug, error, trace};
use luxafor_usb::device::{BitFlags, Lights, Luxafor, RgbColor, SimpleColor};
use std::str::FromStr;

pub struct Color {}

trait Parse<T> {
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

#[derive(Debug)]
enum ColorSpec<'a> {
    NamedColor(&'a str),
    NumericColor(&'a str),
}

fn parse_lights(values: Option<Values>) -> Option<BitFlags<Lights>> {
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

impl Color {
    pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("color")
            .about("Sets the color of the flag")
            .arg(Arg::with_name("COLOR")
                .index(1)
                .possible_values(&["red", "green", "blue", "cyan", "magenta", "yellow", "white", "off"])
                .help("One of the eight pre-defined colors.  Either this or --rgb <RGB> is required.")
            )
            .arg(
                Arg::with_name("RGB")
                    .short("r")
                    .long("rgb")
                    .alias("rgb-color")
                    .empty_values(false)
                    .help("The RGB color to set (R,G,B decimal, or #RRGGBB or #RGB hex).  Either this or <COLOR> is required.")
                    .long_help("The RGB color to set, specified in either R,G,B format with R, G, and B being in the range 0-255, or an HTML-style #RRGGBB, or CSS shorthand #RGB, hex color.  #RGB will be expanded to #RRGGBB as in CSS; that is, #b0b => #bb00bb.")
            )
            .group(
                ArgGroup::with_name("colors")
                    .args(&["COLOR", "RGB"])
                    .required(true)
            )
            .arg(
                Arg::with_name("DURATION")
                    .short("f")
                    .long("fade")
                    .empty_values(true)
                    .default_value_if("COLOR", None, "")
                    .default_value_if("RGB", None, "0")
                    .help("The duration (0-255) over which to fade to the given color.  Smaller values are faster (0 is instant, and the default).")
                    .long_help("The duration (0-255) over which to fade to the given color.  Smaller values are faster (0 is instant, and the default).  The precise duration that corresponds to <DURATION> is determined by the hardware, and the same value may produce different real-time durations vary based on the starting and ending colors.")
            )
            .arg(
                Arg::with_name("LIGHTS")
                    .short("l")
                    .long("light")
                    .required(false)
                    .multiple(true)
                    .min_values(1)
                    .max_values(6)
                    .possible_values(&["all","flag","back","flag-bottom","flag-middle","flag-top","back-bottom","back-middle","back-top","1","2","3","4","5","6"])
                    .help("The light or lights whose color you wish to set.  Specify once for each light (e.g., -l flag-top -l back-top).  Defaults to \"all\".")
                    .long_help("The light or lights whose color you wish to set.  Specify once for each light (e.g., -l flag-top -l back-top).  Defaults to \"all\".\nThe special values 'all', 'flag', and 'tab' may be used to set all the lights, only the flag lights, or only the back lights, or an arbitrary combination of other lights may be specified.  In addition, \"f\" and \"b\" may be used as shorthand for \"flag\" and \"back\", and the individual LEDs may be referred to numerically (1-6), with 1 being the bottom flag LED and 4 being the bottom back LED and going up from there.")
            )
    }

    pub fn exec(opts: &ArgMatches) -> Result<(), String> {
        trace!("executing \"color\" subcommand");
        let luxafor = Luxafor::new()?;

        let duration_value = opts.value_of("DURATION").unwrap();
        let color_value = if opts.is_present("COLOR") {
            ColorSpec::NamedColor(opts.value_of("COLOR").unwrap())
        } else if opts.is_present("RGB") {
            ColorSpec::NumericColor(opts.value_of("RGB").unwrap())
        } else {
            // I'm trusting here that arg parsing will properly enforce that either DURATION or
            // COLOR is provided and thus this should be...
            unreachable!();
        };
        let lights_value = opts.values_of("LIGHTS");

        // Can't specify a duration or individual lights with "set simple color".
        if duration_value != "" || lights_value.is_some() {
            trace!(
                "simple color name specified with fade duration or lights - forcing fade command"
            );
            let color = match color_value {
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
            }?;

            trace!("color {:?} is {:?}", color_value, color);

            let lights = parse_lights(lights_value);
            trace!("lights is {:?}", lights);

            let fade_time_value = opts.value_of("DURATION").unwrap();
            let fade_time = match u8::from_str(fade_time_value) {
                Ok(t) => t,
                Err(_) => 0,
            };
            trace!("fade_time is \"{}\" = {:?}", fade_time_value, fade_time);

            luxafor.fade_to_color(color, lights.unwrap_or(BitFlags::all()), fade_time);
        } else {
            let color = match color_value {
                ColorSpec::NumericColor(_) => unreachable!(),
                ColorSpec::NamedColor(name) => match name.to_lowercase().as_str() {
                    "red" => Ok(SimpleColor::Red),
                    "green" => Ok(SimpleColor::Green),
                    "blue" => Ok(SimpleColor::Blue),
                    "cyan" => Ok(SimpleColor::Cyan),
                    "magenta" => Ok(SimpleColor::Magenta),
                    "yellow" => Ok(SimpleColor::Yellow),
                    "white" => Ok(SimpleColor::White),
                    "off" => Ok(SimpleColor::Off),
                    s => {
                        error!("Unrecognized simple color: \"{}\"", s);
                        Err(format!("Invalid color: \"{}\"", s))
                    }
                },
            }?;

            trace!("color {:?} is {:?}", color_value, color);
            luxafor.set_simple_color(color);
        }

        Ok(())
    }
}
