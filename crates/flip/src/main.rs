use anyhow::Context;
use flip_libx11::x11::{Display, Screen};

fn main() -> anyhow::Result<()> {
    let display = Display::new::<&str>(None).context("couldn't get display")?;
    let screen = Screen::from(&display);
    println!("display {display:?} screen {screen:?}");
    Ok(())
}
