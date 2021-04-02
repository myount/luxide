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

use crate::device::{Lights, Luxafor, PatternType, RgbColor, SimpleColor, Target, WaveType};
use crate::protocol;
use enumflags2::BitFlags;
use log::{debug, info, trace, warn};

pub(crate) const LUXAFOR_VID: u16 = 0x04D8;
pub(crate) const LUXAFOR_PID: u16 = 0xF372;

fn lights_to_targets(lights: BitFlags<Lights>) -> Vec<Target> {
    if lights.is_empty() {
        vec![]
    } else if lights.is_all() {
        vec![Target::All]
    } else if lights.bits() == Lights::back().bits() {
        vec![Target::Back]
    } else if lights.bits() == Lights::flag().bits() {
        vec![Target::Tab]
    } else {
        lights
            .iter()
            .map(|light| match light {
                Lights::FlagBottom => Target::TabBottomLed,
                Lights::FlagMiddle => Target::TabMiddleLed,
                Lights::FlagTop => Target::TabTopLed,
                Lights::BackBottom => Target::BackBottomLed,
                Lights::BackMiddle => Target::BackMiddleLed,
                Lights::BackTop => Target::BackTopLed,
            })
            .collect()
    }
}

// We don't bother actually doing anything with the responses in any of these functions because
// the Luxafor flag (or at least, the one I have; maybe they fixed this in a later hardware
// revision) doesn't actually return the response the protocol documentation (that I was able to
// find, such as it is) says it will: we read zero bytes from it after sending a command, but we're
// _supposed to_ get a response telling us the command that's executing, or if there's one already
// executing that we need to wait to finish before sending another.
impl Luxafor {
    /// Sets one of the Luxafor "simple" colors (red, green, blue, cyan, magenta, yellow, white,
    /// or off).  This command always affects all lights.
    pub fn set_simple_color(&self, color: SimpleColor) {
        info!("sending 'set simple color' {:?}", color);

        let cmd = protocol::set_simple_color(color);
        debug!("writing to {:?}", &self);

        match &self.hid_device.write(&cmd) {
            Ok(size) => {
                trace!("wrote {} bytes: {:?}", size, cmd);

                let mut buf = Vec::<u8>::new();
                match &self.hid_device.read_timeout(&mut buf, -1) {
                    Ok(size) => {
                        trace!("read {} bytes: {:?}", size, buf);
                    }
                    Err(e) => warn!("Error reading response: {}", e),
                }
            }
            Err(e) => warn!("Error writing command: {}", e),
        }
    }

    /// Sets an arbitrary RGB color on the specified lights.
    pub fn set_rgb_color(&self, color: RgbColor, lights: BitFlags<Lights>) {
        info!("sending 'set RGB color' {:?} {:?}", color, lights);

        let targets = lights_to_targets(lights);
        debug!("mapped {:?} to targets {:?}", lights, targets);

        debug!(
            "writing command{} to {:?}",
            if targets.len() > 1 { "s" } else { "" },
            &self
        );
        targets
            .iter()
            .map(|&target| protocol::set_rgb_color(color, target))
            .for_each(|cmd| match &self.hid_device.write(&cmd) {
                Ok(size) => {
                    trace!("wrote {} bytes: {:?}", size, cmd);
                    let mut buf = Vec::<u8>::new();
                    match &self.hid_device.read_timeout(&mut buf, -1) {
                        Ok(size) => {
                            trace!("read {} bytes: {:?}", size, buf);
                        }
                        Err(e) => warn!("Error reading response: {}", e),
                    }
                }
                Err(e) => warn!("Error reading response: {}", e),
            });
    }

    /// Fades the specified lights from their current color to the specified one over the duration
    /// given by `fade_time`.
    ///
    /// `fade_time` is a `u8` on an arbitrary scale from 0 (instant) to 255 (very slow). The precise
    /// length of time each value corresponds to is determined by the hardware, and may differ
    /// depending on the starting and ending colors of the fade.
    pub fn fade_to_color(&self, color: RgbColor, lights: BitFlags<Lights>, fade_time: u8) {
        info!(
            "sending 'fade to color' {:?}, {:?}, fade_time {}",
            color, lights, fade_time
        );

        let targets = lights_to_targets(lights);
        debug!("mapped {:?} to targets {:?}", lights, targets);

        debug!(
            "writing command{} to {:?}",
            if targets.len() > 1 { "s" } else { "" },
            &self
        );
        targets
            .iter()
            .map(|&target| protocol::fade_to_color(color, target, fade_time))
            .for_each(|cmd| match &self.hid_device.write(&cmd) {
                Ok(size) => {
                    trace!("wrote {} bytes: {:?}", size, cmd);
                    let mut buf = Vec::<u8>::new();
                    match &self.hid_device.read_timeout(&mut buf, -1) {
                        Ok(size) => {
                            trace!("read {} bytes: {:?}", size, buf);
                        }
                        Err(e) => warn!("Error reading response: {}", e),
                    }
                }
                Err(e) => warn!("Error reading response: {}", e),
            });
    }

    /// Triggers a strobe effect, flashing the specified lights on and off with the specified color.
    /// `speed` specifies how rapidly the lights will flash, and is a `u8` on an arbitrary scale
    /// from 0 (very fast) to 255 (very slow).  `repeat` is a u8 specifying the number of times the
    /// lights should flash.
    pub fn strobe(&self, color: RgbColor, lights: BitFlags<Lights>, speed: u8, repeat: u8) {
        info!(
            "sending 'strobe' {:?}, {:?}, speed {}, repeat {}",
            color, lights, speed, repeat
        );

        let targets = lights_to_targets(lights);
        debug!("mapped {:?} to targets {:?}", lights, targets);

        debug!(
            "writing command{} to {:?}",
            if targets.len() > 1 { "s" } else { "" },
            &self
        );
        targets
            .iter()
            .map(|&target| protocol::strobe(color, target, speed, repeat))
            .for_each(|cmd| match &self.hid_device.write(&cmd) {
                Ok(size) => {
                    trace!("wrote {} bytes: {:?}", size, cmd);
                    let mut buf = Vec::<u8>::new();
                    match &self.hid_device.read_timeout(&mut buf, -1) {
                        Ok(size) => {
                            trace!("read {} bytes {:?}", size, buf);
                        }
                        Err(e) => warn!("Error reading response: {}", e),
                    }
                }
                Err(e) => warn!("Error reading response: {}", e),
            })
    }

    /// Triggers one of the four predefined wave effects.
    pub fn wave(&self, color: RgbColor, wave_type: WaveType, speed: u8, repeat: u8) {
        info!(
            "sending 'wave' {:?}, wave type {:?}, speed {}, repeat {}",
            color, wave_type, speed, repeat
        );

        debug!("writing command to {:?}", &self);
        let cmd = protocol::wave(color, wave_type, speed, repeat);
        match &self.hid_device.write(&cmd) {
            Ok(size) => {
                trace!("wrote {} bytes: {:?}", size, cmd);
                let mut buf = Vec::<u8>::new();
                match &self.hid_device.read_timeout(&mut buf, -1) {
                    Ok(size) => {
                        trace!("read {} bytes: {:?}", size, buf);
                    }
                    Err(e) => warn!("Error reading response: {}", e),
                }
            }
            Err(e) => warn!("Error reading response: {}", e),
        }
    }

    /// Triggers one of the handful of built-in patterns.
    pub fn pattern(&self, pattern_type: PatternType, repeat: u8) {
        info!(
            "sending 'pattern' pattern type {:?}, repeat {}",
            pattern_type, repeat
        );

        debug!("writing command to {:?}", &self);
        let cmd = protocol::pattern(pattern_type, repeat);
        match &self.hid_device.write(&cmd) {
            Ok(size) => {
                trace!("wrote {} bytes {:?}", size, cmd);
                let mut buf = Vec::<u8>::new();
                match &self.hid_device.read_timeout(&mut buf, -1) {
                    Ok(size) => {
                        trace!("read {} bytes {:?}", size, buf);
                    }
                    Err(e) => warn!("Error reading response: {}", e),
                }
            }
            Err(e) => warn!("Error reading response: {}", e),
        }
    }
}
