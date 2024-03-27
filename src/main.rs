mod app;
mod errors;
mod tui;

use color_eyre::eyre::Result;

fn main() -> Result<()> {
    errors::install_hooks()?;
    let terminal = tui::init()?;

    tui::restore()
}
