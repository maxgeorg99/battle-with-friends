use spacetimedb::{ReducerContext, rand::Rng, Table, log};
use crate::types::*;
use crate::tables::*;
use crate::systems::ship_upgrade::*;

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
        xp: 0,
        level: 1,
        hp: 5,
        bounty: 0,
        wins: 0,
        win_streak: 0,
        losses: 0,
        ship_type: ShipType::Raft,
        online: true,
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

    // Get crew templates from database (initialized once on server start)
    let templates: Vec<_> = ctx.db.crew_template().iter().collect();

    if templates.is_empty() {
        return Err("Crew template database not initialized".to_string());
    }

    // Generate 5 random crew from template database
    let mut rng = ctx.rng();
    for _ in 0..5 {
        let index = rng.gen_range(0..templates.len());
        let template = &templates[index];

        ctx.db.shop_crew().insert(ShopCrew {
            id: 0,
            player: identity,
            name: template.name.clone(),
            rarity: template.rarity,
            traits: template.traits.clone(),
            max_hp: template.max_hp,
            ability_power: template.ability_power,
            attack: template.attack,
            attack_speed: template.attack_speed,
            defense: template.defense,
            magic_resistance: template.magic_resistance,
            cost: template.cost,
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
        traits: shop_crew.traits.clone(),
        max_hp: shop_crew.max_hp,
        ability_power: shop_crew.ability_power,
        attack: shop_crew.attack,
        attack_speed: shop_crew.attack_speed,
        defense: shop_crew.defense,
        magic_resistance: shop_crew.magic_resistance,
        level: 1,
        slot_index,
        bench_index: slot_index,
        item1: None,
        item2: None,
        item3: None
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

    // Update ship based on active trait
    update_player_ship(ctx, identity);

    // Get player's current bounty
    let player = ctx.db.player().identity().find(identity)
        .ok_or("Player not found")?;
    let player_bounty = player.bounty;

    // Find waiting battle or create new one
    if let Some(mut battle) = ctx.db.battle().status().filter(&BattleStatus::WaitingForOpponent)
        .find(|b| b.player1 != identity) {
        // Join existing battle
        battle.player2 = Some(identity);
        battle.status = BattleStatus::InProgress;
        battle.player2_bounty = player_bounty;

        ctx.db.battle().id().update(battle);
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
            player1_bounty: player_bounty,
            player2_bounty: 0,
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

/// Equip an item from player's inventory to a crew member
#[spacetimedb::reducer]
pub fn equip_item_to_crew(ctx: &ReducerContext, crew_id: u64, player_item_id: u64) -> Result<(), String> {
    let identity = ctx.sender;

    let crew = ctx.db.crew().id().find(crew_id)
        .ok_or("Crew not found")?;

    if crew.owner != identity {
        return Err("Not your crew".to_string());
    }

    // Check if player owns this item
    let player_item = ctx.db.player_item().id().find(player_item_id)
        .ok_or("Item not found")?;

    if player_item.owner != identity {
        return Err("Not your item".to_string());
    }

    // Check if crew has available item slots (max 3)
    let updated_crew = if crew.item1.is_none() {
        Crew { item1: Some(player_item.item), ..crew }
    } else if crew.item2.is_none() {
        Crew { item2: Some(player_item.item), ..crew }
    } else if crew.item3.is_none() {
        Crew { item3: Some(player_item.item), ..crew }
    } else {
        return Err("Crew already has 3 items equipped".to_string());
    };

    // Remove item from player's inventory
    ctx.db.player_item().id().delete(player_item_id);

    // Update crew with equipped item
    ctx.db.crew().id().update(updated_crew);

    Ok(())
}

/// Remove an item from a crew member and return it to player's inventory
#[spacetimedb::reducer]
pub fn remove_item_from_crew(ctx: &ReducerContext, crew_id: u64, slot: u8) -> Result<(), String> {
    let identity = ctx.sender;

    let crew = ctx.db.crew().id().find(crew_id)
        .ok_or("Crew not found")?;

    if crew.owner != identity {
        return Err("Not your crew".to_string());
    }

    // Determine which item to remove based on slot (1, 2, or 3)
    let (updated_crew, removed_item) = match slot {
        1 if crew.item1.is_some() => (Crew { item1: None, ..crew }, crew.item1.unwrap()),
        2 if crew.item2.is_some() => (Crew { item2: None, ..crew }, crew.item2.unwrap()),
        3 if crew.item3.is_some() => (Crew { item3: None, ..crew }, crew.item3.unwrap()),
        _ => return Err("Invalid slot or no item in slot".to_string()),
    };

    // Return item to player's inventory
    ctx.db.player_item().insert(PlayerItem {
        id: 0,
        owner: identity,
        item: removed_item,
        bench_slot: None,
    });

    // Update crew
    ctx.db.crew().id().update(updated_crew);

    Ok(())
}

/// Add an item to player's inventory (e.g., from treasure chest or rewards)
#[spacetimedb::reducer]
pub fn add_item_to_inventory(ctx: &ReducerContext, item: Item) -> Result<(), String> {
    let identity = ctx.sender;

    ctx.db.player_item().insert(PlayerItem {
        id: 0,
        owner: identity,
        item,
        bench_slot: None,
    });

    Ok(())
}

/// Organize items in treasure chest by setting bench slot positions
#[spacetimedb::reducer]
pub fn set_item_bench_slot(ctx: &ReducerContext, player_item_id: u64, bench_slot: Option<u8>) -> Result<(), String> {
    let identity = ctx.sender;

    let player_item = ctx.db.player_item().id().find(player_item_id)
        .ok_or("Item not found")?;

    if player_item.owner != identity {
        return Err("Not your item".to_string());
    }

    ctx.db.player_item().id().update(PlayerItem {
        bench_slot,
        ..player_item
    });

    Ok(())
}

// ========== SHIP UPGRADE REDUCERS ==========

/// Update player's ship based on active traits (call this after buying/moving crew)
#[spacetimedb::reducer]
pub fn update_ship(ctx: &ReducerContext) -> Result<(), String> {
    let identity = ctx.sender;
    update_player_ship(ctx, identity);
    Ok(())
}

/// Complete a battle and handle bounty rewards
#[spacetimedb::reducer]
pub fn complete_battle(ctx: &ReducerContext, battle_id: u64) -> Result<(), String> {
    let identity = ctx.sender;

    // Get battle
    let battle = ctx.db.battle().id().find(&battle_id)
        .ok_or("Battle not found")?;

    // Verify battle is finished
    if battle.status != BattleStatus::Finished {
        return Err("Battle not finished yet".to_string());
    }

    // Verify sender is part of this battle
    if battle.player1 != identity && battle.player2 != Some(identity) {
        return Err("Not your battle".to_string());
    }

    let winner = battle.winner.ok_or("Battle has no winner")?;
    let loser = if winner == battle.player1 {
        battle.player2.ok_or("Battle has no player2")?
    } else {
        battle.player1
    };

    // Get winner and loser
    let mut winner_player = ctx.db.player().identity().find(&winner)
        .ok_or("Winner not found")?;
    let mut loser_player = ctx.db.player().identity().find(&loser)
        .ok_or("Loser not found")?;

    // Calculate bounty reward (loser's bounty goes to winner)
    let bounty_reward = loser_player.bounty;

    // Update winner
    winner_player.wins += 1;
    winner_player.bounty += 100_000; // +100k per win
    winner_player.berries += bounty_reward; // Claim loser's bounty

    ctx.db.player().identity().update(winner_player);

    // Update loser
    loser_player.losses += 1;
    loser_player.bounty = 0; // Reset bounty to 0 on loss

    ctx.db.player().identity().update(loser_player);

    // Update battle with bounty reward
    ctx.db.battle().id().update(Battle {
        bounty_reward,
        ..battle
    });

    log::info!(
        "Battle {} completed: Winner {} claimed {} bounty from {}",
        battle_id,
        winner,
        bounty_reward,
        loser
    );
    Ok(())
}
