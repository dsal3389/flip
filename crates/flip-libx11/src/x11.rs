use core::fmt;

use crate::ffi::x11::{self as ffix11, XOpenDisplay};

#[derive(Debug)]
pub enum X11Error {
    Info(String),
}

impl fmt::Display for X11Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Info(m) => write!(f, "{}", m),
        }
    }
}

impl std::error::Error for X11Error {}

pub struct Display {
    display_ptr: ffix11::Display,
}

impl Display {
    pub fn new(display_name: Option<&str>) -> Result<Display, X11Error> {
        todo!()
    }
}
