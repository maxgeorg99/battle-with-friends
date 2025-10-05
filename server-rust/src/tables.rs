use spacetimedb::Identity;
use crate::types::*;

// ========== TABLES ==========

#[spacetimedb::table(name = player, public)]
pub struct Player {
    #[primary_key]
    pub identity: Identity,
    pub name: String,
    pub berries: u32,        // Currency
    pub bounty: u32,         // Win streak multiplier
    pub wins: u32,
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
    pub bounty_reward: u32, // Bounty claimed from loser
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
