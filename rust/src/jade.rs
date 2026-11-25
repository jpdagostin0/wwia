//! Jade integration: puts the version on Jade's mod-name line, so vanilla
//! things read "Minecraft 1.16" instead of just "Minecraft".
use crate::added::{self, Added};
use crate::jvm_std::{JClass, JObject, class};
use crate::mc::{self, Block, Entity, Identifier};

// ---- Jade API bindings ----------------------------------------------------

#[jvm::interface("snownee.jade.api.IWailaPlugin", rename_all = "camelCase")]
pub trait IWailaPlugin {
    fn register_client(&mut self, registration: *mut IWailaClientRegistration);
}

#[jvm::interface("snownee.jade.api.IWailaClientRegistration", rename_all = "camelCase")]
impl IWailaClientRegistration {
    #[jvm::method]
    pub fn register_block_component(&mut self, provider: &dyn IComponentProvider, block: *mut JClass) {}

    #[jvm::method]
    pub fn register_entity_component(&mut self, provider: &dyn IComponentProvider, entity: *mut JClass) {}
}

/// `IComponentProvider<T extends Accessor<?>>`, erased; covers the inherited
/// `IJadeProvider` methods too.
#[jvm::interface("snownee.jade.api.IComponentProvider", rename_all = "camelCase")]
pub trait IComponentProvider {
    fn get_uid(&self) -> *mut Identifier;

    fn get_default_priority(&self) -> i32;

    fn append_tooltip(&self, tooltip: *mut ITooltip, accessor: *mut Accessor, config: *mut IPluginConfig);
}

#[jvm::interface("snownee.jade.api.IBlockComponentProvider")]
pub trait IBlockComponentProvider: IComponentProvider {}

#[jvm::interface("snownee.jade.api.IEntityComponentProvider")]
pub trait IEntityComponentProvider: IComponentProvider {}

#[jvm::interface("snownee.jade.api.Accessor")]
pub struct Accessor;

#[jvm::interface("snownee.jade.api.config.IPluginConfig")]
pub struct IPluginConfig;

#[jvm::interface("snownee.jade.api.BlockAccessor", rename_all = "camelCase")]
impl BlockAccessor {
    #[jvm::method]
    pub fn get_block(&self) -> *mut Block {}
}

#[jvm::interface("snownee.jade.api.EntityAccessor", rename_all = "camelCase")]
impl EntityAccessor {
    #[jvm::method]
    pub fn get_entity(&self) -> *mut Entity {}
}

#[jvm::interface("net.minecraft.client.gui.layouts.LayoutElement")]
pub struct LayoutElement;

#[jvm::interface("snownee.jade.api.ITooltip")]
impl ITooltip {
    #[jvm::method]
    pub fn remove(&mut self, tag: *mut Identifier) -> bool {}

    #[jvm::method("add")]
    pub fn add_element(&mut self, element: *mut LayoutElement) {}
}

#[jvm::class("snownee.jade.api.ui.TextElement")]
pub struct TextElement;

#[jvm::interface("snownee.jade.api.theme.IThemeHelper", rename_all = "camelCase")]
impl IThemeHelper {
    #[jvm::static_method]
    pub fn get() -> *mut Self {}

    #[jvm::method]
    pub fn mod_name_element(&self, name: *mut JObject) -> *mut TextElement {}
}

#[jvm::class("snownee.jade.api.JadeIds")]
impl JadeIds {
    #[jvm::static_field(name = "CORE_MOD_NAME")]
    pub fn core_mod_name() -> *mut Identifier {}
}

// ---- Plugin -------------------------------------------------------------

/// Registered through the `jade` entrypoint in fabric.mod.json.
pub struct WwiaJadePlugin;

impl IWailaPlugin for WwiaJadePlugin {
    fn register_client(&mut self, registration: *mut IWailaClientRegistration) {
        added::init();
        unsafe {
            let registration = &mut *registration;
            registration.register_block_component(
                &BLOCK_PROVIDER,
                class("net.minecraft.world.level.block.Block"),
            );
            registration.register_entity_component(
                &ENTITY_PROVIDER,
                class("net.minecraft.world.entity.Entity"),
            );
        }
    }
}

/// Runs after Jade's own mod-name provider (priority 9999).
const PRIORITY: i32 = 10_000;

fn uid() -> *mut Identifier {
    mc::identifier("wwia", "added_in")
}

/// Swaps Jade's "Minecraft" mod-name line for "Minecraft <version>".
fn decorate(tooltip: *mut ITooltip, added: Option<Added>) {
    let Some(added) = added else {
        return;
    };
    let label = added::label(added);
    if label == "Minecraft" {
        // Older than the cutoff: Jade's own line already says "Minecraft".
        return;
    }
    unsafe {
        let tooltip = &mut *tooltip;
        tooltip.remove(JadeIds::core_mod_name());
        let element = (*IThemeHelper::get()).mod_name_element(crate::jvm_std::jstr(&label) as *mut JObject);
        tooltip.add_element(element as *mut LayoutElement);
    }
}

pub struct BlockProvider;
static BLOCK_PROVIDER: BlockProvider = BlockProvider;

impl IComponentProvider for BlockProvider {
    fn get_uid(&self) -> *mut Identifier {
        uid()
    }

    fn get_default_priority(&self) -> i32 {
        PRIORITY
    }

    fn append_tooltip(&self, tooltip: *mut ITooltip, accessor: *mut Accessor, _config: *mut IPluginConfig) {
        let block = unsafe { (*(accessor as *mut BlockAccessor)).get_block() };
        decorate(tooltip, added::block(&mc::block_id(block)));
    }
}

impl IBlockComponentProvider for BlockProvider {}

pub struct EntityProvider;
static ENTITY_PROVIDER: EntityProvider = EntityProvider;

impl IComponentProvider for EntityProvider {
    fn get_uid(&self) -> *mut Identifier {
        uid()
    }

    fn get_default_priority(&self) -> i32 {
        PRIORITY
    }

    fn append_tooltip(&self, tooltip: *mut ITooltip, accessor: *mut Accessor, _config: *mut IPluginConfig) {
        let entity = unsafe { (*(accessor as *mut EntityAccessor)).get_entity() };
        decorate(tooltip, added::entity(&mc::entity_id(entity)));
    }
}

impl IEntityComponentProvider for EntityProvider {}
