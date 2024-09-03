use std::ffi::{c_char, CString};

use crate::{ffi::x11 as ffix11, X11Error};

#[derive(Debug)]
pub struct Screen {
    screen_ptr: *mut ffix11::Screen,
}

#[derive(Debug)]
pub struct Display {
    display_ptr: *mut ffix11::Display,
}

impl Display {
    pub fn new<T>(display_name: Option<T>) -> Result<Display, X11Error>
    where
        T: Into<Vec<u8>>,
    {
        let name_ptr = match display_name {
            Some(s) => CString::new(s)
                .unwrap()
                .as_bytes_with_nul()
                .to_vec()
                .as_mut_ptr(),
            None => std::ptr::null_mut::<u8>(),
        };
        unsafe {
            let display_ptr = ffix11::XOpenDisplay(name_ptr as *mut c_char);
            if display_ptr.is_null() {
                Err(X11Error::NULLError)
            } else {
                Ok(Self { display_ptr })
            }
        }
    }

    /// returns the default screen for the current display
    pub fn default_screen(&self) -> Screen {
        Screen::from(self)
    }
}

/// returns the default Screen for the given display
/// by calling `DefaultScreenOfDisplay`
impl From<&Display> for Screen {
    fn from(value: &Display) -> Self {
        unsafe {
            Self {
                screen_ptr: ffix11::XDefaultScreenOfDisplay(value.display_ptr),
            }
        }
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        unsafe {
            ffix11::XCloseDisplay(self.display_ptr);
        }
    }
}
