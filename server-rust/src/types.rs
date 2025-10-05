use spacetimedb::SpacetimeType;

// ========== ENUMS ==========

#[derive(SpacetimeType, Clone, Copy, Debug, PartialEq)]
pub enum ShipType {
    Raft,
    GoingMerry,
    ThousandSunny,
}

#[derive(SpacetimeType, Clone, Copy, Debug, PartialEq)]
pub enum CrewRarity {
    Common,      // Bronze
    Rare,        // Silver
    Epic,        // Gold
    Legendary,   // Rainbow
}

#[derive(SpacetimeType, Clone, Copy, Debug, PartialEq)]
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
