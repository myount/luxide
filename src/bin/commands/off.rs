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

use clap::{App, SubCommand};
use luxafor_usb::device::{Luxafor, SimpleColor};

pub struct Off {}

impl Off {
    pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("off").about("Shorthand for `luxide color off`")
    }

    pub fn exec() -> Result<(), String> {
        let luxafor = Luxafor::new()?;

        Ok(luxafor.set_simple_color(SimpleColor::Off))
    }
}
