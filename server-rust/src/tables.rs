use spacetimedb::{Identity, ScheduleAt};
use crate::types::*;
use crate::systems::battle::battle_tick;

// ========== TABLES ==========

#[spacetimedb::table(name = player, public)]
pub struct Player {
    #[primary_key]
    pub identity: Identity,
    pub name: String,
    pub berries: u32,        // Currency
    pub bounty: u32,         // Bounty increases by 100k per win, reset to 0 on loss
    pub wins: u32,
    pub losses: u32,
    pub ship_type: ShipType,
    pub online: bool,

    // Ship upgrade system
    pub fights_completed: u32,           // Total fights completed
    pub active_trait: Option<CrewTrait>, // Highest level trait (determines ship)
    pub trait_level: u32,                // Level of active trait (unit count)
}

#[spacetimedb::table(name = crew, public)]
pub struct Crew {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    #[index(btree)]
    pub owner: Identity,
    pub name: String,
    pub rarity: CrewRarity,
    pub trait1: CrewTrait,
    pub trait2: Option<CrewTrait>,
    pub max_hp: u32,
    pub current_hp: u32,
    pub attack: u32,
    pub defense: u32,
    pub level: u8,
    pub slot_index: Option<u8>, // 0-14 on ship (field), None = bench/inventory
    pub item1: Option<ItemComponent>,
    pub item2: Option<ItemComponent>,
    pub completed_item: Option<CompletedItem>,
}

#[spacetimedb::table(name = shop_crew, public)]
pub struct ShopCrew {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    #[index(btree)]
    pub player: Identity,
    pub name: String,
    pub rarity: CrewRarity,
    pub trait1: CrewTrait,
    pub trait2: Option<CrewTrait>,
    pub max_hp: u32,
    pub attack: u32,
    pub defense: u32,
    pub cost: u32,
}

#[spacetimedb::table(name = battle, public)]
pub struct Battle {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub player1: Identity,
    pub player2: Option<Identity>,
    pub winner: Option<Identity>,
    #[index(btree)]
    pub status: BattleStatus,
    pub turn: u32,
    pub bounty_reward: u32,      // Bounty claimed from loser (set when battle ends)
    pub player1_bounty: u32,     // Player1's bounty at battle start
    pub player2_bounty: u32,     // Player2's bounty at battle start
}

#[spacetimedb::table(name = player_item, public)]
pub struct PlayerItem {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    #[index(btree)]
    pub owner: Identity,
    pub component: ItemComponent,
}

// Static crew template database - initialized once on server init
#[spacetimedb::table(name = crew_template, public)]
pub struct CrewTemplate {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub name: String,
    pub rarity: CrewRarity,
    pub trait1: CrewTrait,
    pub trait2: Option<CrewTrait>,
    pub max_hp: u32,
    pub attack: u32,
    pub defense: u32,
    pub cost: u32,
}

// Static item component stats - initialized once on server init
#[spacetimedb::table(name = item_component_stats, public)]
pub struct ItemComponentStats {
    #[primary_key]
    pub component: ItemComponent,
    pub name: String,
    pub description: String,
    pub bonus_ad: i32,           // Attack Damage
    pub bonus_crit_chance: f32,  // Crit Chance (0-1)
    pub bonus_attack_speed: f32, // Attack Speed multiplier
    pub bonus_ap: i32,           // Ability Power
    pub bonus_armor: i32,        // Armor
    pub bonus_mr: i32,           // Magic Resist
    pub bonus_mana: i32,         // Starting Mana
    pub bonus_hp: i32,           // Health Points
}

// Item combination recipes - initialized once on server init
#[spacetimedb::table(name = item_combination_recipe, public)]
pub struct ItemCombinationRecipe {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub component1: ItemComponent,
    pub component2: ItemComponent,
    pub result: CompletedItem,
}

// Static completed item stats - initialized once on server init
#[spacetimedb::table(name = completed_item_stats, public)]
pub struct CompletedItemStats {
    #[primary_key]
    pub item: CompletedItem,
    pub name: String,
    pub description: String,
    pub bonus_ad: i32,
    pub bonus_crit_chance: f32,
    pub bonus_crit_damage: f32,  // Crit Damage multiplier
    pub bonus_attack_speed: f32,
    pub bonus_ap: i32,
    pub bonus_armor: i32,
    pub bonus_mr: i32,
    pub bonus_mana: i32,
    pub bonus_hp: i32,
    pub bonus_hp_regen: f32,     // HP regeneration per second
    pub has_splash: bool,        // Attacks deal splash damage
    pub armor_shred: i32,        // Reduces enemy armor
}

// BattleUnit - represents a crew member actively fighting in a battle
#[spacetimedb::table(name = battle_unit, public)]
pub struct BattleUnit {
    #[primary_key]
    #[auto_inc]
    pub id: u64,

    #[index(btree)]
    pub battle_id: u64,          // Which battle this unit belongs to

    pub crew_id: u64,            // Reference to Crew table for base stats
    pub owner: Identity,         // Player who owns this unit
    pub side: u8,                // 0 = player1, 1 = player2

    // Battle position and movement
    pub position: DbVector2,
    pub velocity: DbVector2,
    pub radius: f32,             // Collision radius

    // Combat stats (copied from Crew + items at battle start)
    pub max_hp: u32,
    pub current_hp: u32,
    pub attack: u32,             // Physical attack damage
    pub defense: u32,            // Physical defense
    pub ability_power: u32,      // Magical/ability damage
    pub magic_resist: u32,       // Magical defense
    pub attack_speed: f32,       // Attacks per second (base 1.0)
    pub crit_chance: f32,        // Critical hit chance (0.0 - 1.0)
    pub crit_damage: f32,        // Critical damage multiplier (default 1.5)

    // Mana system for abilities
    pub max_mana: u32,
    pub current_mana: u32,
    pub mana_per_attack: u32,    // Mana gained per basic attack

    // Attack state
    pub attack_cooldown: f32,    // Frames until next attack (0 = can attack)
    pub target_unit_id: Option<u64>, // Currently targeted enemy unit

    // Abilities (based on crew traits)
    pub ability_ready: bool,
    pub ability_cooldown: f32,

    // Status effects
    pub is_stunned: bool,
    pub stun_duration: f32,
}

// Battle tick timer - scheduled reducer to run battle simulation
#[spacetimedb::table(name = battle_tick_timer, scheduled(battle_tick), public)]
pub struct BattleTickTimer {
    #[primary_key]
    #[auto_inc]
    pub scheduled_id: u64,
    pub scheduled_at: ScheduleAt,
    pub battle_id: u64,
}

// Static ship upgrade definitions (initialized once on server start)
#[spacetimedb::table(name = ship_upgrade_data, public)]
pub struct ShipUpgradeData {
    #[primary_key]
    #[auto_inc]
    pub id: u64,

    #[unique]
    pub upgrade_type: ShipUpgradeType,
    pub name: String,
    pub description: String,
    pub rarity: u8, // 1 = common, 2 = uncommon, 3 = rare (affects drop rate)
}

// Ship upgrades owned by player (like TFT augments)
#[spacetimedb::table(name = player_ship_upgrade, public)]
pub struct PlayerShipUpgrade {
    #[primary_key]
    #[auto_inc]
    pub id: u64,

    #[index(btree)]
    pub owner: Identity,
    pub upgrade_type: ShipUpgradeType,
    pub acquired_at_fight: u32, // Which fight number this was acquired
}

// Shop offers for ship upgrades (Franky's Workshop every 10 fights)
#[spacetimedb::table(name = ship_upgrade_offer, public)]
pub struct ShipUpgradeOffer {
    #[primary_key]
    #[auto_inc]
    pub id: u64,

    #[index(btree)]
    pub player: Identity,
    pub upgrade_type: ShipUpgradeType,
    pub fight_number: u32, // Which fight this offer is for
}

// Treasure Island encounters (rounds 5, 15, 25)
#[spacetimedb::table(name = treasure_island, public)]
pub struct TreasureIsland {
    #[primary_key]
    #[auto_inc]
    pub id: u64,

    #[index(btree)]
    pub player: Identity,
    pub round_number: u32,
    pub island_type: TreasureIslandType,
    pub claimed: bool,
}

// Treasure rewards available for claiming
#[spacetimedb::table(name = treasure_reward, public)]
pub struct TreasureReward {
    #[primary_key]
    #[auto_inc]
    pub id: u64,

    #[index(btree)]
    pub treasure_island_id: u64,
    pub reward_type: TreasureRewardType,
    pub claimed: bool,
}
