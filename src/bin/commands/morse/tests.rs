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

use crate::commands::morse;
use crate::commands::morse::morse_table::{
    GapType::{Letter, Symbol, Word},
    Morsel::{Dash, Dot, Gap},
};

#[test]
fn word_to_morse() {
    let word = String::from("sos");
    let morse = morse::word_to_morse(&word);
    let expected = vec![
        &Dot,
        &Gap(Symbol),
        &Dot,
        &Gap(Symbol),
        &Dot,
        &Gap(Letter),
        &Dash,
        &Gap(Symbol),
        &Dash,
        &Gap(Symbol),
        &Dash,
        &Gap(Letter),
        &Dot,
        &Gap(Symbol),
        &Dot,
        &Gap(Symbol),
        &Dot,
    ];

    println!("{}", word);
    println!("expected: {:?}", expected);
    println!("got:      {:?}", morse);

    assert_eq!(morse.len(), expected.len(), "Arrays aren't the same length");
    assert_eq!(
        format!("{:?}", morse),
        format!("{:?}", expected),
        "Debug output differs"
    );
}

#[test]
fn words_to_morse() {
    let words = vec![String::from("sos"), String::from("lol")];
    let morse = morse::words_to_morse(&words);
    let expected = vec![
        &Dot,
        &Gap(Symbol),
        &Dot,
        &Gap(Symbol),
        &Dot,
        &Gap(Letter),
        &Dash,
        &Gap(Symbol),
        &Dash,
        &Gap(Symbol),
        &Dash,
        &Gap(Letter),
        &Dot,
        &Gap(Symbol),
        &Dot,
        &Gap(Symbol),
        &Dot,
        &Gap(Word),
        &Dot,
        &Gap(Symbol),
        &Dash,
        &Gap(Symbol),
        &Dot,
        &Gap(Symbol),
        &Dot,
        &Gap(Letter),
        &Dash,
        &Gap(Symbol),
        &Dash,
        &Gap(Symbol),
        &Dash,
        &Gap(Letter),
        &Dot,
        &Gap(Symbol),
        &Dash,
        &Gap(Symbol),
        &Dot,
        &Gap(Symbol),
        &Dot,
    ];

    println!("{:?}", words);
    println!("expected: {:?}", expected);
    println!("got:      {:?}", morse);

    assert_eq!(morse.len(), expected.len(), "Arrays aren't the same length");
    assert_eq!(
        format!("{:?}", morse),
        format!("{:?}", expected),
        "Debug output differs"
    );
}

#[test]
fn string_to_morse() {
    let string = String::from("sos lol");
    let morse = morse::string_to_morse(&string);
    let expected = vec![
        &Dot,
        &Gap(Symbol),
        &Dot,
        &Gap(Symbol),
        &Dot,
        &Gap(Letter),
        &Dash,
        &Gap(Symbol),
        &Dash,
        &Gap(Symbol),
        &Dash,
        &Gap(Letter),
        &Dot,
        &Gap(Symbol),
        &Dot,
        &Gap(Symbol),
        &Dot,
        &Gap(Word),
        &Dot,
        &Gap(Symbol),
        &Dash,
        &Gap(Symbol),
        &Dot,
        &Gap(Symbol),
        &Dot,
        &Gap(Letter),
        &Dash,
        &Gap(Symbol),
        &Dash,
        &Gap(Symbol),
        &Dash,
        &Gap(Letter),
        &Dot,
        &Gap(Symbol),
        &Dash,
        &Gap(Symbol),
        &Dot,
        &Gap(Symbol),
        &Dot,
    ];

    println!("{:?}", string);
    println!("expected: {:?}", expected);
    println!("got:      {:?}", morse);

    assert_eq!(morse.len(), expected.len(), "Arrays aren't the same length");
    assert_eq!(
        format!("{:?}", morse),
        format!("{:?}", expected),
        "Debug output differs"
    );
}
