mod app;
mod nordvpn;
mod types;
mod ui;

use app::App;
use color_eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app = App::new()?;
    let result = app.run(terminal);
    ratatui::restore();
    result
}
