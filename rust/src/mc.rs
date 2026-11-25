//! Bindings to Minecraft 26.3 (unobfuscated), Fabric Loader and SLF4J.
#![allow(dead_code)]
use crate::jvm_std::{JObject, JPath, JString, jstr, rstr, to_string};

// ---- Fabric -------------------------------------------------------------

#[jvm::interface("net.fabricmc.api.ClientModInitializer", rename_all = "camelCase")]
pub trait ClientModInitializer {
    fn on_initialize_client(&mut self);
}

#[jvm::interface("net.fabricmc.loader.api.FabricLoader", rename_all = "camelCase")]
impl FabricLoader {
    #[jvm::static_method]
    pub fn get_instance() -> *mut Self {}

    #[jvm::method]
    pub fn is_mod_loaded(&self, id: *mut JString) -> bool {}

    #[jvm::method]
    pub fn get_config_dir(&self) -> *mut JPath {}
}

pub fn is_mod_loaded(id: &str) -> bool {
    unsafe { (*FabricLoader::get_instance()).is_mod_loaded(jstr(id)) }
}

pub fn config_dir() -> String {
    unsafe { to_string((*FabricLoader::get_instance()).get_config_dir() as *mut JObject) }
}

// ---- Logging ------------------------------------------------------------

#[jvm::interface("org.slf4j.Logger")]
impl Logger {
    #[jvm::method]
    pub fn info(&self, message: *mut JString) {}
}

#[jvm::static_method(class = "org.slf4j.LoggerFactory", name = "getLogger")]
fn get_logger(name: *mut JString) -> *mut Logger {}

pub fn log(message: &str) {
    unsafe { (*get_logger(jstr("wwia"))).info(jstr(message)) }
}

// ---- Core ---------------------------------------------------------------

#[jvm::class("net.minecraft.resources.Identifier", rename_all = "camelCase")]
impl Identifier {
    #[jvm::static_method]
    pub fn from_namespace_and_path(namespace: *mut JString, path: *mut JString) -> *mut Self {}
}

pub fn identifier(namespace: &str, path: &str) -> *mut Identifier {
    Identifier::from_namespace_and_path(jstr(namespace), jstr(path))
}

#[jvm::interface("net.minecraft.core.DefaultedRegistry", rename_all = "camelCase")]
impl DefaultedRegistry {
    #[jvm::method]
    pub fn get_key(&self, value: *mut JObject) -> *mut Identifier {}
}

#[jvm::class("net.minecraft.core.registries.BuiltInRegistries")]
impl BuiltInRegistries {
    #[jvm::static_field(name = "BLOCK")]
    pub fn block() -> *mut DefaultedRegistry {}

    #[jvm::static_field(name = "ENTITY_TYPE")]
    pub fn entity_type() -> *mut DefaultedRegistry {}
}

#[jvm::class("net.minecraft.core.BlockPos")]
pub struct BlockPos;

#[jvm::class("net.minecraft.world.level.block.Block", rename_all = "camelCase")]
impl Block {
    #[jvm::method]
    pub fn get_name(&self) -> *mut MutableComponent {}
}

#[jvm::class("net.minecraft.world.level.block.state.BlockState", rename_all = "camelCase")]
impl BlockState {
    #[jvm::method]
    pub fn get_block(&self) -> *mut Block {}
}

#[jvm::class("net.minecraft.world.entity.EntityType")]
pub struct EntityType;

#[jvm::class("net.minecraft.world.entity.Entity", rename_all = "camelCase")]
impl Entity {
    #[jvm::method]
    pub fn get_type(&self) -> *mut EntityType {}

    #[jvm::method]
    pub fn get_name(&self) -> *mut Component {}
}

/// `minecraft:stone`-style registry id of a block.
pub fn block_id(block: *mut Block) -> String {
    unsafe { to_string((*BuiltInRegistries::block()).get_key(block as *mut JObject) as *mut JObject) }
}

/// `minecraft:creeper`-style registry id of an entity's type.
pub fn entity_id(entity: *mut Entity) -> String {
    unsafe {
        let ty = (*entity).get_type();
        to_string((*BuiltInRegistries::entity_type()).get_key(ty as *mut JObject) as *mut JObject)
    }
}

// ---- Text ---------------------------------------------------------------

#[jvm::interface("net.minecraft.network.chat.Component", rename_all = "camelCase")]
impl Component {
    #[jvm::static_method]
    pub fn literal(text: *mut JString) -> *mut MutableComponent {}

    #[jvm::static_method]
    pub fn translatable(key: *mut JString) -> *mut MutableComponent {}

    #[jvm::method]
    pub fn get_string(&self) -> *mut JString {}
}

#[jvm::class("net.minecraft.network.chat.MutableComponent", rename_all = "camelCase")]
impl MutableComponent {
    #[jvm::method]
    pub fn get_string(&self) -> *mut JString {}
}

pub fn literal(text: &str) -> *mut Component {
    Component::literal(jstr(text)) as *mut Component
}

pub fn translatable(key: &str) -> *mut Component {
    Component::translatable(jstr(key)) as *mut Component
}

pub fn component_string(component: *mut Component) -> String {
    unsafe { rstr((*component).get_string()) }
}

// ---- Client -------------------------------------------------------------

#[jvm::class("net.minecraft.client.Minecraft", rename_all = "camelCase")]
impl Minecraft {
    #[jvm::static_method]
    pub fn get_instance() -> *mut Self {}

    #[jvm::field]
    pub fn hit_result(&self) -> *mut HitResult {}

    #[jvm::field]
    pub fn level(&self) -> *mut ClientLevel {}

    #[jvm::field]
    pub fn font(&self) -> *mut Font {}

    #[jvm::field]
    pub fn gui(&self) -> *mut Gui {}
}

#[jvm::class("net.minecraft.client.gui.Gui")]
impl Gui {
    /// The open screen, or null in-game.
    #[jvm::method]
    pub fn screen(&self) -> *mut Screen {}
}

#[jvm::class("net.minecraft.client.multiplayer.ClientLevel", rename_all = "camelCase")]
impl ClientLevel {
    #[jvm::method]
    pub fn get_block_state(&self, pos: *mut BlockPos) -> *mut BlockState {}
}

#[jvm::class("net.minecraft.world.phys.HitResult", rename_all = "camelCase")]
impl HitResult {
    #[jvm::method]
    pub fn get_type(&self) -> *mut HitResultType {}
}

#[jvm::class("net.minecraft.world.phys.HitResult$Type")]
pub struct HitResultType;

#[jvm::class("java.lang.Enum")]
impl JEnum {
    #[jvm::method]
    pub fn ordinal(&self) -> i32 {}
}

/// `HitResult.Type` ordinals.
pub const HIT_MISS: i32 = 0;
pub const HIT_BLOCK: i32 = 1;
pub const HIT_ENTITY: i32 = 2;

#[jvm::class("net.minecraft.world.phys.BlockHitResult", rename_all = "camelCase")]
impl BlockHitResult {
    #[jvm::method]
    pub fn get_block_pos(&self) -> *mut BlockPos {}
}

#[jvm::class("net.minecraft.world.phys.EntityHitResult", rename_all = "camelCase")]
impl EntityHitResult {
    #[jvm::method]
    pub fn get_entity(&self) -> *mut Entity {}
}

#[jvm::class("net.minecraft.client.gui.Font", rename_all = "camelCase")]
impl Font {
    #[jvm::method("width")]
    pub fn width(&self, text: *mut JString) -> i32 {}

    #[jvm::field]
    pub fn line_height(&self) -> i32 {}
}

#[jvm::class("net.minecraft.client.gui.GuiGraphicsExtractor", rename_all = "camelCase")]
impl GuiGraphicsExtractor {
    #[jvm::method]
    pub fn gui_width(&self) -> i32 {}

    #[jvm::method]
    pub fn gui_height(&self) -> i32 {}

    #[jvm::method]
    pub fn fill(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: i32) {}

    #[jvm::method("text")]
    pub fn text(&mut self, font: *mut Font, text: *mut JString, x: i32, y: i32, color: i32, shadow: bool) {}
}

#[jvm::class("net.minecraft.client.DeltaTracker")]
pub struct DeltaTracker;

#[jvm::class("org.spongepowered.asm.mixin.injection.callback.CallbackInfo")]
pub struct CallbackInfo;

#[jvm::class("net.minecraft.client.gui.screens.Screen")]
pub struct Screen;
