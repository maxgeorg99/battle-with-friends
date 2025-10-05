use spacetimedb::{ReducerContext, rand::Rng, Table};
use crate::types::*;
use crate::tables::*;
use crate::items::try_combine_items;

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
            trait1: template.trait1,
            trait2: template.trait2,
            max_hp: template.max_hp,
            attack: template.attack,
            defense: template.defense,
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
