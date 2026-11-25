//! What the player is looking at, and the HUD line drawn when Jade is absent.
use crate::added::{self, Added};
use crate::is_null;
use crate::jvm_std::jstr;
use crate::mc::{self, BlockHitResult, EntityHitResult, GuiGraphicsExtractor, Minecraft};
use crate::{config, mc::component_string};
use std::sync::OnceLock;

/// A looked-at vanilla thing and when it was added.
pub struct Target {
    pub name: String,
    pub added: Added,
}

/// Resolves the crosshair target (block or entity) to its added-in version.
pub fn crosshair_target() -> Option<Target> {
    unsafe {
        let minecraft = &*Minecraft::get_instance();
        let hit = minecraft.hit_result();
        let level = minecraft.level();
        if is_null!(hit) || is_null!(level) {
            return None;
        }
        let kind = (*((*hit).get_type() as *mut mc::JEnum)).ordinal();
        if kind == mc::HIT_BLOCK {
            let pos = (*(hit as *mut BlockHitResult)).get_block_pos();
            let block = (*(*level).get_block_state(pos)).get_block();
            let added = added::block(&mc::block_id(block))?;
            let name = component_string((*block).get_name() as *mut mc::Component);
            Some(Target { name, added })
        } else if kind == mc::HIT_ENTITY {
            let entity = (*(hit as *mut EntityHitResult)).get_entity();
            let added = added::entity(&mc::entity_id(entity))?;
            let name = component_string((*entity).get_name());
            Some(Target { name, added })
        } else {
            None
        }
    }
}

fn jade_installed() -> bool {
    static JADE: OnceLock<bool> = OnceLock::new();
    *JADE.get_or_init(|| mc::is_mod_loaded("jade"))
}

const WHITE: i32 = 0xFFFF_FFFFu32 as i32;
/// Jade's default mod-name colour (blue, like `ChatFormatting.BLUE`).
const BLUE: i32 = 0xFF55_55FFu32 as i32;
const BACKGROUND: i32 = 0xA010_0010u32 as i32;

/// Called from [`crate::mixin::HudMixin`] after the vanilla HUD is extracted.
pub fn extract(graphics: *mut GuiGraphicsExtractor) {
    if jade_installed() || !config::show_hud() {
        return;
    }
    let Some(target) = crosshair_target() else {
        return;
    };
    unsafe {
        let minecraft = &*Minecraft::get_instance();
        if !is_null!((*minecraft.gui()).screen()) {
            return;
        }
        let font = minecraft.font();
        let graphics = &mut *graphics;
        let name = jstr(&target.name);
        let label = jstr(&added::label(target.added));
        let width = (*font).width(name).max((*font).width(label));
        let line = (*font).line_height();
        let x = graphics.gui_width() / 2;
        let top = 4;
        graphics.fill(x - width / 2 - 4, top - 3, x + width / 2 + 4, top + line * 2 + 3, BACKGROUND);
        graphics.text(font, name, x - (*font).width(name) / 2, top, WHITE, true);
        graphics.text(font, label, x - (*font).width(label) / 2, top + line + 1, BLUE, true);
    }
}
