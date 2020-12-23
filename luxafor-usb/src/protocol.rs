/* This file is part of luxafor-usb, a Rust library for communicating with Luxafor Flags.
  Copyright © 2020 Mike Yount

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU Lesser General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU Lesser General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use crate::device::{PatternType, RgbColor, SimpleColor, Target, WaveType};

mod commands {
    pub const SIMPLE_COLOR: u8 = 0;
    pub const RGB_COLOR: u8 = 1;
    pub const FADE_TO_COLOR: u8 = 2;
    pub const STROBE: u8 = 3;
    pub const SET_WAVE: u8 = 4;
    pub const SET_PATTERN: u8 = 6;
    pub const _GET_STATUS: u8 = 0x80;
}

pub(crate) fn set_simple_color(color: SimpleColor) -> [u8; 3] {
    [0, commands::SIMPLE_COLOR, color as u8]
}

pub(crate) fn set_rgb_color(color: RgbColor, target: Target) -> [u8; 6] {
    [
        0,
        commands::RGB_COLOR,
        target as u8,
        color.0,
        color.1,
        color.2,
    ]
}

pub(crate) fn fade_to_color(color: RgbColor, target: Target, fade_time: u8) -> [u8; 7] {
    [
        0,
        commands::FADE_TO_COLOR,
        target as u8,
        color.0,
        color.1,
        color.2,
        fade_time,
    ]
}

pub(crate) fn strobe(color: RgbColor, target: Target, speed: u8, repeat: u8) -> [u8; 9] {
    [
        0,
        commands::STROBE,
        target as u8,
        color.0,
        color.1,
        color.2,
        speed,
        0,
        repeat,
    ]
}

pub(crate) fn wave(color: RgbColor, wave_type: WaveType, speed: u8, repeat: u8) -> [u8; 9] {
    [
        0,
        commands::SET_WAVE,
        wave_type as u8,
        color.0,
        color.1,
        color.2,
        0,
        repeat,
        speed,
    ]
}

pub(crate) fn pattern(pattern_type: PatternType, repeat: u8) -> [u8; 4] {
    [0, commands::SET_PATTERN, pattern_type as u8, repeat]
}

pub(crate) fn _get_status() -> [u8; 8] {
    [commands::_GET_STATUS, 0, 0, 0, 0, 0, 0, 0]
}
