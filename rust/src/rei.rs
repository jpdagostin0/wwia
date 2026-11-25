//! Roughly Enough Items integration: REI ends item tooltips (in its item list
//! and on recipe pages) with a blue "Minecraft" mod-name line. This rewrites
//! that line to "Minecraft <version>", the same way the Jade integration does.
//! Hooked by [`crate::mixin_rei::ReiEntryStackMixin`].
use crate::added;
use crate::is_null;
use crate::jvm_std::{JObject, JString, jstr, rstr, to_string};
use crate::mc::{Component, Identifier, MutableComponent};

#[jvm::interface("me.shedaniel.rei.api.client.gui.widgets.Tooltip", rename_all = "camelCase")]
impl Tooltip {
    #[jvm::method]
    pub fn entries(&self) -> *mut JList {}

    #[jvm::method]
    pub fn get_context_stack(&self) -> *mut EntryStack {}

    #[jvm::static_method(name = "entry", descriptor = "(Lnet/minecraft/network/chat/Component;)Lme/shedaniel/rei/api/client/gui/widgets/Tooltip$Entry;")]
    pub fn entry(text: *mut Component) -> *mut TooltipEntry {}
}

#[jvm::interface("me.shedaniel.rei.api.client.gui.widgets.Tooltip$Entry", rename_all = "camelCase")]
impl TooltipEntry {
    #[jvm::method]
    pub fn is_text(&self) -> bool {}

    #[jvm::method]
    pub fn get_as_text(&self) -> *mut Component {}
}

#[jvm::interface("me.shedaniel.rei.api.common.entry.EntryStack", rename_all = "camelCase")]
impl EntryStack {
    #[jvm::method]
    pub fn get_identifier(&self) -> *mut Identifier {}
}

#[jvm::interface("java.util.List")]
impl JList {
    #[jvm::method]
    pub fn size(&self) -> i32 {}

    #[jvm::method]
    pub fn get(&self, index: i32) -> *mut JObject {}

    #[jvm::method]
    pub fn set(&mut self, index: i32, value: *mut JObject) -> *mut JObject {}
}

#[jvm::interface("net.minecraft.network.chat.Component")]
impl ComponentCopy {
    #[jvm::method]
    pub fn copy(&self) -> *mut MutableComponent {}
}

#[jvm::class("net.minecraft.network.chat.MutableComponent")]
impl MutableAppend {
    #[jvm::method]
    pub fn append(&mut self, text: *mut JString) -> *mut MutableComponent {}
}

#[jvm::class("org.spongepowered.asm.mixin.injection.callback.CallbackInfoReturnable", rename_all = "camelCase")]
impl CallbackInfoReturnable {
    #[jvm::method]
    pub fn get_return_value(&self) -> *mut JObject {}
}

/// Called after `AbstractEntryStack.getTooltip` builds a tooltip.
pub fn decorate(cir: *mut CallbackInfoReturnable) {
    unsafe {
        let tooltip = (*cir).get_return_value() as *mut Tooltip;
        if is_null!(tooltip) {
            return;
        }
        let stack = (*tooltip).get_context_stack();
        if is_null!(stack) {
            return;
        }
        let id = (*stack).get_identifier();
        if is_null!(id) {
            return;
        }
        let Some(when) = added::item(&to_string(id as *mut JObject)) else {
            return;
        };
        let label = added::label(when);
        let Some(suffix) = label.strip_prefix("Minecraft").filter(|s| !s.is_empty()) else {
            return; // older than the cutoff: REI's line already says "Minecraft"
        };

        // Rewrite the last "Minecraft" text line, keeping REI's style.
        let entries = &mut *(*tooltip).entries();
        let mut index = entries.size() - 1;
        while index >= 0 {
            let entry = entries.get(index) as *mut TooltipEntry;
            if (*entry).is_text() {
                let text = (*entry).get_as_text();
                if rstr((*text).get_string()) == "Minecraft" {
                    let copy = (*(text as *mut ComponentCopy)).copy();
                    (*(copy as *mut MutableAppend)).append(jstr(suffix));
                    entries.set(index, Tooltip::entry(copy as *mut Component) as *mut JObject);
                    return;
                }
            }
            index -= 1;
        }
    }
}
