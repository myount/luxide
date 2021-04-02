/* This file is part of Luxide, a command-line tool for operating the Luxafor Flag.
  Copyright © 2020, 2021 Mike Yount

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, version 3.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use crate::commands::util;
use clap::{App, Arg, ArgMatches, SubCommand};
use either::Either::Left;
use log::trace;
use luxafor_usb::device::{BitFlags, Luxafor};
use std::str::FromStr;

pub struct Strobe {}

impl Strobe {
    pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("strobe")
            .about("Strobes lights")
            .arg(
                Arg::with_name("COLOR")
                    .required(true)
                    .help("The color of the lights to be strobed.  Can be a named color or an RGB color.")
                    .long_help("The color of the lights to be strobed, either a named color (red, green, blue, cyan, yellow, magenta, white), or an RGB color specified in either R,G,B format with R, G, and B being in the range 0-255, or an HTML-style #RRGGBB, or CSS shorthand #RGB, hex color.  #RGB will be expanded to #RRGGBB as in CSS; that is, #b0b => #bb00bb.")
            )
            .arg(
                Arg::with_name("REPEATS")
                    .long("repeat")
                    .short("r")
                    .help("Number of times to flash the light (0-255).")
                    .required(false)
                    .default_value("3")
                    .validator(util::validate_string_is_u8)
            )
            .arg(
                Arg::with_name("LIGHTS")
                    .short("l")
                    .long("light")
                    .required(false)
                    .multiple(false)
                    .min_values(1)
                    .max_values(6)
                    .possible_values(&["all","flag","back","flag-bottom","flag-middle","flag-top","back-bottom","back-middle","back-top","1","2","3","4","5","6"])
                    .help("The light or lights you wish to strobe.  Defaults to \"all\".  Arbitrary combinations are not supported.")
                    .long_help("The light or lights you wish to strobe.  Defaults to \"all\".\nThe special values 'all', 'flag', and 'tab' may be used to set all the lights, only the flag lights, or only the back lights, or an arbitrary combination of other lights may be specified.  In addition, \"f\" and \"b\" may be used as shorthand for \"flag\" and \"back\", and the individual LEDs may be referred to numerically (1-6), with 1 being the bottom flag LED and 4 being the bottom back LED and going up from there.  Unlike other commands, arbitrary combinations of lights are not supported.")
            )
            .arg(
                Arg::with_name("SPEED")
                    .long("speed")
                    .short("s")
                    .help("The rate at which to strobe the light(s) (0-255).  Smaller values are faster.")
                    .long_help("The rate at which to strobe the light(s) (0-255).  Smaller values are faster.  The actual real-time duration of the flash is determined by the hardware and may differ depending on what color the lights are already set to.  Keep in mind that rapidly flashing lights can potentially trigger seizures for people with photosensitive epilepsy.")
                    .required(false)
                    .default_value("31")
                    .validator(util::validate_string_is_u8)
            )
    }

    pub fn exec(opts: &ArgMatches) -> Result<(), String> {
        trace!("executing \"strobe\" command");
        let luxafor = Luxafor::new()?;

        let color_value = opts.value_of("COLOR").unwrap();
        let color = util::colorspec_to_rgb(Left(color_value))?;
        trace!("color is {:?}", color);

        let lights_value = opts.values_of("LIGHTS");
        let lights = util::parse_lights(lights_value);
        trace!("lights is {:?}", lights);

        let repeat = u8::from_str(opts.value_of("REPEATS").unwrap())
            .expect("clap was supposed to have validated this!  Noooo...");
        trace!("repeat is {}", repeat);

        let speed = u8::from_str(opts.value_of("SPEED").unwrap())
            .expect("clap was supposed to have validated this!  Noooo...");
        trace!("speed is {}", speed);

        luxafor.strobe(color, lights.unwrap_or(BitFlags::all()), speed, repeat);

        Ok(())
    }
}
