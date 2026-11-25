//! When Was It Added (wwia): a Fabric mod written in Rust and compiled to JVM
//! bytecode by rustc_codegen_jvm. Looking at a block or mob shows the
//! Minecraft release that added it, in Jade's tooltip when Jade is installed
//! and in a small HUD line otherwise.
#![feature(extern_types, register_tool)]
#![register_tool(jvm_codegen)]
// JVM bindings pass references and &dyn interfaces through `extern` imports.
#![allow(improper_ctypes)]

pub mod added;
pub mod config;
pub mod data;
pub mod hud;
pub mod jade;
pub mod jvm_std;
pub mod mc;
pub mod menu;
pub mod mixin;

use mc::ClientModInitializer;

/// `fabric.mod.json` client entrypoint.
pub struct WwiaClient;

impl ClientModInitializer for WwiaClient {
    fn on_initialize_client(&mut self) {
        config::load();
        mc::log(&format!(
            "[wwia] loaded {} block and {} entity dates; cutoff {}",
            data::BLOCK_COUNT,
            data::ENTITY_COUNT,
            config::cutoff_name()
        ));
    }
}
