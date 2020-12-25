mod morse_table;
#[cfg(test)]
mod tests;

use crate::commands::util;
use clap::{App, Arg, ArgMatches, SubCommand};
use either::Either::Left;
use itertools::{Itertools, Position};
use log::{info, trace};
use luxafor_usb::device::{BitFlags, Luxafor, SimpleColor};
use morse_table::{GapType, Morsel, Morsel::*};
use std::{io, io::Write, str::FromStr, thread, time::Duration};

const fn dot_duration(wpm: u64) -> Duration {
    Duration::from_millis(1200 / wpm)
}

fn symbol_duration(symbol: &Morsel, dot_duration: Duration) -> Duration {
    match symbol {
        Dot | Gap(GapType::Symbol) => dot_duration,
        Dash | Gap(GapType::Letter) => dot_duration * 3,
        Gap(GapType::Word) => dot_duration * 7,
    }
}

fn word_to_morse<'a>(word: &String) -> Vec<&'a Morsel> {
    word.chars()
        .map(|c| c.to_ascii_uppercase())
        .filter_map(|c| morse_table::char_to_morse(c).ok())
        .with_position()
        .fold(
            Vec::with_capacity(word.len() * 2),
            |mut arr, letter_morsels| {
                match letter_morsels {
                    Position::First(l) | Position::Middle(l) => {
                        l.iter().for_each(|e| arr.push(e));
                        arr.push(&Gap(GapType::Letter));
                    }
                    Position::Last(l) | Position::Only(l) => l.iter().for_each(|e| arr.push(e)),
                }
                arr // matey
            },
        )
}

fn words_to_morse<'a>(words: &Vec<String>) -> Vec<&'a Morsel> {
    words
        .iter()
        .with_position()
        .fold(Vec::with_capacity(words.len() * 11), |mut arr, word| {
            match word {
                Position::First(w) | Position::Middle(w) => {
                    word_to_morse(w).iter().for_each(|e| arr.push(e));
                    arr.push(&Gap(GapType::Word));
                }
                Position::Last(w) | Position::Only(w) => {
                    word_to_morse(w).iter().for_each(|e| arr.push(e));
                }
            }
            arr // matey
        })
}

pub fn string_to_morse<'a>(words: &String) -> Vec<&'a Morsel> {
    words_to_morse(&words.split_whitespace().map(String::from).collect())
}

pub struct Morse {}

impl Morse {
    pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("morse")
            .about("Signal a message in Morse code")
            .arg(
                Arg::with_name("MESSAGE")
                    .short("m")
                    .long("message")
                    .help("Your message.  (Can be as long as patience and available memory allow.)")
                    .validator(|m| match m.len() {
                        0 => Err(String::from("A message was not provided.")),
                        _ => Ok(()),
                    })
                    .required(true)
                    .require_equals(true)
                    .empty_values(false),
            )
            .arg(
                Arg::with_name("COLOR")
                    .short("c")
                    .long("color")
                    .help("The color in which to flash the light when signaling your message, either as a named color (red, green, blue, cyan, yellow, magenta, white), or a numeric color specified in R,G,B (where R, G, B are from 0-255) decimal, #RRGGBB hex, or #RGB CSS-style shorthand hex.  #RGB will be expanded as in CSS, i.e., #b0b => #bb00bb.")
                    .default_value("white")
            )
            .arg(
                Arg::with_name("SPEED")
                    .short("s")
                    .long("speed")
                    .help("The speed at which to signal your message, in words per minute.")
                    .long_help("The speed at which to signal your message, in words per minute.  The duration of each symbol, in milliseconds, is calculated by 1200/SPEED.")
                    .default_value("10")
                    .validator(util::validate_string_is_nonzero_u64)
            )
            .arg(
                Arg::with_name("QUIET")
                    .short("q")
                    .long("quiet")
                    .help("Do not also echo Morse code to the terminal as it is flashed.  (This will always be set if stdout is not a tty.)")
                    .required(false)
                    .multiple(false)
            )
    }

    pub fn exec(opts: &ArgMatches) -> Result<(), String> {
        trace!("executing 'morse' command");
        let luxafor = Luxafor::new()?;

        let color_value = opts.value_of("COLOR").unwrap();
        let color = util::colorspec_to_rgb(Left(color_value))?;
        trace!("color is {:?}", color);

        let speed_value = opts.value_of("SPEED").unwrap();
        let speed = u64::from_str(speed_value)
            .expect("clap was supposed to have validated this!  Noooo...");
        let dot_duration = dot_duration(speed);
        info!("speed is {} wpm -- 1 dot = {:?}", speed, dot_duration);

        let message = String::from(
            opts.value_of("MESSAGE")
                .expect("clap was supposed to have validated this!  Noooo..."),
        );
        trace!("message is \"{}\"", message);

        let quiet = opts.is_present("QUIET") || !isatty::stdout_isatty();
        trace!("quiet is {}", quiet);

        let morsels = string_to_morse(&message);
        for morsel in morsels {
            let symbol_duration = symbol_duration(morsel, dot_duration);
            if !quiet {
                print!(
                    "{}",
                    match &morsel {
                        Dot => "•",
                        Dash => "-",
                        Gap(t) => match t {
                            GapType::Symbol => "",
                            GapType::Letter => " ",
                            GapType::Word => " / ",
                        },
                    }
                );
                io::stdout().flush().expect("failed to flush stdout");
            }
            match morsel {
                Dot | Dash => luxafor.set_rgb_color(color, BitFlags::all()),
                Gap(_) => luxafor.set_simple_color(SimpleColor::Off),
            }
            thread::sleep(symbol_duration);
        }
        luxafor.set_simple_color(SimpleColor::Off);
        if !quiet {
            println!();
        }

        Ok(())
    }
}
