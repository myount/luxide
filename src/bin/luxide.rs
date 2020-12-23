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

mod commands;

use clap::{App, Arg};
use log::error;
use simplelog::*;

use commands::{color::Color, wave::Wave};

fn main() {
    let opts = App::new("Luxide")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Mike Yount <michael@yount.me>")
        .about("Command-line control for Luxafor flags.")
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .multiple(true)
                .help("Increase verbosity (-v prints info; -vv prints debug output; -vvv prints trace output)"),
        )
        .subcommand(Color::subcommand())
        .subcommand(Wave::subcommand())
        .get_matches();

    TermLogger::init(
        match opts.occurrences_of("verbose") {
            0 => LevelFilter::Error,
            1 => LevelFilter::Info,
            2 => LevelFilter::Debug,
            3 => LevelFilter::Trace,
            _ => LevelFilter::Trace,
        },
        ConfigBuilder::new()
            .set_location_level(LevelFilter::Off)
            .set_time_level(LevelFilter::Off)
            .set_thread_level(LevelFilter::Off)
            .set_target_level(LevelFilter::Off)
            .build(),
        TerminalMode::Mixed,
    )
    .unwrap();

    match opts.subcommand() {
        ("color", Some(opts)) => match Color::exec(opts) {
            Ok(_) => (),
            Err(e) => error!("{}", e),
        },
        ("wave", Some(opts)) => match Wave::exec(opts) {
            Ok(_) => (),
            Err(e) => error!("{}", e),
        },
        (cmd, _) => error!("Unrecognized command {}.  Try --help for help", cmd),
    }
}
