use spacetimedb::{ReducerContext, Identity, Table, SpacetimeType, Timestamp};

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
    Warlord,
    Emperor,
    Supernova,
    DFUser,      // Devil Fruit User
}

#[derive(SpacetimeType, Clone, Copy, Debug, PartialEq)]
pub enum BattleStatus {
    WaitingForOpponent,
    InProgress,
    Finished,
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
    pub slot_index: Option<u8>, // 0-9 on ship (field), None = bench/inventory
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
        berries: 100,
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

    // Crew templates (name, rarity, trait1, trait2, hp, attack, defense, cost)
    let crew_pool: Vec<(&str, CrewRarity, CrewTrait, Option<CrewTrait>, u32, u32, u32, u32)> = vec![
        // Straw Hats
        ("Roronoa Zoro", CrewRarity::Rare, CrewTrait::StrawHat, None, 80, 20, 8, 3),
        ("Nami", CrewRarity::Common, CrewTrait::StrawHat, None, 50, 12, 6, 2),
        ("Usopp", CrewRarity::Common, CrewTrait::StrawHat, None, 60, 15, 5, 2),
        ("Sanji", CrewRarity::Rare, CrewTrait::StrawHat, None, 75, 18, 10, 3),
        ("Tony Tony Chopper", CrewRarity::Common, CrewTrait::StrawHat, Some(CrewTrait::DFUser), 55, 10, 12, 3),
        ("Nico Robin", CrewRarity::Epic, CrewTrait::StrawHat, Some(CrewTrait::DFUser), 70, 22, 8, 4),
        ("Franky", CrewRarity::Rare, CrewTrait::StrawHat, None, 85, 19, 15, 3),
        ("Brook", CrewRarity::Rare, CrewTrait::StrawHat, Some(CrewTrait::DFUser), 65, 17, 7, 3),

        // Marines
        ("Marine Soldier", CrewRarity::Common, CrewTrait::Marine, None, 50, 10, 8, 1),
        ("Smoker", CrewRarity::Epic, CrewTrait::Marine, Some(CrewTrait::DFUser), 90, 22, 12, 5),
        ("Tashigi", CrewRarity::Rare, CrewTrait::Marine, None, 70, 16, 9, 3),

        // Supernovas
        ("Trafalgar Law", CrewRarity::Epic, CrewTrait::Supernova, Some(CrewTrait::DFUser), 85, 24, 10, 5),
        ("Eustass Kid", CrewRarity::Epic, CrewTrait::Supernova, Some(CrewTrait::DFUser), 95, 26, 8, 5),
        ("Killer", CrewRarity::Rare, CrewTrait::Supernova, None, 75, 20, 9, 3),

        // Warlords
        ("Dracule Mihawk", CrewRarity::Legendary, CrewTrait::Warlord, None, 120, 35, 15, 7),
        ("Boa Hancock", CrewRarity::Epic, CrewTrait::Warlord, Some(CrewTrait::DFUser), 85, 25, 12, 5),

        // Emperors
        ("Shanks", CrewRarity::Legendary, CrewTrait::Emperor, None, 150, 40, 20, 8),
        ("Charlotte Katakuri", CrewRarity::Epic, CrewTrait::Emperor, Some(CrewTrait::DFUser), 100, 28, 18, 6),
    ];

    // Simple pseudorandom using timestamp
    let seed = ctx.timestamp.micros_since_unix_epoch;

    // Generate 5 random crew from pool
    for i in 0..5 {
        let index = ((seed + i as u64 * 7919) % crew_pool.len() as u64) as usize;
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
        if slot > 9 {
            return Err("Invalid slot index (0-9 for field)".to_string());
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
