//! Version lookup and the "Minecraft <version>" label.
use crate::{config, data};
use std::collections::HashMap;
use std::sync::OnceLock;

/// When a vanilla block or entity type was added.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Added {
    /// Classic, Indev, Infdev, Alpha or Beta: before Java Edition 1.0.
    PreRelease,
    /// Index into [`data::VERSIONS`].
    Release(usize),
}

struct Tables {
    blocks: HashMap<&'static str, u16>,
    entities: HashMap<&'static str, u16>,
    items: HashMap<&'static str, u16>,
}

fn parse(table: &'static str) -> HashMap<&'static str, u16> {
    table
        .lines()
        .filter_map(|line| {
            let (id, code) = line.split_once(' ')?;
            Some((id, code.parse().ok()?))
        })
        .collect()
}

fn tables() -> &'static Tables {
    static TABLES: OnceLock<Tables> = OnceLock::new();
    TABLES.get_or_init(|| Tables {
        blocks: parse(data::BLOCKS),
        entities: parse(data::ENTITIES),
        items: parse(data::ITEMS),
    })
}

fn decode(code: u16) -> Added {
    if code == data::PRE_RELEASE {
        Added::PreRelease
    } else {
        Added::Release(code as usize)
    }
}

/// Looks up a block by registry id (`minecraft:stone`). Modded ids return None.
pub fn block(id: &str) -> Option<Added> {
    let path = id.strip_prefix("minecraft:")?;
    tables().blocks.get(path).copied().map(decode)
}

/// Looks up an entity type by registry id (`minecraft:creeper`).
pub fn entity(id: &str) -> Option<Added> {
    let path = id.strip_prefix("minecraft:")?;
    tables().entities.get(path).copied().map(decode)
}

/// Looks up an item (`minecraft:diamond_sword`). Falls back to the block of
/// the same id, then to the mob for a spawn egg, for ids the item table lacks.
pub fn item(id: &str) -> Option<Added> {
    let path = id.strip_prefix("minecraft:")?;
    let tables = tables();
    if let Some(code) = tables.items.get(path) {
        return Some(decode(*code));
    }
    if let Some(code) = tables.blocks.get(path) {
        return Some(decode(*code));
    }
    let mob = path.strip_suffix("_spawn_egg")?;
    tables.entities.get(mob).copied().map(decode)
}

/// Warms the lookup tables (called at startup, off the render path).
pub fn init() {
    let _ = tables();
}

/// "Minecraft 1.16", or just "Minecraft" for anything older than the
/// configured cutoff (pre-1.0 content is always plain "Minecraft").
pub fn label(added: Added) -> String {
    label_with_cutoff(added, config::cutoff())
}

pub fn label_with_cutoff(added: Added, cutoff: usize) -> String {
    match added {
        Added::Release(index) if index >= cutoff => match data::VERSIONS.get(index) {
            Some(version) => format!("Minecraft {version}"),
            None => "Minecraft".to_owned(),
        },
        _ => "Minecraft".to_owned(),
    }
}
