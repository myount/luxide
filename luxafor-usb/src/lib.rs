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
