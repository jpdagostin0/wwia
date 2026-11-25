# When Was It Added (wwia)

A client-side Fabric mod for Minecraft 26.3. When you look at a block or mob, it
shows the Minecraft release that added it:

- **With [Jade](https://modrinth.com/mod/jade):** Jade's mod-name line changes
  from "Minecraft" to, for example, "Minecraft 1.16".
- **With [REI](https://modrinth.com/mod/rei):** item tooltips in REI's item
  list and recipe pages read "Minecraft 1.21" (mace) instead of "Minecraft".
  This is a Mixin into REI's `AbstractEntryStack.getTooltip`, from a separate
  optional mixin config so the mod still loads without REI.
- **Without Jade:** the mod draws its own small line at the top of the screen.
  This uses a Mixin into `Hud.extractRenderState`, and you can turn it off in
  the settings.

Anything from before Java Edition 1.0 (Classic through Beta) just says
"Minecraft". The **version cutoff** setting (default 1.0, can't go lower) does
the same for newer releases: with the cutoff at 1.17, a crimson plank reads
"Minecraft" while an armadillo reads "Minecraft 1.20.5". You can change it in
Mod Menu → When Was It Added → Configure (needs Cloth Config), or in
`config/wwia.properties`.

## Rust

There is no Java source. Everything is Rust under `rust/`: the client
entrypoint, the Jade plugin and providers, the REI tooltip hook, the Mod Menu / Cloth Config screen,
and the Mixin classes. It is compiled to JVM bytecode by
[rustc_codegen_jvm](https://github.com/IntegralPilot/rustc_codegen_jvm).

Mixins are written as Rust structs with method annotations. For example, from
`rust/src/mixin.rs`:

```rust
#[jvm::annotation("org.spongepowered.asm.mixin.Mixin",
    value = [class("net.minecraft.client.gui.Hud")], retention = "class")]
pub struct HudMixin;

impl HudMixin {
    #[jvm::annotation("org.spongepowered.asm.mixin.injection.Inject",
        method = ["extractRenderState"],
        at = [annotation("org.spongepowered.asm.mixin.injection.At", value = "TAIL")])]
    pub fn wwia_extract_render_state(&mut self, graphics: *mut GuiGraphicsExtractor,
                                     _delta: *mut DeltaTracker, _ci: *mut CallbackInfo) {
        crate::hud::extract(graphics);
    }
}
```

Upstream rustc_codegen_jvm doesn't support this yet. It needs the
[`mixin-support` branch](https://github.com/jpdagostin0/rustc_codegen_jvm/tree/mixin-support)
of my fork, which adds two things:

- `#[jvm::annotation]` works on impl methods.
- `@Mixin` structs are emitted as clean template classes, with no `RustCopy`,
  `eq` or public static bridges, because Mixin rejects or would merge those.

`rust/Cargo.toml` pulls the `jvm` macro crate from that branch, and cargo-jvm's
backend has to be built from it too:

```sh
git clone -b mixin-support https://github.com/jpdagostin0/rustc_codegen_jvm
cd rustc_codegen_jvm && cargo install --path cargo-jvm && cargo jvm setup "$PWD"
```

## Data

`data/added.json` maps every 26.3 block, entity type and item to its first release.
It was built from the 26.3 registries, PrismarineJS minecraft-data (1.13+), and
minecraft.wiki history and version pages. `data/NOTES.md` lists the judgement
calls. `tools/gen_data.py` regenerates `rust/src/data*.{rs,txt}` from it.

## Building

Requires JDK 25, rustup, and cargo-jvm with the patched backend.

```sh
./gradlew build                     # with JAVA_HOME pointing at a JDK 25   # -> build/libs/wwia-1.0.0.jar
./gradlew runClient                 # dev client with Jade, Mod Menu and Cloth Config
./gradlew runClient -PnoJade        # tests the HUD fallback
./gradlew runClient -PnoRei         # without REI
```
