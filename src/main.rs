mod app;
mod nordvpn;
mod types;
mod ui;

use app::App;
use color_eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;

    // Try to create the app - if it fails, show a helpful error
    let app = match App::new() {
        Ok(app) => app,
        Err(e) => {
            eprintln!("Error: Failed to initialize nordvpn-tui\n");
            eprintln!("This application requires the NordVPN service to be running.");
            eprintln!("\nPlease ensure:");
            eprintln!("  1. NordVPN is installed: https://nordvpn.com/download/linux/");
            eprintln!("  2. The nordvpn daemon is running");
            eprintln!("  3. You are logged in: nordvpn login");
            eprintln!("\nOriginal error: {}", e);
            std::process::exit(1);
        }
    };

    let terminal = ratatui::init();
    let result = app.run(terminal);
    ratatui::restore();
    result
}
