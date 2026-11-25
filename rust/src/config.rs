//! Settings, persisted to `config/wwia.properties`.
use crate::{data, mc};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

/// Index into [`data::VERSIONS`]: releases older than this display as plain
/// "Minecraft". Index 0 is 1.0, the earliest allowed cutoff.
static CUTOFF: AtomicUsize = AtomicUsize::new(0);
/// Draw the HUD line when Jade is not installed.
static SHOW_HUD: AtomicBool = AtomicBool::new(true);

pub const DEFAULT_CUTOFF: usize = 0;

pub fn cutoff() -> usize {
    CUTOFF.load(Ordering::Relaxed)
}

pub fn cutoff_name() -> &'static str {
    data::VERSIONS.get(cutoff()).copied().unwrap_or("1.0")
}

pub fn set_cutoff(index: usize) {
    CUTOFF.store(index.min(data::VERSIONS.len().saturating_sub(1)), Ordering::Relaxed);
}

pub fn show_hud() -> bool {
    SHOW_HUD.load(Ordering::Relaxed)
}

pub fn set_show_hud(value: bool) {
    SHOW_HUD.store(value, Ordering::Relaxed);
}

fn path() -> String {
    format!("{}/wwia.properties", mc::config_dir())
}

pub fn load() {
    let Ok(text) = std::fs::read_to_string(path()) else {
        save();
        return;
    };
    for line in text.lines() {
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        match key.trim() {
            "cutoff" => {
                let value = value.trim();
                if let Some(index) = data::VERSIONS.iter().position(|v| *v == value) {
                    set_cutoff(index);
                }
            }
            "show_hud_without_jade" => set_show_hud(value.trim() != "false"),
            _ => {}
        }
    }
}

pub fn save() {
    let text = format!(
        "# When Was It Added\n\
         # Things added before this release just say \"Minecraft\" (earliest: 1.0).\n\
         cutoff={}\n\
         # Show the added-in line on the HUD when Jade isn't installed.\n\
         show_hud_without_jade={}\n",
        cutoff_name(),
        show_hud()
    );
    if let Err(error) = std::fs::write(path(), text) {
        mc::log(&format!("[wwia] could not save config: {error}"));
    }
}
