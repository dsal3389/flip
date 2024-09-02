extern crate pkg_config;

fn main() {
    pkg_config::Config::new()
        .atleast_version("1.8")
        .probe("x11")
        .unwrap();
}
