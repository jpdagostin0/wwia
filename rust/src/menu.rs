//! Mod Menu "Configure" screen, built with Cloth Config when it's installed.
use crate::jvm_std::{Consumer, Function, JBoolean, JInteger, JObject, Runnable};
use crate::mc::{self, Component, Screen, literal, translatable};
use crate::{config, data};

// ---- Mod Menu -------------------------------------------------------------

#[jvm::interface("com.terraformersmc.modmenu.api.ModMenuApi", rename_all = "camelCase")]
pub trait ModMenuApi {
    fn get_mod_config_screen_factory(&self) -> &'static dyn ConfigScreenFactory;
}

#[jvm::interface("com.terraformersmc.modmenu.api.ConfigScreenFactory")]
pub trait ConfigScreenFactory {
    fn create(&self, parent: *mut Screen) -> *mut Screen;
}

/// Registered through the `modmenu` entrypoint in fabric.mod.json.
pub struct WwiaModMenu;

impl ModMenuApi for WwiaModMenu {
    fn get_mod_config_screen_factory(&self) -> &'static dyn ConfigScreenFactory {
        &SCREEN_FACTORY
    }
}

pub struct ScreenFactory;
static SCREEN_FACTORY: ScreenFactory = ScreenFactory;

impl ConfigScreenFactory for ScreenFactory {
    fn create(&self, parent: *mut Screen) -> *mut Screen {
        if !mc::is_mod_loaded("cloth-config") {
            // No Cloth Config: stay on Mod Menu's screen. The settings are
            // still editable in config/wwia.properties.
            return parent;
        }
        build_screen(parent)
    }
}

// ---- Cloth Config -------------------------------------------------------

#[jvm::interface("me.shedaniel.clothconfig2.api.ConfigBuilder", rename_all = "camelCase")]
impl ConfigBuilder {
    #[jvm::static_method]
    pub fn create() -> *mut Self {}

    #[jvm::method]
    pub fn set_parent_screen(&mut self, parent: *mut Screen) -> *mut Self {}

    #[jvm::method]
    pub fn set_title(&mut self, title: *mut Component) -> *mut Self {}

    #[jvm::method]
    pub fn get_or_create_category(&mut self, name: *mut Component) -> *mut ConfigCategory {}

    #[jvm::method]
    pub fn entry_builder(&mut self) -> *mut ConfigEntryBuilder {}

    #[jvm::method]
    pub fn set_saving_runnable(&mut self, runnable: &dyn Runnable) -> *mut Self {}

    #[jvm::method]
    pub fn build(&mut self) -> *mut Screen {}
}

#[jvm::interface("me.shedaniel.clothconfig2.api.ConfigCategory", rename_all = "camelCase")]
impl ConfigCategory {
    #[jvm::method]
    pub fn add_entry(&mut self, entry: *mut ConfigListEntry) -> *mut Self {}
}

#[jvm::class("me.shedaniel.clothconfig2.api.AbstractConfigListEntry")]
pub struct ConfigListEntry;

#[jvm::class("me.shedaniel.clothconfig2.gui.entries.IntegerSliderEntry")]
pub struct IntegerSliderEntry;

#[jvm::class("me.shedaniel.clothconfig2.gui.entries.BooleanListEntry")]
pub struct BooleanListEntry;

#[jvm::interface("me.shedaniel.clothconfig2.api.ConfigEntryBuilder", rename_all = "camelCase")]
impl ConfigEntryBuilder {
    #[jvm::method]
    pub fn start_int_slider(&mut self, name: *mut Component, value: i32, min: i32, max: i32) -> *mut IntSliderBuilder {}

    #[jvm::method]
    pub fn start_boolean_toggle(&mut self, name: *mut Component, value: bool) -> *mut BooleanToggleBuilder {}
}

#[jvm::class("me.shedaniel.clothconfig2.impl.builders.IntSliderBuilder", rename_all = "camelCase")]
impl IntSliderBuilder {
    #[jvm::method]
    pub fn set_default_value(&mut self, value: i32) -> *mut Self {}

    #[jvm::method]
    pub fn set_text_getter(&mut self, getter: &dyn Function) -> *mut Self {}

    #[jvm::method]
    pub fn set_save_consumer(&mut self, consumer: &dyn Consumer) -> *mut Self {}

    #[jvm::method]
    pub fn set_tooltip(&mut self, lines: *mut ComponentArray) -> *mut Self {}

    #[jvm::method]
    pub fn build(&mut self) -> *mut IntegerSliderEntry {}
}

#[jvm::class("me.shedaniel.clothconfig2.impl.builders.BooleanToggleBuilder", rename_all = "camelCase")]
impl BooleanToggleBuilder {
    #[jvm::method]
    pub fn set_default_value(&mut self, value: bool) -> *mut Self {}

    #[jvm::method]
    pub fn set_save_consumer(&mut self, consumer: &dyn Consumer) -> *mut Self {}

    #[jvm::method]
    pub fn set_tooltip(&mut self, lines: *mut ComponentArray) -> *mut Self {}

    #[jvm::method]
    pub fn build(&mut self) -> *mut BooleanListEntry {}
}

/// `java.util.Optional`, used to pass a `Component[]` tooltip (the Rust
/// bindings can't spell array types, so the array travels as an Object).
#[jvm::class("java.util.Optional")]
impl ComponentArray {
    #[jvm::static_method("of")]
    pub fn of(value: *mut JObject) -> *mut Self {}
}

#[jvm::class("java.lang.reflect.Array")]
impl JArray {
    #[jvm::static_method(name = "newInstance")]
    pub fn new_instance(component_type: *mut crate::jvm_std::JClass, length: i32) -> *mut JObject {}

    #[jvm::static_method(name = "set")]
    pub fn set(array: *mut JObject, index: i32, value: *mut JObject) {}
}

fn components(lines: &[*mut Component]) -> *mut ComponentArray {
    let array = JArray::new_instance(
        crate::jvm_std::class("net.minecraft.network.chat.Component"),
        lines.len() as i32,
    );
    for (index, line) in lines.iter().enumerate() {
        JArray::set(array, index as i32, *line as *mut JObject);
    }
    ComponentArray::of(array)
}

/// Slider label: the version name for a cutoff index.
pub struct VersionText;
static VERSION_TEXT: VersionText = VersionText;

impl Function for VersionText {
    fn apply(&self, value: *mut JObject) -> *mut JObject {
        let index = unsafe { (*(value as *mut JInteger)).int_value() } as usize;
        let version = data::VERSIONS.get(index).copied().unwrap_or("?");
        literal(&format!("Show versions from: {version}")) as *mut JObject
    }
}

pub struct SaveCutoff;
static SAVE_CUTOFF: SaveCutoff = SaveCutoff;

impl Consumer for SaveCutoff {
    fn accept(&self, value: *mut JObject) {
        config::set_cutoff(unsafe { (*(value as *mut JInteger)).int_value() } as usize);
    }
}

pub struct SaveShowHud;
static SAVE_SHOW_HUD: SaveShowHud = SaveShowHud;

impl Consumer for SaveShowHud {
    fn accept(&self, value: *mut JObject) {
        config::set_show_hud(unsafe { (*(value as *mut JBoolean)).boolean_value() });
    }
}

pub struct SaveFile;
static SAVE_FILE: SaveFile = SaveFile;

impl Runnable for SaveFile {
    fn run(&self) {
        config::save();
    }
}

fn build_screen(parent: *mut Screen) -> *mut Screen {
    unsafe {
        let builder = &mut *ConfigBuilder::create();
        builder.set_parent_screen(parent);
        builder.set_title(translatable("wwia.config.title"));
        builder.set_saving_runnable(&SAVE_FILE);
        let category = &mut *builder.get_or_create_category(translatable("wwia.config.general"));
        let entries = &mut *builder.entry_builder();

        let last = data::VERSIONS.len() as i32 - 1;
        let slider = &mut *entries.start_int_slider(
            translatable("wwia.config.cutoff"),
            config::cutoff() as i32,
            0,
            last.max(0),
        );
        slider.set_default_value(config::DEFAULT_CUTOFF as i32);
        slider.set_text_getter(&VERSION_TEXT);
        slider.set_save_consumer(&SAVE_CUTOFF);
        slider.set_tooltip(components(&[
            translatable("wwia.config.cutoff.tooltip"),
            translatable("wwia.config.cutoff.tooltip2"),
        ]));
        category.add_entry(slider.build() as *mut ConfigListEntry);

        let toggle = &mut *entries.start_boolean_toggle(
            translatable("wwia.config.show_hud"),
            config::show_hud(),
        );
        toggle.set_default_value(true);
        toggle.set_save_consumer(&SAVE_SHOW_HUD);
        toggle.set_tooltip(components(&[translatable("wwia.config.show_hud.tooltip")]));
        category.add_entry(toggle.build() as *mut ConfigListEntry);

        builder.build()
    }
}
