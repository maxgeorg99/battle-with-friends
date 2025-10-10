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

    // Tier 1 ships (3-5 units of a trait)
    FlyingLamb,        // Straw Hat Pirates (Going Merry equivalent)
    MarineShip,        // Marines
    RevolutionaryShip, // Revolutionary Army
    GiantShip,         // Giants
    RedForce,          // Red Hair Pirates

    // Tier 2 ships (6+ units of a trait)
    ThousandSunny,     // Straw Hat Pirates (upgraded)
    GarpsShip,         // Marines (upgraded)
    RevolutionaryBattleship, // Revolutionary Army (upgraded)
    GiantWarship,      // Giants (upgraded)
    RedForceUpgraded,  // Red Hair Pirates (upgraded)
}

impl ShipType {
    /// Get ship based on active trait and trait level
    pub fn from_trait_and_level(active_trait: Option<CrewTrait>, trait_level: u32) -> Self {
        match (active_trait, trait_level) {
            // No trait or low level = Raft
            (None, _) | (_, 0..=2) => ShipType::Raft,

            // Tier 1: 3-5 units
            (Some(CrewTrait::StrawHat), 3..=5) => ShipType::FlyingLamb,
            (Some(CrewTrait::Marine), 3..=5) => ShipType::MarineShip,
            (Some(CrewTrait::Revolutionary), 3..=5) => ShipType::RevolutionaryShip,
            (Some(CrewTrait::Giants), 3..=5) => ShipType::GiantShip,
            (Some(CrewTrait::RedHairPirates), 3..=5) => ShipType::RedForce,

            // Tier 2: 6+ units
            (Some(CrewTrait::StrawHat), 6..) => ShipType::ThousandSunny,
            (Some(CrewTrait::Marine), 6..) => ShipType::GarpsShip,
            (Some(CrewTrait::Revolutionary), 6..) => ShipType::RevolutionaryBattleship,
            (Some(CrewTrait::Giants), 6..) => ShipType::GiantWarship,
            (Some(CrewTrait::RedHairPirates), 6..) => ShipType::RedForceUpgraded,

            // Other traits default to generic ships
            (Some(_), 3..=5) => ShipType::FlyingLamb,
            (Some(_), 6..) => ShipType::ThousandSunny,
        }
    }

    /// Get max crew slots for this ship
    pub fn max_crew_size(&self) -> u8 {
        match self {
            ShipType::Raft => 5,

            // Tier 1 ships: 10 crew slots
            ShipType::FlyingLamb
            | ShipType::MarineShip
            | ShipType::RevolutionaryShip
            | ShipType::GiantShip
            | ShipType::RedForce => 10,

            // Tier 2 ships: 15 crew slots
            ShipType::ThousandSunny
            | ShipType::GarpsShip
            | ShipType::RevolutionaryBattleship
            | ShipType::GiantWarship
            | ShipType::RedForceUpgraded => 15,
        }
    }

    /// Get asset filename for this ship
    pub fn asset_filename(&self) -> &'static str {
        match self {
            ShipType::Raft => "raft.png", // You'll need to add this
            ShipType::FlyingLamb => "FlyingLamb.jpeg",
            ShipType::MarineShip => "MarineShip.jpeg",
            ShipType::RevolutionaryShip => "MarineShip.jpeg", // TODO: Add revolutionary ship asset
            ShipType::GiantShip => "MarineShip.jpeg", // TODO: Add giant ship asset
            ShipType::RedForce => "RedForce.jpeg",
            ShipType::ThousandSunny => "sunny.jpeg",
            ShipType::GarpsShip => "GarpsShip.jpeg",
            ShipType::RevolutionaryBattleship => "GarpsShip.jpeg", // TODO: Add revolutionary battleship
            ShipType::GiantWarship => "GarpsShip.jpeg", // TODO: Add giant warship
            ShipType::RedForceUpgraded => "RedForce.jpeg", // Could be same or different
        }
    }

    /// Get display name for this ship
    pub fn display_name(&self) -> &'static str {
        match self {
            ShipType::Raft => "Raft",
            ShipType::FlyingLamb => "Going Merry (Flying Lamb)",
            ShipType::MarineShip => "Marine Warship",
            ShipType::RevolutionaryShip => "Revolutionary Ship",
            ShipType::GiantShip => "Giant Longship",
            ShipType::RedForce => "Red Force",
            ShipType::ThousandSunny => "Thousand Sunny",
            ShipType::GarpsShip => "Garp's Battleship",
            ShipType::RevolutionaryBattleship => "Revolutionary Battleship",
            ShipType::GiantWarship => "Giant Warship",
            ShipType::RedForceUpgraded => "Red Force (Upgraded)",
        }
    }
}

#[derive(SpacetimeType, Clone, Copy, Debug, PartialEq)]
pub enum ShipUpgradeType {
    // Stat boosts
    BonusGold,          // +1 gold per round
    BonusHealth,        // +10 HP to all units
    BonusAttack,        // +2 attack to all units
    BonusDefense,       // +2 defense to all units
    BonusSpeed,         // +10% attack speed to all units

    // Trait-specific boosts
    StrawHatBuff,       // StrawHat units get +20% stats
    MarineBuff,         // Marine units get +20% stats
    RevolutionaryBuff,  // Revolutionary units get +20% stats
    LogiaBuff,          // Logia units gain shield
    ParameciaBuff,      // Paramecia units gain lifesteal
    ZoanBuff,           // Zoan units gain +50 HP

    // Special effects
    GamblerLuck,        // Rerolls cost 1 gold
    FastLearner,        // Units gain +50% XP
    Plunderer,          // Win streak gives extra gold
    Medic,              // Units heal 5 HP per second
    Arsenal,            // Units start with +1 item slot
}

#[derive(SpacetimeType, Clone, Copy, Debug, PartialEq)]
pub enum CrewRarity {
    Common,      // Bronze
    Rare,        // Silver
    Epic,        // Gold
    Legendary,   // Rainbow
}

#[derive(SpacetimeType, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CrewTrait {
    StrawHat,
    Marine,
    Revolutionary,
    RedHairPirates,
    Giants,
    HolyKnights,
    FiveElders,
    Logia,
    Paramecia,
    Zoan,
    Sword,
    DFUser, // Devil Fruit User
}

#[derive(SpacetimeType, Clone, Copy, Debug, PartialEq)]
pub enum BattleStatus {
    WaitingForOpponent,
    InProgress,
    Finished,
}

#[derive(SpacetimeType, Clone, Copy, Debug, PartialEq)]
pub enum ItemComponent {
    Cutlass,           // +AD
    SniperGoggles,     // +Crit Chance
    ShellDial,         // +AS (Attack Speed)
    ToneDial,          // +AP (Ability Power)
    SeastoneFragment,  // +Armor
    TidalCloak,        // +MR (Magic Resist)
    EnergyDrink,       // +Starting Mana
    Meat,              // +HP
}

#[derive(SpacetimeType, Clone, Copy, Debug, PartialEq)]
pub enum CompletedItem {
    // Damage / AD Focus
    Yoru,              // Cutlass + SniperGoggles → +75% crit damage
    Kabuto,            // Cutlass + ShellDial → Attacks deal splash
    Shusui,            // Cutlass + SeastoneFragment → Bonus AD + armor shred

    // AP Focus
    ClimaTact,         // ToneDial + ToneDial → Doubles AP
    ThunderTempo,      // ToneDial + ShellDial → AP + attack speed
    MirageFlower,      // ToneDial + EnergyDrink → AP + starting mana

    // Tank Focus
    AdamWood,          // SeastoneFragment + SeastoneFragment → Massive armor
    SeaKingScale,      // SeastoneFragment + TidalCloak → Armor + MR
    ThousandSunnyHull, // SeastoneFragment + Meat → Armor + HP

    // Utility Focus
    VivrCard,          // EnergyDrink + TidalCloak → Mana + survivability
    LogPose,           // SniperGoggles + EnergyDrink → Crit + mana
    Poneglyph,         // ToneDial + SeastoneFragment → AP + armor

    // Hybrid
    GumGumFruit,       // Cutlass + Meat → AD + HP
    GomuGomuNoMi,      // Meat + Meat → Massive HP regeneration
    HakiMastery,       // ToneDial + Cutlass → AD + AP hybrid
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

// ========== TREASURE ISLAND CONSTANTS ==========

pub const TREASURE_ISLAND_ROUNDS: [u32; 3] = [5, 15, 25]; // Rounds when treasure islands appear

#[derive(SpacetimeType, Clone, Copy, Debug, PartialEq)]
pub enum TreasureRewardType {
    Gold(u32),           // Berries
    Item(ItemComponent), // Random item component
    Reroll,              // Free shop reroll
    Experience,          // Bonus XP for units (future)
}

#[derive(SpacetimeType, Clone, Copy, Debug, PartialEq)]
pub enum TreasureIslandType {
    SmallIsland,   // 1 reward (rounds 5)
    MediumIsland,  // 2 rewards (rounds 15)
    LargeIsland,   // 3 rewards (rounds 25)
}
