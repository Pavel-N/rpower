mod error;
mod menu_button;
mod prelude;
mod rpower;
use crate::prelude::Result;
use error::Error;
use iced::Application;

fn main() -> Result<()> {
    // NOTE: My machine
    std::env::set_var("WGPU_BACKEND", "primary");

    // Get home directory
    let home_dir = home::home_dir()
        .ok_or(Error::NoHome)?
        .to_string_lossy()
        .to_string();

    // Load config
    let config: rpower::RPowerConfig = config::Config::builder()
        .add_source(config::File::with_name(
            format!("{home_dir}/.config/rpower/config.toml").as_str(),
        ))
        .build()?
        .try_deserialize()?;

    // Run RPower
    rpower::RPower::run(iced::Settings {
        window: iced::window::Settings {
            size: (config.width, config.height),
            resizable: false,
            decorations: false,
            // always_on_top: true,
            ..Default::default()
        },
        antialiasing: false,
        flags: (config, home_dir),
        ..Default::default()
    })?;

    Ok(())
}
