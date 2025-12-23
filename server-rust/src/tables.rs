use spacetimedb::{Identity, Timestamp};
use crate::types::*;

// ========== TABLES ==========

#[spacetimedb::table(name = player, public)]
pub struct Player {
    #[primary_key]
    pub identity: Identity,
    pub name: String,
    pub berries: u32,        // Currency
    pub xp: u8,
    pub level: u8,
    pub hp: u8,
    pub bounty: u32,         // Bounty increases by 100k per win, reset to 0 on loss
    pub wins: u32,
    pub win_streak: u32,
    pub losses: u32,
    pub ship_type: ShipType,
    pub online: bool,
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
    pub traits: Vec<CrewTrait>,
    pub max_hp: u32,
    pub ability_power: u32,
    pub attack: u32,
    pub attack_speed: f32,
    pub defense: u32,
    pub magic_resistance: u32,
    pub level: u8,
    pub slot_index: Option<u8>, // 0-28 on ship/field
    pub bench_index: Option<u8>, // 0-10 on bench
    pub item1: Option<Item>,
    pub item2: Option<Item>,
    pub item3: Option<Item>,
}

// Player's item inventory - items not equipped to any crew
#[spacetimedb::table(name = player_item, public)]
pub struct PlayerItem {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    #[index(btree)]
    pub owner: Identity,
    pub item: Item,
    pub bench_slot: Option<u8>, // Optional slot index for organizing items in treasure chest
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
    pub traits: Vec<CrewTrait>,
    pub max_hp: u32,
    pub ability_power: u32,
    pub attack: u32,
    pub attack_speed: f32,
    pub defense: u32,
    pub magic_resistance: u32,
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

// Static crew template database - initialized once on server init
#[spacetimedb::table(name = crew_template, public)]
pub struct CrewTemplate {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub name: String,
    pub rarity: CrewRarity,
    pub traits: Vec<CrewTrait>,
    pub max_hp: u32,
    pub ability_power: u32,
    pub attack: u32,
    pub attack_speed: f32,
    pub defense: u32,
    pub magic_resistance: u32,
    pub cost: u32,
}

// Static pve unit database - initialized once on server init
#[spacetimedb::table(name = enemy, public)]
pub struct Enemy {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub enemy_type: EnemyType,
    pub max_hp: u32,
    pub attack: u32,
    pub defense: u32,
}

#[spacetimedb::table(name = journey, public)]
pub struct Journey {
    #[primary_key]
    pub id: u64,
    pub player_id: Identity,
    pub current_location: Option<u64>,
    //meta data
    pub created_at: Timestamp,
    pub seed: u64,
}

#[spacetimedb::table(name = location, public)]
pub struct Location {
    #[primary_key]
    pub id: u64,
    pub journey_id: u64, // Foreign key to MapInstance
    pub floor: u32,
    pub position_x: u32,
    pub location_type: LocationType,
    pub is_visited: bool,
    pub is_available: bool,

    // Optional fields based on location_type
    pub treasure_items: Option<Vec<Item>>,
    pub pvp_opponent: Option<Identity>,
}

#[spacetimedb::table(name = pve_combat, public)]
pub struct PveCombat {
    #[primary_key]
    pub location_id: u64,
    pub enemies: Vec<EnemySpawn>,
    pub reward_items: Vec<Item>,
    pub reward_gold: u32,
}