mod commands;

use clap::{App, Arg, ArgGroup, SubCommand};
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
        .subcommand(
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
                        .help("The light or lights whose color you wish to set.  Specify once for each light (e.g., -l flag-top -l back-top).")
                        .long_help("The light or lights whose color you wish to set.  Specify once for each light (e.g., -l flag-top -l back-top).\nThe special values 'all', 'flag', and 'tab' may be used to set all the lights, only the flag lights, or only the back lights, or an arbitrary combination of other lights may be specified.  In addition, \"f\" and \"b\" may be used as shorthand for \"flag\" and \"back\", and the individual LEDs may be referred to numerically (1-6), with 1 being the bottom flag LED and 4 being the bottom back LED and going up from there.")
                        .default_value("all")
                )
        )
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
