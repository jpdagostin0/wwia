import json, re
W = '/home/jp/.hermes/cache/scratch/wwia-data/'
m = json.load(open(W + 'manifest.json'))
REL = [v['id'] for v in m['versions'] if v['type'] == 'release'][::-1]
# 1.5 was a real release (13 Mar 2013) but is missing from Mojang's manifest
VERSIONS = REL[:REL.index('1.5.1')] + ['1.5'] + REL[REL.index('1.5.1'):]
assert VERSIONS[-1] == '26.3'
sets = json.load(open(W + 'sets.json'))
cur = json.load(open(W + 'gen/reports/registries.json'))
CUR_B = [k.split(':')[1] for k in cur['minecraft:block']['entries']]
CUR_E = [k.split(':')[1] for k in cur['minecraft:entity_type']['entries']]
CUR_I = [k.split(':')[1] for k in cur['minecraft:item']['entries']]
MOD = REL[REL.index('1.13'):]

def first(kind, i):
    for v in MOD:
        if v in sets[kind] and i in sets[kind][v]:
            return v
    return None

COLORS = ['white', 'orange', 'magenta', 'light_blue', 'yellow', 'lime', 'pink', 'gray', 'light_gray',
          'cyan', 'purple', 'blue', 'brown', 'green', 'red', 'black']
OLD_WOOD = ['oak', 'spruce', 'birch']  # pre-1.0 log/leaves/sapling types

# ---------- blocks that existed in 1.13 (pre-flattening history) ----------
B = {}
def s(v, *ids):
    for i in ids:
        B[i] = v

# 1.0 (Beta 1.9 prereleases / RC)
s('1.0', 'mycelium', 'lily_pad', 'nether_bricks', 'nether_brick_fence', 'nether_brick_stairs', 'nether_wart',
  'enchanting_table', 'brewing_stand', 'cauldron', 'end_portal', 'end_portal_frame', 'end_stone', 'dragon_egg')
# 1.2.1
s('1.2.1', 'jungle_log', 'jungle_leaves', 'jungle_sapling', 'redstone_lamp', 'chiseled_stone_bricks')
# 1.2.4
s('1.2.4', 'spruce_planks', 'birch_planks', 'jungle_planks', 'chiseled_sandstone', 'cut_sandstone')
# 1.3.1
s('1.3.1', 'oak_slab', 'spruce_slab', 'birch_slab', 'jungle_slab', 'emerald_ore', 'emerald_block', 'ender_chest',
  'sandstone_stairs', 'tripwire_hook', 'tripwire', 'spruce_stairs', 'birch_stairs', 'jungle_stairs', 'cocoa',
  'oak_wood', 'spruce_wood', 'birch_wood', 'jungle_wood')
# 1.4.2
s('1.4.2', 'beacon', 'command_block', 'flower_pot', 'oak_button', 'cobblestone_wall', 'mossy_cobblestone_wall',
  'anvil', 'chipped_anvil', 'damaged_anvil', 'carrots', 'potatoes',
  'skeleton_skull', 'skeleton_wall_skull', 'wither_skeleton_skull', 'wither_skeleton_wall_skull',
  'zombie_head', 'zombie_wall_head', 'player_head', 'player_wall_head', 'creeper_head', 'creeper_wall_head')
for p in ['oak_sapling', 'spruce_sapling', 'birch_sapling', 'jungle_sapling', 'dandelion', 'poppy',
          'red_mushroom', 'brown_mushroom', 'cactus', 'dead_bush', 'fern']:
    B['potted_' + p] = '1.4.2'
# 1.4.6
s('1.4.6', 'nether_brick_slab')
# 1.5
s('1.5', 'nether_quartz_ore', 'redstone_block', 'daylight_detector', 'hopper', 'comparator', 'trapped_chest',
  'light_weighted_pressure_plate', 'heavy_weighted_pressure_plate', 'activator_rail', 'quartz_block',
  'chiseled_quartz_block', 'quartz_pillar', 'quartz_slab', 'quartz_stairs', 'dropper', 'smooth_sandstone',
  'smooth_quartz')
# 1.6.1
s('1.6.1', 'hay_block', 'terracotta', 'coal_block', *[c + '_carpet' for c in COLORS], *[c + '_terracotta' for c in COLORS])
# 1.7.2
s('1.7.2', 'blue_orchid', 'allium', 'azure_bluet', 'red_tulip', 'orange_tulip', 'white_tulip', 'pink_tulip',
  'oxeye_daisy', 'sunflower', 'lilac', 'tall_grass', 'large_fern', 'rose_bush', 'peony', 'packed_ice', 'podzol',
  'coarse_dirt', 'red_sand', 'infested_mossy_stone_bricks', 'infested_cracked_stone_bricks',
  'infested_chiseled_stone_bricks', *[c + '_stained_glass' for c in COLORS], *[c + '_stained_glass_pane' for c in COLORS],
  'acacia_log', 'dark_oak_log', 'acacia_planks', 'dark_oak_planks', 'acacia_leaves', 'dark_oak_leaves',
  'acacia_sapling', 'dark_oak_sapling', 'acacia_wood', 'dark_oak_wood', 'acacia_stairs', 'dark_oak_stairs',
  'acacia_slab', 'dark_oak_slab')
for p in ['acacia_sapling', 'dark_oak_sapling', 'blue_orchid', 'allium', 'azure_bluet', 'red_tulip', 'orange_tulip',
          'white_tulip', 'pink_tulip', 'oxeye_daisy']:
    B['potted_' + p] = '1.7.2'
# 1.8
s('1.8', 'andesite', 'diorite', 'granite', 'polished_andesite', 'polished_diorite', 'polished_granite', 'slime_block',
  'barrier', 'iron_trapdoor', 'prismarine', 'prismarine_bricks', 'dark_prismarine', 'sea_lantern', 'wet_sponge',
  'red_sandstone', 'chiseled_red_sandstone', 'cut_red_sandstone', 'smooth_red_sandstone', 'red_sandstone_stairs',
  'red_sandstone_slab', *[c + '_banner' for c in COLORS], *[c + '_wall_banner' for c in COLORS])
for w in ['spruce', 'birch', 'jungle', 'acacia', 'dark_oak']:
    s('1.8', w + '_fence', w + '_fence_gate', w + '_door')
# 1.9
s('1.9', 'chorus_plant', 'chorus_flower', 'dragon_head', 'dragon_wall_head', 'end_gateway', 'end_rod',
  'end_stone_bricks', 'grass_path', 'purpur_block', 'purpur_pillar', 'purpur_slab', 'purpur_stairs',
  'structure_block', 'frosted_ice', 'beetroots', 'chain_command_block', 'repeating_command_block')
# 1.10
s('1.10', 'magma_block', 'nether_wart_block', 'red_nether_bricks', 'bone_block', 'structure_void')
# 1.11
s('1.11', 'observer', *[c + '_shulker_box' for c in COLORS])
# 1.12
s('1.12', *[c + '_concrete' for c in COLORS], *[c + '_concrete_powder' for c in COLORS],
  *[c + '_glazed_terracotta' for c in COLORS], *[c + '_bed' for c in COLORS if c != 'red'])
# 1.13
s('1.13', 'cave_air', 'void_air', 'bubble_column', 'seagrass', 'tall_seagrass', 'kelp', 'kelp_plant',
  'dried_kelp_block', 'turtle_egg', 'sea_pickle', 'blue_ice', 'conduit', 'shulker_box',
  'prismarine_stairs', 'prismarine_brick_stairs', 'dark_prismarine_stairs', 'prismarine_slab',
  'prismarine_brick_slab', 'dark_prismarine_slab')
for w in ['oak', 'spruce', 'birch', 'jungle', 'acacia', 'dark_oak']:
    s('1.13', 'stripped_%s_log' % w, 'stripped_%s_wood' % w)
    if w != 'oak':
        s('1.13', w + '_button', w + '_pressure_plate', w + '_trapdoor')
for c in ['tube', 'brain', 'bubble', 'fire', 'horn']:
    s('1.13', c + '_coral', c + '_coral_block', c + '_coral_fan', c + '_coral_wall_fan',
      'dead_%s_coral_block' % c, 'dead_%s_coral_fan' % c, 'dead_%s_coral_wall_fan' % c)
    s('1.13.1', 'dead_%s_coral' % c)

# ---------- block renames / special cases outside 1.13 baseline ----------
RENAMED_B = {
    'oak_sign': 'pre-1.0', 'oak_wall_sign': 'pre-1.0',         # sign/wall_sign renamed 1.14
    'smooth_stone_slab': 'pre-1.0',                             # was stone_slab in 1.13
    'stone_slab': '1.14',                                       # plain stone slab; the 1.13 id meant smooth stone slab
    'dirt_path': '1.9',                                         # grass_path renamed 1.17
    'short_grass': 'pre-1.0',                                   # grass renamed 1.20.3
    'iron_chain': '1.16',                                       # chain renamed 1.21.9
    'water_cauldron': '1.0',                                    # split from cauldron 1.17; filled cauldrons existed since 1.0
}
# experimental-at-first-registration -> release where it shipped in the default game
EXP = {'1.19.3': '1.20', '1.19.4': '1.20', '1.20.3': '1.21', '1.20.5': '1.21', '1.21.2': '1.21.4'}
EXP_DEFAULT_OK = {  # things first registered in an "EXP" version that were NOT experimental there
    'block': {'short_grass'},  # handled by rename anyway
    'entity_type': {'armadillo', 'block_display', 'item_display', 'text_display', 'interaction'},
}

# ---------- entities ----------
E = {}
def e(v, *ids):
    for i in ids:
        E[i] = v
e('pre-1.0', 'arrow', 'cave_spider', 'chest_minecart', 'chicken', 'cow', 'creeper', 'egg', 'enderman',
  'experience_orb', 'falling_block', 'fireball', 'fishing_bobber', 'furnace_minecart', 'ghast', 'giant', 'item',
  'lightning_bolt', 'minecart', 'painting', 'pig', 'player', 'sheep', 'silverfish', 'skeleton', 'slime', 'snowball',
  'spider', 'squid', 'tnt', 'wolf', 'zombie', 'zombified_piglin', 'oak_boat')
e('1.0', 'blaze', 'ender_dragon', 'magma_cube', 'mooshroom', 'snow_golem', 'villager', 'end_crystal', 'eye_of_ender',
  'small_fireball', 'splash_potion', 'ender_pearl')
e('1.2.1', 'ocelot', 'iron_golem', 'experience_bottle', 'cat')
e('1.4.2', 'bat', 'witch', 'wither', 'wither_skeleton', 'wither_skull', 'zombie_villager', 'item_frame')
e('1.4.6', 'firework_rocket')
e('1.5', 'tnt_minecart', 'hopper_minecart', 'spawner_minecart')
e('1.6.1', 'horse', 'donkey', 'mule', 'skeleton_horse', 'zombie_horse', 'leash_knot')
e('1.7.2', 'command_block_minecart')
e('1.8', 'endermite', 'guardian', 'elder_guardian', 'rabbit', 'armor_stand')
e('1.9', 'shulker', 'shulker_bullet', 'area_effect_cloud', 'dragon_fireball', 'spectral_arrow', 'lingering_potion',
  'spruce_boat', 'birch_boat', 'jungle_boat', 'acacia_boat', 'dark_oak_boat')
e('1.10', 'polar_bear', 'husk', 'stray')
e('1.11', 'llama', 'llama_spit', 'vindicator', 'evoker', 'evoker_fangs', 'vex')
e('1.12', 'parrot', 'illusioner')
e('1.13', 'cod', 'salmon', 'pufferfish', 'tropical_fish', 'dolphin', 'drowned', 'phantom', 'trident', 'turtle')
e('1.14', 'fox', 'panda', 'pillager', 'ravager', 'trader_llama', 'wandering_trader')
e('1.19', 'mangrove_boat', *[w + '_chest_boat' for w in ['oak', 'spruce', 'birch', 'jungle', 'acacia', 'dark_oak', 'mangrove']])
e('1.20', 'cherry_boat', 'cherry_chest_boat', 'bamboo_raft', 'bamboo_chest_raft')
e('1.21.4', 'pale_oak_boat', 'pale_oak_chest_boat')

PRE13_E = set(E)  # entity ids whose value is fully manual

# ---------- items ----------
# Block items (item id == block id) take the block's date. I holds the non-block items that already existed in the
# 1.13 registry (pre-flattening history, from the wiki "Additions" sections and item History tables); anything
# from the 1.13 baseline that is not listed is pre-1.0. Items first registered after 1.13 use the registry diff.
I = {}
def it(v, *ids):
    for i in ids:
        I[i] = v
it('1.0', 'blaze_rod', 'ghast_tear', 'gold_nugget', 'potion', 'glass_bottle', 'spider_eye', 'fermented_spider_eye',
   'blaze_powder', 'magma_cream', 'ender_eye', 'glistering_melon_slice', 'splash_potion',
   *['music_disc_' + d for d in ['11', 'blocks', 'chirp', 'far', 'mall', 'mellohi', 'stal', 'strad', 'ward']])
it('1.1', *[m + '_spawn_egg' for m in ['blaze', 'cave_spider', 'chicken', 'cow', 'creeper', 'enderman', 'ghast',
   'magma_cube', 'mooshroom', 'pig', 'sheep', 'silverfish', 'skeleton', 'slime', 'spider', 'squid', 'villager',
   'wolf', 'zombie']])
it('1.2.1', 'ocelot_spawn_egg', 'experience_bottle', 'fire_charge')
it('1.3.1', 'writable_book', 'written_book', 'emerald', 'enchanted_golden_apple')
it('1.4.2', 'bat_spawn_egg', 'witch_spawn_egg', 'item_frame', 'carrot', 'potato', 'baked_potato', 'poisonous_potato',
   'golden_carrot', 'carrot_on_a_stick', 'nether_star', 'pumpkin_pie', 'map')   # map = empty map (12w34a)
it('1.4.4', 'music_disc_wait')
it('1.4.6', 'firework_rocket', 'firework_star', 'enchanted_book')
it('1.5', 'nether_brick', 'quartz', 'tnt_minecart', 'hopper_minecart')
it('1.6.1', 'horse_spawn_egg', 'iron_horse_armor', 'golden_horse_armor', 'diamond_horse_armor', 'lead', 'name_tag')
it('1.7.2', 'salmon', 'cooked_salmon', 'pufferfish', 'tropical_fish', 'command_block_minecart')
it('1.8', 'prismarine_shard', 'prismarine_crystals', 'mutton', 'cooked_mutton', 'rabbit', 'cooked_rabbit',
   'rabbit_stew', 'rabbit_foot', 'rabbit_hide', 'armor_stand', 'endermite_spawn_egg', 'guardian_spawn_egg',
   'rabbit_spawn_egg')
it('1.9', 'spectral_arrow', 'tipped_arrow', 'beetroot', 'beetroot_seeds', 'beetroot_soup', 'chorus_fruit',
   'popped_chorus_fruit', 'dragon_breath', 'lingering_potion', 'shield', 'elytra', 'end_crystal', 'shulker_spawn_egg',
   'spruce_boat', 'birch_boat', 'jungle_boat', 'acacia_boat', 'dark_oak_boat')
it('1.10', 'polar_bear_spawn_egg')
# 16w20a (1.10) eggs were removed again in 1.10-pre2, so these date from their re-addition in 1.11
it('1.11', 'totem_of_undying', 'shulker_shell', *[m + '_spawn_egg' for m in ['wither_skeleton', 'donkey', 'mule',
   'skeleton_horse', 'zombie_horse', 'elder_guardian', 'husk', 'stray', 'zombie_villager', 'vindicator', 'llama',
   'evoker', 'vex']])
it('1.11.1', 'iron_nugget')
it('1.12', 'knowledge_book', 'parrot_spawn_egg')
it('1.13', 'turtle_helmet', 'trident', 'phantom_membrane', 'nautilus_shell', 'heart_of_the_sea', 'cod_bucket',
   'salmon_bucket', 'pufferfish_bucket', 'tropical_fish_bucket', 'dried_kelp', 'debug_stick',
   *[m + '_spawn_egg' for m in ['cod', 'salmon', 'pufferfish', 'tropical_fish', 'dolphin', 'drowned', 'phantom',
   'turtle']])

RENAMED_I = {
    'red_dye': 'pre-1.0', 'yellow_dye': 'pre-1.0', 'green_dye': 'pre-1.0',  # rose_red/dandelion_yellow/cactus_green (1.14)
    'zombified_piglin_spawn_egg': '1.1',                                   # zombie_pigman_spawn_egg (1.16)
    'turtle_scute': '1.13',                                                # scute (1.20.5)
    # 26.3 split explorer maps off filled_map; they keep the date the map variant first existed
    'ocean_monument_map': '1.11', 'woodland_mansion_map': '1.11',          # ocean/woodland explorer maps
    'buried_treasure_map': '1.13',
    'buried_trial_chambers_map': '1.21',                                   # trial explorer map (exp. 1.20.5)
    'jungle_pyramid_map': '1.21.5', 'swamp_hut_map': '1.21.5',             # trade-rebalance maps, default in 25w07a
    **{v + '_village_map': '1.21.5' for v in ['desert', 'plains', 'savanna', 'snowy', 'taiga']},
}
EXP_DEFAULT_OK['item'] = {
    'ender_dragon_spawn_egg', 'iron_golem_spawn_egg', 'snow_golem_spawn_egg', 'wither_spawn_egg',  # 1.19.3, default
    'armadillo_scute', 'armadillo_spawn_egg', 'wolf_armor',                                        # 1.20.5, default
    'bordure_indented_banner_pattern', 'field_masoned_banner_pattern',                             # 1.21.2, default
    *[c + '_bundle' for c in COLORS],                                                              # 1.21.2, default
}

def item_value(i, blocks):
    if i in RENAMED_I:
        return RENAMED_I[i], 'rename/special'
    if 'minecraft:' + i in blocks:
        return blocks['minecraft:' + i], 'block item'
    f = first('item', i)
    if f is None:
        raise SystemExit('no first for item ' + i)
    if f == '1.13':
        return I.get(i, 'pre-1.0'), 'pre-1.13 table' + ('' if i in I else ' (default pre-1.0)')
    if f in EXP and i not in EXP_DEFAULT_OK['item']:
        return EXP[f], 'experimental in %s' % f
    return f, 'registry diff'

def block_value(i):
    if i in RENAMED_B:
        return RENAMED_B[i], 'rename/special'
    f = first('block', i)
    if f is None:
        raise SystemExit('no first for ' + i)
    if f == '1.13':
        return B.get(i, 'pre-1.0'), 'pre-1.13 table' + ('' if i in B else ' (default pre-1.0)')
    if f in EXP and i not in EXP_DEFAULT_OK['block']:
        return EXP[f], 'experimental in %s' % f
    return f, 'registry diff'

def entity_value(i):
    if i in E:
        return E[i], 'manual'
    f = first('entity_type', i)
    if f is None:
        raise SystemExit('no first for entity ' + i)
    if f == '1.14':
        raise SystemExit('entity in 1.14 baseline not classified: ' + i)
    if f in EXP and i not in EXP_DEFAULT_OK['entity_type']:
        return EXP[f], 'experimental in %s' % f
    return f, 'registry diff'

blocks, ents, why = {}, {}, {}
for i in CUR_B:
    v, w = block_value(i); blocks['minecraft:' + i] = v; why['b:' + i] = w
for i in CUR_E:
    v, w = entity_value(i); ents['minecraft:' + i] = v; why['e:' + i] = w
items = {}
for i in CUR_I:
    v, w = item_value(i, blocks); items['minecraft:' + i] = v; why['i:' + i] = w

# sanity
valid = set(VERSIONS) | {'pre-1.0'}
assert all(v in valid for v in blocks.values()), [k for k, v in blocks.items() if v not in valid]
assert all(v in valid for v in ents.values())
assert all(v in valid for v in items.values()), [k for k, v in items.items() if v not in valid]
assert len(items) == len(CUR_I) == len(set(CUR_I))
unused = set(B) - set(sets['block']['1.13'])
print('B keys not in 1.13:', unused)
print('E keys not in 26.3:', set(E) - set(CUR_E))
print('I keys not in 1.13 items:', set(I) - set(sets['item']['1.13']))
print('I keys that are blocks:', {i for i in I if 'minecraft:' + i in blocks})
print('RENAMED_I keys not in 26.3:', set(RENAMED_I) - set(CUR_I))
print('EXP_DEFAULT_OK items not in an EXP version:',
      {i for i in EXP_DEFAULT_OK['item'] if first('item', i) not in EXP})
out = {'versions': VERSIONS, 'blocks': blocks, 'entities': ents, 'items': items}
json.dump(out, open('/home/jp/projs/wwia/data/added.json', 'w'), indent=2)
json.dump(why, open(W + 'why.json', 'w'), indent=1)
from collections import Counter
order = {v: n for n, v in enumerate(['pre-1.0'] + VERSIONS)}
cb, ce, ci = Counter(blocks.values()), Counter(ents.values()), Counter(items.values())
print(len(blocks), 'blocks', len(ents), 'entities', len(items), 'items')
for v in sorted(set(cb) | set(ce) | set(ci), key=lambda v: order[v]):
    print('%-8s blocks=%4d entities=%3d items=%4d' % (v, cb[v], ce[v], ci[v]))
