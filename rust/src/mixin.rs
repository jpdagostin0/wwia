//! Mixin classes. Each struct here compiles to a JVM class carrying
//! `@Mixin`, and each annotated method to an injector handler. They are listed
//! in `wwia.mixins.json`; Rust code must never name them at runtime.
//!
//! The handler bodies only forward to ordinary Rust functions, which are
//! emitted into the separate `wwia.mixin` module class.
use crate::mc::{CallbackInfo, DeltaTracker, GuiGraphicsExtractor};

/// Draws the "added in" line on the HUD (only used when Jade is absent).
#[jvm::annotation("org.spongepowered.asm.mixin.Mixin",
    value = [class("net.minecraft.client.gui.Hud")], retention = "class")]
pub struct HudMixin;

impl HudMixin {
    #[jvm::annotation("org.spongepowered.asm.mixin.injection.Inject",
        method = ["extractRenderState"],
        at = [annotation("org.spongepowered.asm.mixin.injection.At", value = "TAIL")])]
    pub fn wwia_extract_render_state(
        &mut self,
        graphics: *mut GuiGraphicsExtractor,
        _delta: *mut DeltaTracker,
        _ci: *mut CallbackInfo,
    ) {
        crate::hud::extract(graphics);
    }
}
