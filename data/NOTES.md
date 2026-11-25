# added.json — sources & judgement calls

## Sources
- **ID lists**: Minecraft 26.3 server jar data generator (`--reports`, registries.json): 1286 blocks, 161 entity types, 1658 items.
- **Post-1.13 first appearance**: per-release registries from misode/mcmeta (`<version>-registries` tags, 1.14 → 26.3)
  plus data-generator `blocks.json` reports from the 1.13 / 1.13.1 / 1.13.2 server jars. Computed by diffing
  consecutive releases, then correcting renames and experimental-only availability.
- **Pre-1.13 history**: minecraft.wiki (MediaWiki API): "Java Edition 1.0.0 … 1.13" Additions sections and the
  History tables of individual block and entity pages.
- **Versions list**: Mojang's version_manifest_v2 releases, plus `1.5`. Mojang's manifest leaves out 1.5, but it was
  released on 13 Mar 2013, so 1.5 additions are tagged "1.5". (1.0.1 was server-only, added nothing and is left out.)

## Rules applied
- Beta 1.9 prereleases/RCs count as **1.0**, not pre-1.0 (brewing stand, splash potion, ender pearl entity etc.).
- Snapshot → the release that contains it. If something was experimental first, the release where it became
  available without the experiment is used:
  1.19.3/1.19.4 "Update 1.20" content → 1.20 (bamboo wood, hanging signs, chiseled bookshelf, piglin head,
  cherry, decorated pot, suspicious sand, torchflower, pink petals, camel, sniffer);
  1.20.3/1.20.5 "Update 1.21" content → 1.21 (crafter, trial spawner, vault, heavy core, tuff/copper variants,
  breeze, wind charges, bogged, ominous item spawner);
  1.21.2 "Winter Drop" → 1.21.4 (pale oak set, pale moss, creaking, creaking heart).
  Armadillo (1.20.5) and display/interaction entities (1.19.4) were in the default game, so they keep those versions.
  26.1 to 26.3 content was never experimental, and the 26.3 jar ships only the minecart_improvements,
  redstone_experiments and trade_rebalance experiments, which add no IDs.
- Renames resolved: short_grass (grass) pre-1.0; dirt_path (grass_path) 1.9; zombified_piglin (zombie_pigman) pre-1.0;
  oak_sign/oak_wall_sign (sign) pre-1.0; iron_chain (chain) 1.16; smooth_stone_slab (1.13's stone_slab) pre-1.0;
  stone_slab (the new plain stone slab) 1.14; boat → oak_boat pre-1.0; potion → splash_potion 1.0,
  lingering_potion 1.9.
- Per-wood boats: spruce/birch/jungle/acacia/dark_oak 1.9 (15w41a); mangrove + all chest boats 1.19; cherry/bamboo 1.20;
  pale oak 1.21.4.
- Variants follow the variant: potted saplings/flowers from 1.4.2 (flower pot), or later if the plant is newer;
  wall heads/banners/fans follow their block; colored beds 1.12 (red bed pre-1.0); cobblestone/mossy walls 1.4.2,
  other walls 1.14; wood buttons/pressure plates/trapdoors other than oak 1.13; other-wood fences/gates/doors 1.8;
  infested mossy/cracked/chiseled stone bricks 1.7.2 (the other three Beta 1.8).

## Judgement calls / uncertain entries
- `smooth_stone` pre-1.0: it existed as the seamless double stone slab from Beta 1.3. It briefly disappeared in
  1.5 and only became a separate block in 1.13.
- `pumpkin` pre-1.0: pumpkins have existed since Alpha. 1.13 only brought back the uncarved texture as its own block.
  `carved_pumpkin` is pre-1.0 too.
- `oak_wood`, `spruce_wood`, `birch_wood`, `jungle_wood` 1.3.1, and `acacia_wood`, `dark_oak_wood` 1.7.2: these
  six-sided bark blocks existed only as unobtainable metadata until 1.13. The date used is when they existed in the game.
- `smooth_sandstone` and `smooth_quartz` 1.5, `smooth_red_sandstone` 1.8: these existed as double-slab variants,
  mostly unobtainable.
- `coarse_dirt` 1.7.2: added as "grassless dirt" in 13w36a and renamed to coarse dirt in 1.8.
- `spruce_planks`, `birch_planks`, `jungle_planks` 1.2.4; `chiseled_sandstone`, `cut_sandstone` 1.2.4. Their textures
  shipped unused in 1.2.1, but the blocks were added in 1.2.4.
- `oak_slab` (and the spruce/birch/jungle slabs) 1.3.1: the real wooden slab. The older "wooden slab" is
  `petrified_oak_slab`, pre-1.0.
- `water_cauldron` 1.0: became its own block in 1.17, but filled cauldrons have existed since 1.0.
- `cat` 1.2.1: you could tame an ocelot into a cat from 12w04a on. It only became a separate entity type in 1.14.
- `ender_pearl` (thrown entity) 1.0: the item came in Beta 1.8, but you couldn't throw it until Beta 1.9 Prerelease 2.
- `elder_guardian` 1.8, `skeleton_horse`/`zombie_horse` 1.6.1, `wither_skeleton`/`zombie_villager` 1.4.2,
  `husk`/`stray` 1.10: these are the dates the mobs were added. They only got their own entity IDs in 1.11.
- `command_block_minecart` 1.7.2 (13w39a); `cave_air`, `void_air` 1.13 (technical).
- `exposed_/weathered_/oxidized_(waxed_)lightning_rod` 1.21.9, the version that made them separate blocks.
- `lava_cauldron` 1.17, `powder_snow_cauldron` 1.17, `light` 1.17, `marker` 1.17.

## Items (1658 ids from the 26.3 `minecraft:item` registry)
- **Sources**: per-release `item/data.json` from misode/mcmeta `<version>-registries` tags (1.14 → 26.3) plus the
  1.13 / 1.13.1 / 1.13.2 data-generator `items.json` reports (stored as `sets.json["item"]`). For pre-1.13 items:
  the "Additions → Items" sections of Java Edition 1.0.0 … 1.12 and item History tables on minecraft.wiki.
- **Block items** (item id equals a block id, 1129 of them) take the block's date unchanged. Every block-id item
  whose first item registration disagreed with the block date was a rename already handled for the block
  (oak_sign, smooth_stone_slab, short_grass, dirt_path, iron_chain). No block item has an item form older or newer
  than its block. `stone_slab` stays 1.14: the 1.13 `stone_slab` item was the smooth stone slab.
- **Non-block items in the 1.13 baseline** are tabled by hand in `I`; anything not listed is pre-1.0 (tools, armor,
  foods, dyes, filled_map, ender_pearl, melon_slice/seeds, raw meats and rotten_flesh from Beta 1.8, shears, cod,
  cooked_cod, music_disc_13/cat, …). Beta 1.9 prerelease items count as 1.0: brewing ingredients, potion,
  glass_bottle, splash_potion, gold_nugget, ender_eye and music discs 11/blocks/chirp/far/mall/mellohi/stal/strad/ward.
- **Spawn eggs** are dated by the egg itself, not by the mob:
  1.1 (the original 20, including zombie_pigman → `zombified_piglin_spawn_egg`); ocelot 1.2.1; bat and witch
  1.4.2; horse 1.6.1; endermite, guardian and rabbit 1.8; shulker 1.9; polar_bear 1.10. Wither skeleton, donkey,
  mule, skeleton/zombie horse, elder guardian, husk and stray eggs appeared in 16w20a, were removed in 1.10-pre2 and
  came back in 1.11, so they are 1.11, along with zombie_villager/vindicator/evoker/vex/llama. Parrot 1.12; fish,
  turtle, phantom, drowned and dolphin eggs 1.13; cat egg 1.14 (removed in 1.10-pre2, re-added in 18w44a).
  ender_dragon/iron_golem/snow_golem/wither eggs 1.19.3. These were in the default game, not part of Update 1.20.
  camel/sniffer eggs → 1.20, breeze/bogged → 1.21, creaking → 1.21.4 (experimental first). armadillo egg 1.20.5.
- **Experimental exceptions**: armadillo_scute, armadillo_spawn_egg and wolf_armor were in the default 1.20.5 game,
  so they are 1.20.5. The 16 dyed bundles and the field_masoned/bordure_indented banner patterns shipped by default
  in 1.21.2, so they are 1.21.2. Everything else first registered in 1.19.3/1.19.4/1.20.3/1.20.5/1.21.2 follows the
  experiment mapping above (trims, sherds, brush, mace, wind_charge, trial keys, ominous items, pale oak, …).
- **Renames**: red_dye/yellow_dye/green_dye (rose_red/dandelion_yellow/cactus_green) pre-1.0; turtle_scute (scute)
  1.13; archer/arms_up/prize/skull_pottery_sherd (pottery_shard_*) 1.20. white/black/blue/brown_dye are new
  in 1.14 (they replaced bone meal, ink sac, lapis and cocoa as dyes but are separate items).
- `map` (empty map) 1.4.2: before 12w34a, crafting a map gave a filled map directly. `filled_map` is pre-1.0.
- `end_crystal` item 1.9 (the entity is 1.0, but you could only place crystals from 15w31a). `command_block_minecart`
  1.7.2 (item 422 dates from 13w39a). `iron_nugget` 1.11.1. `enchanted_golden_apple` 1.3.1. `fire_charge` 1.2.1 (its
  texture existed from Beta 1.9 pre). `music_disc_wait` 1.4.4: it was in the files from Beta 1.9 pre2 but only
  implemented in 1.4.4.
- Wood-variant items follow the wood: other-wood boats 1.9, chest boats 1.19 (bamboo/cherry 1.20, pale oak 1.21.4,
  poplar 26.3).
- **26.3 explorer maps**: 26.3 made every explorer map its own item; before that they were `filled_map` with data.
  Following the `water_cauldron`/`splash_potion` convention, each keeps the date that map variant first existed:
  ocean_monument_map and woodland_mansion_map 1.11; buried_treasure_map 1.13; buried_trial_chambers_map 1.21
  (experimental in 1.20.5); jungle_pyramid_map, swamp_hut_map and the five *_village_map 1.21.5. These were
  Villager Trade Rebalance maps from 1.20.2, made default in 25w07a. The genuinely new maps are 26.3: abandoned_camp,
  buried_ancient_city, buried_mineshaft, desert_pyramid and warm_ocean_ruins.
- **Uncertain**: `bundle` 1.17. It was registered in 20w45a and obtainable only with commands, behind an experiment
  from 1.19.3, and default from 1.21.2 (24w39a). It was never experimental-only when first added, so the
  existed-in-game rule gives 1.17. Use 1.21.2 if "survival-available" is the preferred meaning. The explorer-map
  dates above are a convention choice too; 26.3 is the alternative.
- Verification: `wiki_items.py` in the scratch dir batch-fetches each item's wiki page and compares the first Java
  history entry. Every disagreement was an aggregate page (Spawn Egg, Boat, Bundle, Explorer Map, Planks, …), the
  experimental mapping, or a mob/texture-only entry (cod/salmon mob pages, crying obsidian texture).

Generator script: `build_added.py` (expects the inputs in ~/.hermes/cache/scratch/wwia-data: manifest.json,
sets.json built from mcmeta registries + 1.13.x reports (keys block, entity_type, item), and the 26.3
registries.json).
