#[repr(C)]
pub(crate) struct Display {}

extern "C" {
    pub(crate) fn XOpenDisplay(display_name: *mut std::os::raw::c_char) -> *mut Display;
    pub(crate) fn XCloseDisplay(display: *mut Display);
}
