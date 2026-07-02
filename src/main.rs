// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Result;
use log::*;
use rfd::FileDialog;
use std::error::Error;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the logger (e.g. for Slint warnings)
    env_logger::init();

    // Load the UI from the .slint file
    let ui = AppWindow::new()?;

    ui.on_open_file_dialog(move || {
        info!("Clicked Opening file dialog...");
        let path = FileDialog::new()
            .set_directory(std::env::home_dir().unwrap())
            .pick_folder();

        info!("Selected folder: {:?}", path);
    });

    // Show the UI
    ui.run()?;

    Ok(())
}
