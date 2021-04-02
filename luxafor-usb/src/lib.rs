/* This file is part of luxafor-usb, a Rust library for communicating with Luxafor Flags.
  Copyright © 2020 Mike Yount

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU Lesser General Public License as published by
   the Free Software Foundation, version 3.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU Lesser General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

pub mod device;
mod protocol;
mod usb;

pub struct LuxaforError {
    message: &'static str,
}

impl Error for LuxaforError {}
impl Display for LuxaforError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}
impl Debug for LuxaforError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}
