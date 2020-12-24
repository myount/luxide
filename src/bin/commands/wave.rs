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

use crate::commands::util;
use clap::{App, Arg, ArgMatches, SubCommand};
use either::Either::Left;
use log::trace;
use luxafor_usb::device::{Luxafor, WaveType};
use std::str::FromStr;

pub struct Wave {}

impl Wave {
    pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("wave")
            .about("Animates lights in a wave pattern")
            .arg(
                Arg::with_name("WAVE-TYPE")
                    // .long("wave")
                    // .alias("wave-type")
                    // .short("w")
                    .help("The wave type (1-4) to display.")
                    .long_help("The wave type (1-4) to display.\nTypes 1 and 3 fade from off or the previous color, respectively, to the specified color, one LED at a time.\nTypes 2 and 4 fade up to an entire side of the device at one time from off or the previous color, respectively, to the specified color.")
                    .possible_values(&["1","2","3","4"])
                    .hide_possible_values(true)
                    .required(true)
            )
            .arg(
                Arg::with_name("COLOR")
                    .required(true)
                    .help("The color of the wave.  Can be a named color or an RGB color.")
                    .long_help("The color of the wave, either a named color (red, green, blue, cyan, yellow, magenta, white), or an RGB color specified in either R,G,B format with R, G, and B being in the range 0-255, or an HTML-style #RRGGBB, or CSS shorthand #RGB, hex color.  #RGB will be expanded to #RRGGBB as in CSS; that is, #b0b => #bb00bb.")
            )
            .arg(
                Arg::with_name("REPEATS")
                    .long("repeat")
                    .short("r")
                    .help("Number of times to repeat the wave (0-255).")
                    .required(false)
                    .default_value("3")
                    .validator(util::validate_string_is_u8)
            )
            .arg(
                Arg::with_name("SPEED")
                    .long("speed")
                    .short("s")
                    .help("Speed at which to animate the wave (0-255).  Smaller values are faster.")
                    .long_help("Speed at which to animate the wave (0-255).  Smaller values are faster.  The actual real-time durations that these correspond to are determined by the hardware.")
                    .required(false)
                    .default_value("31")
                    .validator(util::validate_string_is_u8)
            )
    }

    pub fn exec(opts: &ArgMatches) -> Result<(), String> {
        trace!("executing \"wave\" command");
        let luxafor = Luxafor::new()?;

        let color_value = opts.value_of("COLOR").unwrap();
        let color = util::colorspec_to_rgb(&Left(color_value))?;
        trace!("color is {:?}", color);

        let wave_type_num = u8::from_str(opts.value_of("WAVE-TYPE").unwrap())
            .expect("clap was supposed to have validated this!  Noooo...");
        let wave_type = match wave_type_num {
            1 => WaveType::Short,
            2 => WaveType::Long,
            3 => WaveType::OverlappingShort,
            4 => WaveType::OverlappingLong,
            _ => unreachable!(),
        };
        trace!("wave type is {} ({:?})", wave_type_num, wave_type);

        let repeat = u8::from_str(opts.value_of("REPEATS").unwrap())
            .expect("clap was supposed to have validated this!  Noooo...");
        trace!("repeat is {}", repeat);

        let speed = u8::from_str(opts.value_of("SPEED").unwrap())
            .expect("clap was supposed to have validated this!  Noooo...");
        trace!("speed is {}", speed);

        luxafor.wave(color, wave_type, speed, repeat);

        Ok(())
    }
}
