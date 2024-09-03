mod ffi;
pub mod x11;

#[derive(Debug)]
pub enum X11Error {
    NULLError,
}

impl std::fmt::Display for X11Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NULLError => write!(f, "NULL"),
        }
    }
}

impl std::error::Error for X11Error {}
