//! Mixin into Roughly Enough Items, from the optional `wwia.rei.mixins.json`
//! config (skipped when REI isn't installed).
use crate::rei::CallbackInfoReturnable;

#[jvm::annotation("org.spongepowered.asm.mixin.Mixin",
    targets = ["me.shedaniel.rei.impl.common.entry.AbstractEntryStack"], remap = false,
    retention = "class")]
pub struct ReiEntryStackMixin;

impl ReiEntryStackMixin {
    #[jvm::annotation("org.spongepowered.asm.mixin.injection.Inject",
        method = ["getTooltip(Lme/shedaniel/rei/api/client/gui/widgets/TooltipContext;Z)Lme/shedaniel/rei/api/client/gui/widgets/Tooltip;"],
        at = [annotation("org.spongepowered.asm.mixin.injection.At", value = "RETURN")],
        remap = false)]
    pub fn wwia_get_tooltip(
        &mut self,
        _context: *mut ReiTooltipContext,
        _append_mod_name: bool,
        cir: *mut CallbackInfoReturnable,
    ) {
        crate::rei::decorate(cir);
    }
}

#[jvm::interface("me.shedaniel.rei.api.client.gui.widgets.TooltipContext")]
pub struct ReiTooltipContext;
