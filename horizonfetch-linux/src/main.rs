// HorizonFetch Linux Edition
// AnmiTaliDev <anmitalidev@nuros.org>

mod config;
mod display;
mod system;

use config::Config;
use display::Display;
use system::SystemInfo;

fn main() -> std::io::Result<()> {
    // Load configuration
    let config = Config::load_default();

    // Gather system information
    let info = SystemInfo::gather();

    // Display everything
    let display = Display::new(config, info);
    display.render()?;

    Ok(())
}
