use spacetimedb::SpacetimeType;
// ========== MATH TYPES ==========

#[derive(SpacetimeType, Clone, Copy, Debug, PartialEq)]
pub struct DbVector2 {
    pub x: f32,
    pub y: f32,
}

impl DbVector2 {
    pub fn new(x: f32, y: f32) -> Self {
        DbVector2 { x, y }
    }

    pub fn normalize(&self) -> DbVector2 {
        let d2 = self.x * self.x + self.y * self.y;
        if d2 > 0.0 {
            let inv_mag = 1.0 / d2.sqrt();
            DbVector2::new(self.x * inv_mag, self.y * inv_mag)
        } else {
            DbVector2::new(0.0, 0.0)
        }
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn length_sq(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }
}

impl std::ops::Add for DbVector2 {
    type Output = DbVector2;

    fn add(self, other: DbVector2) -> DbVector2 {
        DbVector2::new(self.x + other.x, self.y + other.y)
    }
}

impl std::ops::Sub for DbVector2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Mul<f32> for DbVector2 {
    type Output = DbVector2;

    fn mul(self, scalar: f32) -> DbVector2 {
        DbVector2::new(self.x * scalar, self.y * scalar)
    }
}

// ========== ENUMS ==========

#[derive(SpacetimeType, Clone, Copy, Debug, PartialEq)]
pub enum ShipType {
    // Starter ships (0-2 units of a trait)
    Raft,              // Default starter
    ThousandSunny,     // Straw Hat Pirates
    RedForce,          // Red Hair Pirates
    Naglfar,           // Giants
    SaberOfXebec,      //Blackbeared
    PolarTang,         //Hear Pirates
    BigTopBlaster,     //Cross guid
    MobyDick,          //Whitebeared Pirates
    QueenMama,         //Big mom Pirates
}

impl ShipType {
    /// Get asset filename for this ship
    pub fn asset_filename(&self) -> &'static str {
        match self {
            ShipType::Raft => "Raft.png",
            ShipType::RedForce => "RedForce.png",
            ShipType::ThousandSunny => "Sunny.png",
            ShipType::Naglfar => "Naglfar.png",
            ShipType::SaberOfXebec => "SaberOfXebec.png",
            ShipType::PolarTang => "PolarTang.png",
            ShipType::BigTopBlaster => "BigTopBlaster.png",
            ShipType::MobyDick => "MobyDick.png",
            ShipType::QueenMama => "QueenMama.png",
        }
    }

    /// Get display name for this ship
    pub fn display_name(&self) -> &'static str {
        match self {
            ShipType::Raft => "Raft",
            ShipType::RedForce => "Red Force",
            ShipType::ThousandSunny => "Thousand Sunny",
            ShipType::Naglfar => "Naglfar",
            ShipType::SaberOfXebec => "Saber Of Xebec",
            ShipType::PolarTang => "Polar Tang",
            ShipType::BigTopBlaster => "Big Top Blaster",
            ShipType::MobyDick => "Moby Dick",
            ShipType::QueenMama => "Queen Mama",
        }
    }
}

#[derive(SpacetimeType, Clone, Copy, Debug, PartialEq)]
pub enum CrewRarity {
    Common,      // Green 1 Gold
    Uncommon,    // Blue 2 Gold
    Rare,        // Yellow 3 Gold
    Epic,        // Purple 4 Gold
    Legendary,   // Gold 5 Gold
}

#[derive(SpacetimeType, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CrewTrait {
    StrawHat,
    Revolutionary,
    RedHairPirates,
    Giants,
    HolyKnights,
    Gorosei,
    BlackbearedPirates,
    WhitebearedPirates,
    BigMomPirates,
    HeartPirates,
    CrossGuildPirates,

    Logia,
    Paramecia,
    Zoan,
    Swordsman,
    Brawler,
    Sniper,
    Emperor,
}
impl CrewTrait {
    pub fn ship_trait(self) -> Option<ShipType> {
        match self {
            CrewTrait::StrawHat => Some(ShipType::ThousandSunny),
            CrewTrait::RedHairPirates => Some(ShipType::RedForce),
            CrewTrait::Giants => Some(ShipType::Naglfar),
            CrewTrait::BlackbearedPirates => Some(ShipType::SaberOfXebec),
            CrewTrait::HeartPirates => Some(ShipType::PolarTang),
            CrewTrait::WhitebearedPirates => Some(ShipType::MobyDick),
            CrewTrait::BigMomPirates => Some(ShipType::QueenMama),
            CrewTrait::CrossGuildPirates => Some(ShipType::BigTopBlaster),

            // Non-ship traits
            CrewTrait::Swordsman
            | CrewTrait::Brawler
            | CrewTrait::Logia
            | CrewTrait::Paramecia
            | CrewTrait::Zoan
            | CrewTrait::Revolutionary
            | CrewTrait::HolyKnights
            | CrewTrait::Gorosei
            | CrewTrait::Sniper
            | CrewTrait::Emperor => None,
        }
    }

    pub fn is_ship_defining(self) -> bool {
        self.ship_trait().is_some()
    }
}

#[derive(SpacetimeType, Clone, Copy, Debug, PartialEq)]
pub enum BattleStatus {
    WaitingForOpponent,
    InProgress,
    Finished,
}
#[derive(SpacetimeType, Clone, Copy, Debug, PartialEq)]
pub enum ItemComponent {
    Sword,      // +4 AD
    Ring,       // +5 AP
    Gloves,     // +10% Attack Speed
}

#[derive(SpacetimeType, Clone, Copy, Debug, PartialEq)]
pub enum CompletedItem {
    // Sword + Sword → Yooru
    // +15 AD
    // Effect: Every 3rd attack deals 50 damage to ALL enemies
    Yooru,

    // Gloves + Gloves → Kabuto
    // +30% Attack Speed, +5 AP
    // Effect: 25% chance to fire 3 rapid shots (ranged units only)
    Kabuto,

    // Gloves + Sword → Shusui
    // +10 AD, +15% Attack Speed
    // Effect: Every 4th attack deals 200% damage
    Shusui,

    // Ring + Ring → RingRing
    // +13 AP
    // Effect: Begin battle: Launch 10 damage fireball at random enemy
    RingRing,

    // Ring + Sword → 10T Hammer
    // +10 AD, +10 AP
    // Effect: 10% chance to stun target for 1.5 seconds
    TenTonHammer,

    // Ring + Gloves → Impact Dial
    // +10 AP, +15% Attack Speed
    // Effect: Begin battle: Deal 15 damage to nearest enemy
    ImpactDial,
}

// Item enum - can be either component or completed
#[derive(SpacetimeType, Clone, Copy, Debug, PartialEq)]
pub enum Item {
    Component(ItemComponent),
    Completed(CompletedItem),
}

impl Item {
    pub fn is_component(&self) -> bool {
        matches!(self, Item::Component(_))
    }

    pub fn get_name(&self) -> &'static str {
        match self {
            Item::Component(ItemComponent::Sword) => "Sword",
            Item::Component(ItemComponent::Ring) => "Ring",
            Item::Component(ItemComponent::Gloves) => "Gloves",
            Item::Completed(CompletedItem::Yooru) => "Yooru",
            Item::Completed(CompletedItem::Kabuto) => "Kabuto",
            Item::Completed(CompletedItem::Shusui) => "Shusui",
            Item::Completed(CompletedItem::RingRing) => "RingRing",
            Item::Completed(CompletedItem::TenTonHammer) => "10T Hammer",
            Item::Completed(CompletedItem::ImpactDial) => "Impact Dial",
        }
    }

    pub fn get_description(&self) -> &'static str {
        match self {
            Item::Component(ItemComponent::Sword) => "+4 AD",
            Item::Component(ItemComponent::Ring) => "+5 AP",
            Item::Component(ItemComponent::Gloves) => "+10% ATK Speed",
            Item::Completed(CompletedItem::Yooru) =>
                "+15 AD\nMihawk's legendary black blade.\nEvery 3rd attack: Deal 50 damage to ALL enemies.",
            Item::Completed(CompletedItem::Kabuto) =>
                "+30% ATK Speed, +5 AP\nUsopp's enhanced slingshot.\n25% chance to fire 3 rapid shots.",
            Item::Completed(CompletedItem::Shusui) =>
                "+10 AD, +15% ATK Speed\nZoro's legendary cursed sword.\nEvery 4th attack: Deal 200% damage.",
            Item::Completed(CompletedItem::RingRing) =>
                "+13 AP\nBegin: Launch a 10 dmg fireball at a random enemy.",
            Item::Completed(CompletedItem::TenTonHammer) =>
                "+10 AD, +10 AP\nA devastating 10 ton hammer.\n10% chance to stun target for 1.5s.",
            Item::Completed(CompletedItem::ImpactDial) =>
                "+10 AP, +15% ATK Speed\nBegin: Deal 15 damage to nearest enemy.",
        }
    }
}

#[derive(SpacetimeType, Clone, Copy, PartialEq)]
pub enum LocationType {
    Start,
    End,
    PVECombat,
    PVPCombat,
    TreasureIsland,
}

#[derive(SpacetimeType, Clone, Copy, PartialEq)]
pub enum EnemyType {
    MarineSwordsman,
    MarineRifle,
    Parcifista,
    Smoker,
    Korby,
    Garp,
    Kizaru
}

#[derive(SpacetimeType, Clone)]
pub struct EnemySpawn {
    pub enemy_id: u64,      // Reference to Enemy table
    pub position_x: u32,
    pub position_y: u32,
}

// ========== BATTLE CONSTANTS ==========

pub const BATTLE_ARENA_SIZE: f32 = 1600.0; // 1600x1600 battle arena
pub const BATTLE_TICK_RATE: u32 = 20; // 20 ticks per second (50ms per tick)
pub const DELTA_TIME: f32 = 1.0 / BATTLE_TICK_RATE as f32; // 0.05 seconds per tick

// Spatial hash grid constants
pub const GRID_CELL_SIZE: u16 = 200;
pub const GRID_WIDTH: u16 = 8; // 1600 / 200 = 8
pub const GRID_HEIGHT: u16 = 8;
pub const NUM_GRID_CELLS: u16 = GRID_WIDTH * GRID_HEIGHT; // 64 cells