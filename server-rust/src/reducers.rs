use spacetimedb::{ReducerContext, rand::Rng, Table, log};
use crate::types::*;
use crate::tables::*;
use crate::items::try_combine_items;
use crate::systems::ship_upgrade::*;
use crate::systems::treasure_island::*;

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
        fights_completed: 0,
        active_trait: None,
        trait_level: 0,
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
        if let Some(completed) = try_combine_items(ctx, item1, component) {
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

// ========== SHIP UPGRADE REDUCERS ==========

/// Update player's ship based on active traits (call this after buying/moving crew)
#[spacetimedb::reducer]
pub fn update_ship(ctx: &ReducerContext) -> Result<(), String> {
    let identity = ctx.sender;
    update_player_ship(ctx, identity);
    Ok(())
}

/// Check if player should visit workshop and generate offers
#[spacetimedb::reducer]
pub fn check_workshop(ctx: &ReducerContext) -> Result<(), String> {
    let identity = ctx.sender;

    let player = ctx.db.player().identity().find(identity)
        .ok_or("Player not found")?;

    if should_visit_workshop(player.fights_completed) {
        generate_upgrade_offers(ctx, identity, player.fights_completed);
        Ok(())
    } else {
        Err(format!("Workshop opens every 10 fights. Next at fight {}",
            ((player.fights_completed / 10) + 1) * 10))
    }
}

/// Choose a ship upgrade from Franky's Workshop
#[spacetimedb::reducer]
pub fn choose_ship_upgrade(ctx: &ReducerContext, offer_id: u64) -> Result<(), String> {
    let identity = ctx.sender;

    let offer = ctx.db.ship_upgrade_offer().id().find(offer_id)
        .ok_or("Offer not found")?;

    if offer.player != identity {
        return Err("Not your offer".to_string());
    }

    let player = ctx.db.player().identity().find(identity)
        .ok_or("Player not found")?;

    // Verify this is the correct fight number
    if !should_visit_workshop(player.fights_completed) {
        return Err("Can only choose upgrades at workshop (every 10 fights)".to_string());
    }

    // Add upgrade to player
    ctx.db.player_ship_upgrade().insert(PlayerShipUpgrade {
        id: 0,
        owner: identity,
        upgrade_type: offer.upgrade_type,
        acquired_at_fight: player.fights_completed,
    });

    // Clear all offers for this player
    let offers: Vec<_> = ctx.db.ship_upgrade_offer()
        .player()
        .filter(&identity)
        .collect();

    for o in offers {
        ctx.db.ship_upgrade_offer().id().delete(o.id);
    }

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
    winner_player.fights_completed += 1;

    // Apply Plunderer upgrade (extra gold based on win streak)
    let has_plunderer = ctx.db.player_ship_upgrade()
        .owner()
        .filter(&winner)
        .any(|u| u.upgrade_type == ShipUpgradeType::Plunderer);

    if has_plunderer {
        let streak_bonus = winner_player.wins.min(10) * 10; // Cap at 10 wins = 100 bonus
        winner_player.berries += streak_bonus;
    }

    // Apply BonusGold upgrade
    let has_bonus_gold = ctx.db.player_ship_upgrade()
        .owner()
        .filter(&winner)
        .any(|u| u.upgrade_type == ShipUpgradeType::BonusGold);

    if has_bonus_gold {
        winner_player.berries += 1;
    }

    let winner_fights = winner_player.fights_completed;
    ctx.db.player().identity().update(winner_player);

    // Update loser
    loser_player.losses += 1;
    loser_player.bounty = 0; // Reset bounty to 0 on loss
    loser_player.fights_completed += 1;

    let loser_fights = loser_player.fights_completed;
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

    // Check if winner should visit workshop
    if should_visit_workshop(winner_fights) {
        generate_upgrade_offers(ctx, winner, winner_fights);
    }

    // Check if loser should visit workshop
    if should_visit_workshop(loser_fights) {
        generate_upgrade_offers(ctx, loser, loser_fights);
    }

    Ok(())
}

// ========== TREASURE ISLAND REDUCERS ==========

/// Advance to next round and check for treasure island
#[spacetimedb::reducer]
pub fn advance_round(ctx: &ReducerContext) -> Result<(), String> {
    let identity = ctx.sender;

    let mut player = ctx.db.player().identity().find(identity)
        .ok_or("Player not found")?;

    // Increment round (we can use fights_completed as round number)
    player.fights_completed += 1;
    let round_number = player.fights_completed;

    ctx.db.player().identity().update(player);

    // Check if this is a treasure island round
    if should_spawn_treasure_island(round_number) {
        spawn_treasure_island(ctx, identity, round_number);
    }

    // Check if workshop should open (every 10 rounds)
    if should_visit_workshop(round_number) {
        generate_upgrade_offers(ctx, identity, round_number);
    }

    Ok(())
}

/// Claim all rewards from a treasure island
#[spacetimedb::reducer]
pub fn claim_treasure_island(ctx: &ReducerContext, treasure_island_id: u64) -> Result<(), String> {
    let identity = ctx.sender;
    claim_treasure_reward(ctx, identity, treasure_island_id)
}

/// Get current unclaimed treasure island for player
#[spacetimedb::reducer]
pub fn get_current_treasure_island(ctx: &ReducerContext) -> Result<(), String> {
    let identity = ctx.sender;

    // Find unclaimed treasure island
    let treasure_island = ctx.db.treasure_island()
        .player()
        .filter(&identity)
        .find(|t| !t.claimed)
        .ok_or("No unclaimed treasure island")?;

    log::info!("Player has unclaimed treasure island at round {}", treasure_island.round_number);

    Ok(())
}
