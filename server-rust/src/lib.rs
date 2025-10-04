use spacetimedb::{ReducerContext, Identity, Table, SpacetimeType, Timestamp, rand::Rng};

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
    Sword
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
    pub slot_index: Option<u8>, // 0-14 on ship (field), None = bench/inventory TODO we should use seprate index for bench and ship
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

// ========== REDUCERS ==========

#[spacetimedb::reducer]
pub fn register_player(ctx: &ReducerContext, name: String) -> Result<(), String> {
    let identity = ctx.sender;

    if ctx.db.player().identity().find(identity).is_some() {
        return Err("Player already registered".to_string());
    }

    ctx.db.player().insert(Player {
        identity,
        name,
        berries: 1000000, // Start with 1,000,000 berries
        bounty: 0,
        wins: 0,
        losses: 0,
        ship_type: ShipType::Raft,
        online: true,
    });

    // Give starter crew: Luffy
    ctx.db.crew().insert(Crew {
        id: 0,
        owner: identity,
        name: "Monkey D. Luffy".to_string(),
        rarity: CrewRarity::Legendary,
        trait1: CrewTrait::StrawHat,
        trait2: Some(CrewTrait::DFUser),
        max_hp: 100,
        current_hp: 100,
        attack: 25,
        defense: 10,
        level: 1,
        slot_index: Some(0),
        item1: None,
        item2: None,
        completed_item: None,
    });

    // Give starter items
    ctx.db.player_item().insert(PlayerItem {
        id: 0,
        owner: identity,
        component: ItemComponent::Cutlass,
    });
    ctx.db.player_item().insert(PlayerItem {
        id: 0,
        owner: identity,
        component: ItemComponent::Meat,
    });

    // Initialize shop with random crew
    refresh_shop(ctx)?;

    Ok(())
}

#[spacetimedb::reducer]
pub fn refresh_shop(ctx: &ReducerContext) -> Result<(), String> {
    let identity = ctx.sender;

    // Clear old shop
    for shop_crew in ctx.db.shop_crew().player().filter(&identity) {
        ctx.db.shop_crew().id().delete(shop_crew.id);
    }

    //This should totally be a table we initialize once.
    // Crew templates (name, rarity, trait1, trait2, hp, attack, defense, cost in 100k berries)
    let crew_pool: Vec<(&str, CrewRarity, CrewTrait, Option<CrewTrait>, u32, u32, u32, u32)> = vec![
        // Straw Hats
        ("Roronoa Zoro", CrewRarity::Rare, CrewTrait::StrawHat, CrewTrait::Sword, 80, 20, 8, 300000),
        ("Nami", CrewRarity::Common, CrewTrait::StrawHat, None, 50, 12, 6, 200000),
        ("Usopp", CrewRarity::Common, CrewTrait::StrawHat, None, 60, 15, 5, 200000),
        ("Sanji", CrewRarity::Rare, CrewTrait::StrawHat, None, 75, 18, 10, 300000),
        ("Tony Tony Chopper", CrewRarity::Common, CrewTrait::StrawHat, Some(CrewTrait::Zoan), 55, 14, 7, 200000),
        ("Nico Robin", CrewRarity::Rare, CrewTrait::StrawHat, Some(CrewTrait::Paramecia), 70, 16, 9, 300000),
        ("Franky", CrewRarity::Rare, CrewTrait::StrawHat, None, 78, 19, 6, 300000),
        ("Brook", CrewRarity::Rare, CrewTrait::StrawHat, Some(CrewTrait::Paramecia), 72, 17, 8, 300000),
        ("Jinbe", CrewRarity::Epic, CrewTrait::StrawHat, None, 85, 22, 12, 500000),

        // Marines
        ("Monkey D. Garp", CrewRarity::Legendary, CrewTrait::Marine, None, 95, 25, 15, 800000),
        ("Sengoku", CrewRarity::Legendary, CrewTrait::Marine, Some(CrewTrait::Zoan), 92, 24, 14, 800000),
        ("Akainu", CrewRarity::Legendary, CrewTrait::Marine, Some(CrewTrait::Logia), 98, 26, 16, 1000000),
        ("Kizaru", CrewRarity::Legendary, CrewTrait::Marine, Some(CrewTrait::Logia), 94, 25, 14, 900000),
        ("Fujitora", CrewRarity::Epic, CrewTrait::Marine, Some(CrewTrait::Paramecia), 88, 23, 13, 600000),
        ("Smoker", CrewRarity::Rare, CrewTrait::Marine, Some(CrewTrait::Logia), 76, 19, 9, 350000),
        ("Tashigi", CrewRarity::Common, CrewTrait::Marine, Some(CrewTrait::Sword), 58, 14, 6, 200000),
        ("Koby", CrewRarity::Rare, CrewTrait::Marine, None, 68, 16, 8, 280000),

        // Revolutionary Army
        ("Monkey D. Dragon", CrewRarity::Legendary, CrewTrait::Revolutionary, None, 100, 28, 18, 1200000),
        ("Sabo", CrewRarity::Epic, CrewTrait::Revolutionary, Some(CrewTrait::Logia), 89, 23, 13, 650000),
        ("Emporio Ivankov", CrewRarity::Rare, CrewTrait::Revolutionary, Some(CrewTrait::Paramecia), 74, 18, 10, 320000),
        ("Bartholomew Kuma", CrewRarity::Epic, CrewTrait::Revolutionary, Some(CrewTrait::Paramecia), 87, 22, 12, 600000),
        ("Inazuma", CrewRarity::Common, CrewTrait::Revolutionary, Some(CrewTrait::Paramecia), 62, 15, 7, 220000),

        // Red Hair Pirates
        ("Shanks", CrewRarity::Legendary, CrewTrait::RedHairPirates, None, 99, 27, 17, 1100000),
        ("Ben Beckman", CrewRarity::Epic, CrewTrait::RedHairPirates, None, 90, 24, 14, 700000),
        ("Yasopp", CrewRarity::Rare, CrewTrait::RedHairPirates, None, 77, 20, 9, 350000),
        ("Lucky Roux", CrewRarity::Rare, CrewTrait::RedHairPirates, None, 75, 19, 11, 340000),

        // Giants
        ("Dorry", CrewRarity::Rare, CrewTrait::Giants, None, 82, 21, 8, 380000),
        ("Brogy", CrewRarity::Rare, CrewTrait::Giants, None, 82, 21, 8, 380000),
        ("Oimo", CrewRarity::Common, CrewTrait::Giants, None, 65, 16, 6, 240000),
        ("Kashii", CrewRarity::Common, CrewTrait::Giants, None, 65, 16, 6, 240000),
        ("Hajrudin", CrewRarity::Rare, CrewTrait::Giants, None, 79, 20, 9, 360000),
        ("Loki", CrewRarity::Epic, CrewTrait::Giants, CrewTrait::Zoan, 79, 20, 9, 360000),

        // Five Elders
        ("St. Jaygarcia Saturn", CrewRarity::Legendary, CrewTrait::FiveElders, Some(CrewTrait::Zoan), 97, 27, 16, 1050000),
        ("St. Marcus Mars", CrewRarity::Legendary, CrewTrait::FiveElders, Some(CrewTrait::Zoan), 97, 27, 16, 1050000),
        ("St. Topman Warcury", CrewRarity::Legendary, CrewTrait::FiveElders, Some(CrewTrait::Zoan), 98, 28, 17, 1100000),
        ("St. Ethanbaron V. Nusjuro", CrewRarity::Legendary, CrewTrait::FiveElders, Some(CrewTrait::Zoan), 97, 27, 16, 1050000),
        ("St. Shepherd Ju Peter", CrewRarity::Legendary, CrewTrait::FiveElders, Some(CrewTrait::Zoan), 97, 27, 16, 1050000),

        // Holy Knights
        ("Figarland Garling", CrewRarity::Legendary, CrewTrait::HolyKnights, None, 96, 26, 15, 950000),
        ("Figarland Shamrock", CrewRarity::Epic, CrewTrait::HolyKnights, None, 86, 22, 12, 580000),
        ("Hanmayer Gunko", CrewRarity::Rare, CrewTrait::HolyKnights, None, 73, 18, 10, 330000),
        ("Shepherd Sommers", CrewRarity::Rare, CrewTrait::HolyKnights, None, 75, 19, 9, 340000),
        ("Rimoshifu Kiilingham", CrewRarity::Rare, CrewTrait::HolyKnights, None, 71, 17, 9, 310000),
        ("Satcheis Maffey", CrewRarity::Rare, CrewTrait::HolyKnights, None, 69, 16, 11, 300000),
    ];

    // Generate 5 random crew from pool
    for i in 0..5 {
        let mut rng = ctx.rng();
        let index = rng.gen_range(0..crew_pool.len());
        let template = crew_pool[index];

        ctx.db.shop_crew().insert(ShopCrew {
            id: 0,
            player: identity,
            name: template.0.to_string(),
            rarity: template.1,
            trait1: template.2,
            trait2: template.3,
            max_hp: template.4,
            attack: template.5,
            defense: template.6,
            cost: template.7,
        });
    }

    Ok(())
}

#[spacetimedb::reducer]
pub fn buy_crew(ctx: &ReducerContext, shop_crew_id: u64, slot_index: Option<u8>) -> Result<(), String> {
    let identity = ctx.sender;

    if let Some(slot) = slot_index {
        if slot > 14 {
            return Err("Invalid slot index (0-14 for field)".to_string());
        }
    }

    let shop_crew = ctx.db.shop_crew().id().find(shop_crew_id)
        .ok_or("Shop crew not found")?;

    if shop_crew.player != identity {
        return Err("Not your shop".to_string());
    }

    let mut player = ctx.db.player().identity().find(identity)
        .ok_or("Player not found")?;

    if player.berries < shop_crew.cost {
        return Err("Not enough Berries".to_string());
    }

    // Check if slot is occupied (if placing on field)
    if let Some(slot) = slot_index {
        if ctx.db.crew().owner().filter(&identity)
            .any(|c| c.slot_index == Some(slot)) {
            return Err("Slot already occupied".to_string());
        }
    }

    // Deduct berries
    player.berries -= shop_crew.cost;
    ctx.db.player().identity().update(player);

    // Add crew to player
    ctx.db.crew().insert(Crew {
        id: 0,
        owner: identity,
        name: shop_crew.name.clone(),
        rarity: shop_crew.rarity,
        trait1: shop_crew.trait1,
        trait2: shop_crew.trait2,
        max_hp: shop_crew.max_hp,
        current_hp: shop_crew.max_hp,
        attack: shop_crew.attack,
        defense: shop_crew.defense,
        level: 1,
        slot_index,
        item1: None,
        item2: None,
        completed_item: None,
    });

    // Remove from shop
    ctx.db.shop_crew().id().delete(shop_crew_id);

    Ok(())
}

#[spacetimedb::reducer]
pub fn move_crew(ctx: &ReducerContext, crew_id: u64, new_slot: Option<u8>) -> Result<(), String> {
    let identity = ctx.sender;

    if let Some(slot) = new_slot {
        if slot > 9 {
            return Err("Invalid slot index".to_string());
        }
    }

    let crew = ctx.db.crew().id().find(crew_id)
        .ok_or("Crew not found")?;

    if crew.owner != identity {
        return Err("Not your crew".to_string());
    }

    // Check if target slot is occupied
    if let Some(slot) = new_slot {
        if let Some(other_crew) = ctx.db.crew().owner().filter(&identity)
            .find(|c| c.slot_index == Some(slot) && c.id != crew_id) {
            // Swap positions
            ctx.db.crew().id().update(Crew {
                slot_index: crew.slot_index,
                ..other_crew
            });
        }
    }

    // Move crew
    ctx.db.crew().id().update(Crew {
        slot_index: new_slot,
        ..crew
    });

    Ok(())
}

#[spacetimedb::reducer]
pub fn start_battle(ctx: &ReducerContext) -> Result<(), String> {
    let identity = ctx.sender;

    // Check if player has crew on field
    let field_crew_count = ctx.db.crew().owner().filter(&identity)
        .filter(|c| c.slot_index.is_some())
        .count();

    if field_crew_count == 0 {
        return Err("Need at least one crew member on the field".to_string());
    }

    // Check for ship upgrade
    let straw_hat_count = ctx.db.crew().owner().filter(&identity)
        .filter(|c| c.trait1 == CrewTrait::StrawHat || c.trait2 == Some(CrewTrait::StrawHat))
        .count();

    let mut player = ctx.db.player().identity().find(identity)
        .ok_or("Player not found")?;

    if straw_hat_count >= 6 && player.ship_type != ShipType::ThousandSunny {
        player.ship_type = ShipType::ThousandSunny;
        ctx.db.player().identity().update(player);
    } else if straw_hat_count >= 3 && player.ship_type == ShipType::Raft {
        player.ship_type = ShipType::GoingMerry;
        ctx.db.player().identity().update(player);
    }

    // Find waiting battle or create new one
    if let Some(battle) = ctx.db.battle().status().filter(&BattleStatus::WaitingForOpponent)
        .find(|b| b.player1 != identity) {
        // Join existing battle
        ctx.db.battle().id().update(Battle {
            player2: Some(identity),
            status: BattleStatus::InProgress,
            ..battle
        });
    } else {
        // Check if already waiting
        if ctx.db.battle().status().filter(&BattleStatus::WaitingForOpponent)
            .any(|b| b.player1 == identity) {
            return Err("Already waiting for opponent".to_string());
        }

        // Create new battle
        ctx.db.battle().insert(Battle {
            id: 0,
            player1: identity,
            player2: None,
            winner: None,
            status: BattleStatus::WaitingForOpponent,
            turn: 0,
            bounty_reward: 0,
        });
    }

    Ok(())
}

#[spacetimedb::reducer(client_connected)]
pub fn client_connected(ctx: &ReducerContext) {
    let identity = ctx.sender;

    if let Some(player) = ctx.db.player().identity().find(identity) {
        ctx.db.player().identity().update(Player {
            online: true,
            ..player
        });
    }
}

#[spacetimedb::reducer(client_disconnected)]
pub fn client_disconnected(ctx: &ReducerContext) {
    let identity = ctx.sender;

    if let Some(player) = ctx.db.player().identity().find(identity) {
        ctx.db.player().identity().update(Player {
            online: false,
            ..player
        });
    }
}

// ========== ITEM MANAGEMENT REDUCERS ==========

#[spacetimedb::reducer]
pub fn equip_item_to_crew(ctx: &ReducerContext, crew_id: u64, component: ItemComponent) -> Result<(), String> {
    let identity = ctx.sender;

    let crew = ctx.db.crew().id().find(crew_id)
        .ok_or("Crew not found")?;

    if crew.owner != identity {
        return Err("Not your crew".to_string());
    }

    // Check if player has this item
    let player_item = ctx.db.player_item().owner().filter(&identity)
        .find(|item| item.component == component)
        .ok_or("You don't have this item")?;

    // Check if crew already has 2 items or a completed item
    if crew.completed_item.is_some() {
        return Err("Crew already has a completed item".to_string());
    }

    let updated_crew = if crew.item1.is_none() {
        // First item slot
        Crew {
            item1: Some(component),
            ..crew
        }
    } else if crew.item2.is_none() {
        // Second item slot - check for combination
        let item1 = crew.item1.unwrap();
        if let Some(completed) = try_combine_items(item1, component) {
            // Items combine! Create completed item
            Crew {
                item1: None,
                item2: None,
                completed_item: Some(completed),
                ..crew
            }
        } else {
            // No combination, just add second item
            Crew {
                item2: Some(component),
                ..crew
            }
        }
    } else {
        return Err("Crew already has 2 items".to_string());
    };

    // Remove item from player's inventory
    ctx.db.player_item().id().delete(player_item.id);

    // Update crew
    ctx.db.crew().id().update(updated_crew);

    Ok(())
}

#[spacetimedb::reducer]
pub fn remove_item_from_crew(ctx: &ReducerContext, crew_id: u64, slot: u8) -> Result<(), String> {
    let identity = ctx.sender;

    let crew = ctx.db.crew().id().find(crew_id)
        .ok_or("Crew not found")?;

    if crew.owner != identity {
        return Err("Not your crew".to_string());
    }

    let (updated_crew, removed_component) = if slot == 1 && crew.item1.is_some() {
        (Crew { item1: None, ..crew }, crew.item1.unwrap())
    } else if slot == 2 && crew.item2.is_some() {
        (Crew { item2: None, ..crew }, crew.item2.unwrap())
    } else {
        return Err("Invalid slot or no item in slot".to_string());
    };

    // Return item to player's inventory
    ctx.db.player_item().insert(PlayerItem {
        id: 0,
        owner: identity,
        component: removed_component,
    });

    ctx.db.crew().id().update(updated_crew);

    Ok(())
}

// Helper function to check item combinations
fn try_combine_items(item1: ItemComponent, item2: ItemComponent) -> Option<CompletedItem> {
    use ItemComponent::*;
    use CompletedItem::*;

    match (item1, item2) {
        // Damage / AD Focus
        (Cutlass, SniperGoggles) | (SniperGoggles, Cutlass) => Some(Yoru),
        (Cutlass, ShellDial) | (ShellDial, Cutlass) => Some(Kabuto),
        (Cutlass, SeastoneFragment) | (SeastoneFragment, Cutlass) => Some(Shusui),

        // AP Focus
        (ToneDial, ToneDial) => Some(ClimaTact),
        (ToneDial, ShellDial) | (ShellDial, ToneDial) => Some(ThunderTempo),
        (ToneDial, EnergyDrink) | (EnergyDrink, ToneDial) => Some(MirageFlower),

        // Tank Focus
        (SeastoneFragment, SeastoneFragment) => Some(AdamWood),
        (SeastoneFragment, TidalCloak) | (TidalCloak, SeastoneFragment) => Some(SeaKingScale),
        (SeastoneFragment, Meat) | (Meat, SeastoneFragment) => Some(ThousandSunnyHull),

        // Utility Focus
        (EnergyDrink, TidalCloak) | (TidalCloak, EnergyDrink) => Some(VivrCard),
        (SniperGoggles, EnergyDrink) | (EnergyDrink, SniperGoggles) => Some(LogPose),
        (ToneDial, SeastoneFragment) | (SeastoneFragment, ToneDial) => Some(Poneglyph),

        // Hybrid
        (Cutlass, Meat) | (Meat, Cutlass) => Some(GumGumFruit),
        (Meat, Meat) => Some(GomuGomuNoMi),
        (ToneDial, Cutlass) | (Cutlass, ToneDial) => Some(HakiMastery),

        _ => None,
    }
}
