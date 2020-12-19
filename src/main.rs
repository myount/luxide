#![allow(unused_imports)]
use luxafor_usb::device::{Lights, Luxafor, RgbColor, SimpleColor};

fn main() {
    let device = Luxafor::new().unwrap();
    println!("{:?}", device);
    device.set_rgb_color(RgbColor::off(), Lights::all());
}
