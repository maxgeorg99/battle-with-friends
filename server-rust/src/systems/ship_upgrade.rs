use spacetimedb::{ReducerContext, log};
use crate::types::*;
use crate::tables::*;
use std::collections::HashMap;

/// Calculate the active trait (highest level trait) for a player
/// Returns (trait, level) where level is the count of units with that trait
pub fn calculate_ship_type(ctx: &ReducerContext, player: spacetimedb::Identity) -> ShipType {
    let trait_counts: HashMap<CrewTrait, u32> = ctx
        .db
        .crew()
        .owner()
        .filter(&player)
        .filter(|crew| crew.slot_index.is_some())
        .flat_map(|crew| crew.traits)
        .fold(HashMap::new(), |mut acc, trait_type| {
            *acc.entry(trait_type).or_insert(0) += 1;
            acc
        });

    trait_counts
        .iter()
        .filter(|(trait_type, _)| trait_type.is_ship_defining())
        .max_by_key(|(_, count)| *count)
        .and_then(|(trait_type, _level)| trait_type.ship_trait())
        .unwrap_or(ShipType::Raft)
}

/// Update player's ship type based on active trait level
pub fn update_player_ship(ctx: &ReducerContext, player_identity: spacetimedb::Identity) {
    let mut player = match ctx.db.player().identity().find(&player_identity) {
        Some(p) => p,
        None => return,
    };

    // Update ship type based on trait and level
    let new_ship_type = calculate_ship_type(ctx, player_identity);

    if new_ship_type != player.ship_type {
        log::info!(
            "Player {} upgraded ship from {} to {}",
            player.name,
            player.ship_type.display_name(),
            new_ship_type.display_name(),
        );
    }

    player.ship_type = new_ship_type;

    ctx.db.player().identity().update(player);
}
