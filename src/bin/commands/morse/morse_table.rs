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

use crate::commands::morse::morse_table::morse_table::{letters, numbers, punctuation};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Morsel {
    Dot,
    Dash,
    Gap(GapType),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum GapType {
    Symbol,
    Letter,
    Word,
}

pub(in crate::commands::morse) mod morse_table {
    pub mod letters {
        use crate::commands::morse::morse_table::GapType::Symbol;
        use crate::commands::morse::morse_table::Morsel;
        use crate::commands::morse::morse_table::Morsel::{Dash, Dot, Gap};

        pub const A: &[Morsel] = &[Dot, Gap(Symbol), Dash];
        pub const B: &[Morsel] = &[Dash, Gap(Symbol), Dot, Gap(Symbol), Dot, Gap(Symbol), Dot];
        pub const C: &[Morsel] = &[Dash, Gap(Symbol), Dot, Gap(Symbol), Dash, Gap(Symbol), Dot];
        pub const D: &[Morsel] = &[Dash, Gap(Symbol), Dot, Gap(Symbol), Dot];
        pub const E: &[Morsel] = &[Dot];
        pub const F: &[Morsel] = &[Dot, Gap(Symbol), Dot, Gap(Symbol), Dash, Gap(Symbol), Dot];
        pub const G: &[Morsel] = &[Dash, Gap(Symbol), Dash, Gap(Symbol), Dot];
        pub const H: &[Morsel] = &[Dot, Gap(Symbol), Dot, Gap(Symbol), Dot, Gap(Symbol), Dot];
        pub const I: &[Morsel] = &[Dot, Gap(Symbol), Dot];
        pub const J: &[Morsel] = &[Dot, Gap(Symbol), Dash, Gap(Symbol), Dash, Gap(Symbol), Dash];
        pub const K: &[Morsel] = &[Dash, Gap(Symbol), Dot, Gap(Symbol), Dash];
        pub const L: &[Morsel] = &[Dot, Gap(Symbol), Dash, Gap(Symbol), Dot, Gap(Symbol), Dot];
        pub const M: &[Morsel] = &[Dash, Gap(Symbol), Dash];
        pub const N: &[Morsel] = &[Dash, Gap(Symbol), Dot];
        pub const O: &[Morsel] = &[Dash, Gap(Symbol), Dash, Gap(Symbol), Dash];
        pub const P: &[Morsel] = &[Dot, Gap(Symbol), Dash, Gap(Symbol), Dash, Gap(Symbol), Dot];
        pub const Q: &[Morsel] = &[Dash, Gap(Symbol), Dash, Gap(Symbol), Dot, Gap(Symbol), Dash];
        pub const R: &[Morsel] = &[Dot, Gap(Symbol), Dash, Gap(Symbol), Dot];
        pub const S: &[Morsel] = &[Dot, Gap(Symbol), Dot, Gap(Symbol), Dot];
        pub const T: &[Morsel] = &[Dash];
        pub const U: &[Morsel] = &[Dot, Gap(Symbol), Dot, Gap(Symbol), Dash];
        pub const V: &[Morsel] = &[Dot, Gap(Symbol), Dot, Gap(Symbol), Dot, Gap(Symbol), Dash];
        pub const W: &[Morsel] = &[Dot, Gap(Symbol), Dash, Gap(Symbol), Dash];
        pub const X: &[Morsel] = &[Dash, Gap(Symbol), Dot, Gap(Symbol), Dot, Gap(Symbol), Dash];
        pub const Y: &[Morsel] = &[Dash, Gap(Symbol), Dot, Gap(Symbol), Dash, Gap(Symbol), Dash];
        pub const Z: &[Morsel] = &[Dash, Gap(Symbol), Dash, Gap(Symbol), Dot, Gap(Symbol), Dot];
    }

    pub mod numbers {
        use crate::commands::morse::morse_table::GapType::Symbol;
        use crate::commands::morse::morse_table::Morsel;
        use crate::commands::morse::morse_table::Morsel::{Dash, Dot, Gap};

        pub const ZERO: &[Morsel] = &[
            Dash,
            Gap(Symbol),
            Dash,
            Gap(Symbol),
            Dash,
            Gap(Symbol),
            Dash,
            Gap(Symbol),
            Dash,
        ];
        pub const ONE: &[Morsel] = &[
            Dot,
            Gap(Symbol),
            Dash,
            Gap(Symbol),
            Dash,
            Gap(Symbol),
            Dash,
            Gap(Symbol),
            Dash,
        ];
        pub const TWO: &[Morsel] = &[
            Dot,
            Gap(Symbol),
            Dot,
            Gap(Symbol),
            Dash,
            Gap(Symbol),
            Dash,
            Gap(Symbol),
            Dash,
        ];
        pub const THREE: &[Morsel] = &[
            Dot,
            Gap(Symbol),
            Dot,
            Gap(Symbol),
            Dot,
            Gap(Symbol),
            Dash,
            Gap(Symbol),
            Dash,
        ];
        pub const FOUR: &[Morsel] = &[
            Dot,
            Gap(Symbol),
            Dot,
            Gap(Symbol),
            Dot,
            Gap(Symbol),
            Dot,
            Gap(Symbol),
            Dash,
        ];
        pub const FIVE: &[Morsel] = &[
            Dot,
            Gap(Symbol),
            Dot,
            Gap(Symbol),
            Dot,
            Gap(Symbol),
            Dot,
            Gap(Symbol),
            Dot,
        ];
        pub const SIX: &[Morsel] = &[
            Dash,
            Gap(Symbol),
            Dot,
            Gap(Symbol),
            Dot,
            Gap(Symbol),
            Dot,
            Gap(Symbol),
            Dot,
        ];
        pub const SEVEN: &[Morsel] = &[
            Dash,
            Gap(Symbol),
            Dash,
            Gap(Symbol),
            Dot,
            Gap(Symbol),
            Dot,
            Gap(Symbol),
            Dot,
        ];
        pub const EIGHT: &[Morsel] = &[
            Dash,
            Gap(Symbol),
            Dash,
            Gap(Symbol),
            Dash,
            Gap(Symbol),
            Dot,
            Gap(Symbol),
            Dot,
        ];
        pub const NINE: &[Morsel] = &[
            Dash,
            Gap(Symbol),
            Dash,
            Gap(Symbol),
            Dash,
            Gap(Symbol),
            Dash,
            Gap(Symbol),
            Dot,
        ];
    }

    pub mod punctuation {
        use crate::commands::morse::morse_table::GapType::Symbol;
        use crate::commands::morse::morse_table::Morsel;
        use crate::commands::morse::morse_table::Morsel::{Dash, Dot, Gap};

        pub const PERIOD: &[Morsel] = &[
            Dot,
            Gap(Symbol),
            Dash,
            Gap(Symbol),
            Dot,
            Gap(Symbol),
            Dash,
            Gap(Symbol),
            Dot,
            Gap(Symbol),
            Dash,
        ];
        pub const COMMA: &[Morsel] = &[
            Dash,
            Gap(Symbol),
            Dash,
            Gap(Symbol),
            Dot,
            Gap(Symbol),
            Dot,
            Gap(Symbol),
            Dash,
            Gap(Symbol),
            Dash,
        ];
        pub const QUESTION: &[Morsel] = &[
            Dot,
            Gap(Symbol),
            Dot,
            Gap(Symbol),
            Dash,
            Gap(Symbol),
            Dash,
            Gap(Symbol),
            Dot,
            Gap(Symbol),
            Dot,
        ];
        pub const BACKSLASH: &[Morsel] = &[
            Dot,
            Gap(Symbol),
            Dash,
            Gap(Symbol),
            Dash,
            Gap(Symbol),
            Dash,
            Gap(Symbol),
            Dash,
            Gap(Symbol),
            Dot,
        ];
        pub const EXCLAMATION: &[Morsel] = &[
            Dash,
            Gap(Symbol),
            Dot,
            Gap(Symbol),
            Dash,
            Gap(Symbol),
            Dot,
            Gap(Symbol),
            Dash,
            Gap(Symbol),
            Dash,
        ]; // &
        pub const LEFT_PAREN: &[Morsel] = &[
            Dash,
            Gap(Symbol),
            Dot,
            Gap(Symbol),
            Dash,
            Gap(Symbol),
            Dash,
            Gap(Symbol),
            Dot,
        ];
        pub const RIGHT_PAREN: &[Morsel] = &[
            Dash,
            Gap(Symbol),
            Dot,
            Gap(Symbol),
            Dash,
            Gap(Symbol),
            Dash,
            Gap(Symbol),
            Dot,
            Gap(Symbol),
            Dash,
        ];
        pub const AMPERSAND: &[Morsel] = &[
            Dot,
            Gap(Symbol),
            Dash,
            Gap(Symbol),
            Dot,
            Gap(Symbol),
            Dot,
            Gap(Symbol),
            Dot,
        ];
        pub const COLON: &[Morsel] = &[
            Dash,
            Gap(Symbol),
            Dash,
            Gap(Symbol),
            Dash,
            Gap(Symbol),
            Dot,
            Gap(Symbol),
            Dot,
            Gap(Symbol),
            Dot,
        ];
        pub const SEMICOLON: &[Morsel] = &[
            Dash,
            Gap(Symbol),
            Dot,
            Gap(Symbol),
            Dash,
            Gap(Symbol),
            Dot,
            Gap(Symbol),
            Dash,
            Gap(Symbol),
            Dot,
        ]; // &
        pub const EQUALS: &[Morsel] = &[
            Dash,
            Gap(Symbol),
            Dot,
            Gap(Symbol),
            Dot,
            Gap(Symbol),
            Dot,
            Gap(Symbol),
            Dash,
        ];
        pub const PLUS: &[Morsel] = &[
            Dot,
            Gap(Symbol),
            Dash,
            Gap(Symbol),
            Dot,
            Gap(Symbol),
            Dash,
            Gap(Symbol),
            Dot,
        ];
        pub const MINUS: &[Morsel] = &[
            Dash,
            Gap(Symbol),
            Dot,
            Gap(Symbol),
            Dot,
            Gap(Symbol),
            Dot,
            Gap(Symbol),
            Dot,
            Gap(Symbol),
            Dash,
        ];
        pub const UNDERSCORE: &[Morsel] = &[
            Dot,
            Gap(Symbol),
            Dot,
            Gap(Symbol),
            Dash,
            Gap(Symbol),
            Dash,
            Gap(Symbol),
            Dot,
            Gap(Symbol),
            Dash,
        ]; // &
        pub const DOUBLE_QUOTE: &[Morsel] = &[
            Dot,
            Gap(Symbol),
            Dash,
            Gap(Symbol),
            Dot,
            Gap(Symbol),
            Dot,
            Gap(Symbol),
            Dash,
            Gap(Symbol),
            Dot,
        ];
        pub const DOLLAR: &[Morsel] = &[
            Dot,
            Gap(Symbol),
            Dot,
            Gap(Symbol),
            Dot,
            Gap(Symbol),
            Dash,
            Gap(Symbol),
            Dot,
            Gap(Symbol),
            Dot,
            Gap(Symbol),
            Dash,
        ]; // &
        pub const AT_SIGN: &[Morsel] = &[
            Dot,
            Gap(Symbol),
            Dash,
            Gap(Symbol),
            Dash,
            Gap(Symbol),
            Dot,
            Gap(Symbol),
            Dash,
            Gap(Symbol),
            Dot,
        ];
    }
}

pub const fn char_to_morse<'a>(ch: char) -> Result<&'a [Morsel], ()> {
    match ch {
        // ---- letters ----
        'A' => Ok(letters::A),
        'B' => Ok(letters::B),
        'C' => Ok(letters::C),
        'D' => Ok(letters::D),
        'E' => Ok(letters::E),
        'F' => Ok(letters::F),
        'G' => Ok(letters::G),
        'H' => Ok(letters::H),
        'I' => Ok(letters::I),
        'J' => Ok(letters::J),
        'K' => Ok(letters::K),
        'L' => Ok(letters::L),
        'M' => Ok(letters::M),
        'N' => Ok(letters::N),
        'O' => Ok(letters::O),
        'P' => Ok(letters::P),
        'Q' => Ok(letters::Q),
        'R' => Ok(letters::R),
        'S' => Ok(letters::S),
        'T' => Ok(letters::T),
        'U' => Ok(letters::U),
        'V' => Ok(letters::V),
        'W' => Ok(letters::W),
        'X' => Ok(letters::X),
        'Y' => Ok(letters::Y),
        'Z' => Ok(letters::Z),
        // ---- numbers ----
        '0' => Ok(numbers::ZERO),
        '1' => Ok(numbers::ONE),
        '2' => Ok(numbers::TWO),
        '3' => Ok(numbers::THREE),
        '4' => Ok(numbers::FOUR),
        '5' => Ok(numbers::FIVE),
        '6' => Ok(numbers::SIX),
        '7' => Ok(numbers::SEVEN),
        '8' => Ok(numbers::EIGHT),
        '9' => Ok(numbers::NINE),
        // -- punctuation --
        '.' => Ok(punctuation::PERIOD),
        ',' => Ok(punctuation::COMMA),
        '?' => Ok(punctuation::QUESTION),
        '\\' => Ok(punctuation::BACKSLASH),
        '!' => Ok(punctuation::EXCLAMATION),
        '(' => Ok(punctuation::LEFT_PAREN),
        ')' => Ok(punctuation::RIGHT_PAREN),
        '&' => Ok(punctuation::AMPERSAND),
        ':' => Ok(punctuation::COLON),
        ';' => Ok(punctuation::SEMICOLON),
        '=' => Ok(punctuation::EQUALS),
        '+' => Ok(punctuation::PLUS),
        '-' => Ok(punctuation::MINUS),
        '_' => Ok(punctuation::UNDERSCORE),
        '"' => Ok(punctuation::DOUBLE_QUOTE),
        '$' => Ok(punctuation::DOLLAR),
        '@' => Ok(punctuation::AT_SIGN),
        _ => Err(()),
    }
}
