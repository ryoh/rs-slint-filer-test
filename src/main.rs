// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Result;
use log::*;
use rfd::FileDialog;
use slint::SharedString;
use std::error::Error;
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

slint::include_modules!();

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryKind {
    File,
    Directory,
    Symlink,
    Other,
}

#[derive(Debug, Clone)]
pub struct FsEntry {
    pub path: PathBuf,
    pub metadata: fs::Metadata,
    pub file_name: OsString,
    pub kind: EntryKind,
}

pub struct EntryRow {
    pub name: SharedString,
    pub kind: SharedString,
    pub size: SharedString,
    pub modified: SharedString,

    pub sort_size: u64,
    pub sort_modified: u64,
    pub is_directory: bool,
}

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
        for entry in WalkDir::new(path.unwrap())
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
            .skip(1)
        {
            info!("Found file: {:?}", entry.path());
            let metadata = match fs::metadata(entry.path()) {
                Ok(metadata) => metadata,
                Err(e) => {
                    error!("Failed to get metadata for file {:?}: {}", entry.path(), e);
                    continue;
                }
            };

            let fsentry = FsEntry {
                path: entry.path().to_path_buf(),
                metadata,
                file_name: entry.file_name().to_os_string(),
                kind: match entry.file_type() {
                    t if t.is_file() => EntryKind::File,
                    t if t.is_dir() => EntryKind::Directory,
                    t if t.is_symlink() => EntryKind::Symlink,
                    _ => EntryKind::Other,
                },
            };

            info!("FsEntry: {:?}", fsentry);
        }
    });

    // Show the UI
    ui.run()?;

    Ok(())
}
