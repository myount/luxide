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
use log::trace;
use luxafor_usb::device::{Luxafor, PatternType};
use std::str::FromStr;

pub struct Pattern {}

impl Pattern {
    pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("pattern")
            .about("Displays one of a few built-in patterns")
            .arg(
                Arg::with_name("PATTERN")
                    .required(true)
                    .help("The pattern type to play.")
                    .long_help("The pattern type to play.  May be specified numerically (1-8) or by identifier.  The identifiers correspond to the named patterns in the Luxafor software.")
                    .possible_values(&["luxafor","police","random1","random2","random3","random4","random5","rainbow-wave","1","2","3","4","5","6","7","8"])
            )
            .arg(Arg::with_name("REPEAT")
                .required(false)
                .help("The number of times to repeat the pattern (0-255).")
                .default_value("3")
                .validator(util::validate_string_is_u8)
            )
    }

    pub fn exec(opts: &ArgMatches) -> Result<(), String> {
        trace!("executing \"pattern\" command");
        let luxafor = Luxafor::new()?;

        let pattern_value = opts
            .value_of("PATTERN")
            .expect("clap was supposed to enforce the presence of this!  Noooo...");
        let pattern = match pattern_value.to_lowercase().as_str() {
            "1" | "luxafor" => Ok(PatternType::Luxafor),
            "2" | "random1" => Ok(PatternType::Random1),
            "3" | "random2" => Ok(PatternType::Random2),
            "4" | "random3" => Ok(PatternType::Random3),
            "6" | "random4" => Ok(PatternType::Random4),
            "7" | "random5" => Ok(PatternType::Random5),
            "8" | "rainbow-wave" | "rainbow" => Ok(PatternType::RainbowWave),
            "5" | "police" => Ok(PatternType::Police),
            p => Err(format!("An invalid pattern type {} was specified.", p)),
        }?;
        trace!("pattern is {} {:?}", pattern_value, pattern);

        let repeat = u8::from_str(opts.value_of("REPEAT").unwrap())
            .expect("clap was supposed to have validated this!  Noooo...");
        trace!("repeat is {}", repeat);

        luxafor.pattern(pattern, repeat);

        Ok(())
    }
}
