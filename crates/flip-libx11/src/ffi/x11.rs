#[repr(C)]
pub(crate) struct Display {}

#[repr(C)]
pub(crate) struct Screen {}

extern "C" {
    pub(crate) fn XOpenDisplay(display_name: *mut std::os::raw::c_char) -> *mut Display;
    pub(crate) fn XCloseDisplay(display: *mut Display);
    pub(crate) fn XDefaultScreenOfDisplay(display: *mut Display) -> *mut Screen;
    pub(crate) fn XDefaultRootWindow(display: *mut Display);
}
