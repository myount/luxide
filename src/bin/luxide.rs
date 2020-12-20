mod commands;

use clap::{App, Arg};
use log::error;
use simplelog::*;

use commands::color::Color;

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
        (cmd, _) => error!("Unrecognized command {}.  Try --help for help", cmd),
    }
}
