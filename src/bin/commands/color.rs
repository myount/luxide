/* This file is part of Luxide, a command-line tool for operating the Luxafor Flag.
  Copyright © 2020 Mike Yount

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use crate::commands::{util, util::ColorSpec};
use clap::{App, Arg, ArgGroup, ArgMatches, SubCommand};
use either::Either::Right;
use log::{error, trace};
use luxafor_usb::device::{BitFlags, Luxafor, SimpleColor};
use std::str::FromStr;

pub struct Color {}

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
            let color = util::colorspec_to_rgb(&Right(&color_value))?;
            trace!("color {:?} is {:?}", color_value, color);

            let lights = util::parse_lights(lights_value);
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
