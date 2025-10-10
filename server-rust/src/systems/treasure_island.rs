use spacetimedb::{ReducerContext, Table, log, rand::Rng, Identity};
use crate::types::*;
use crate::tables::*;

/// Check if the current round should trigger a treasure island
pub fn should_spawn_treasure_island(round_number: u32) -> bool {
    TREASURE_ISLAND_ROUNDS.contains(&round_number)
}

/// Get treasure island type based on round number
pub fn get_island_type(round_number: u32) -> TreasureIslandType {
    match round_number {
        5 => TreasureIslandType::SmallIsland,
        15 => TreasureIslandType::MediumIsland,
        25 => TreasureIslandType::LargeIsland,
        _ => TreasureIslandType::SmallIsland,
    }
}

/// Get number of rewards based on island type
pub fn get_reward_count(island_type: TreasureIslandType) -> u32 {
    match island_type {
        TreasureIslandType::SmallIsland => 1,
        TreasureIslandType::MediumIsland => 2,
        TreasureIslandType::LargeIsland => 3,
    }
}

/// Generate a treasure island encounter for a player
pub fn spawn_treasure_island(ctx: &ReducerContext, player: Identity, round_number: u32) {
    // Check if treasure island already exists for this round
    if ctx.db.treasure_island()
        .player()
        .filter(&player)
        .any(|t| t.round_number == round_number) {
        log::info!("Treasure island already exists for player at round {}", round_number);
        return;
    }

    let island_type = get_island_type(round_number);
    let reward_count = get_reward_count(island_type);

    // Create treasure island
    let treasure_island = ctx.db.treasure_island().insert(TreasureIsland {
        id: 0,
        player,
        round_number,
        island_type,
        claimed: false,
    });

    // Generate random rewards
    let mut rng = ctx.rng();
    for _ in 0..reward_count {
        let reward_type = generate_random_reward(&mut rng, island_type);

        ctx.db.treasure_reward().insert(TreasureReward {
            id: 0,
            treasure_island_id: treasure_island.id,
            reward_type,
            claimed: false,
        });
    }

    log::info!(
        "Spawned {:?} with {} rewards for player at round {}",
        island_type,
        reward_count,
        round_number
    );
}

/// Generate a random reward based on island type
fn generate_random_reward(rng: &mut impl Rng, island_type: TreasureIslandType) -> TreasureRewardType {
    let roll = rng.gen_range(0..100);

    match island_type {
        TreasureIslandType::SmallIsland => {
            // Small island: 60% gold, 40% item
            if roll < 60 {
                let gold = rng.gen_range(50..=100);
                TreasureRewardType::Gold(gold)
            } else {
                let item = random_item_component(rng);
                TreasureRewardType::Item(item)
            }
        }
        TreasureIslandType::MediumIsland => {
            // Medium island: 40% gold, 50% item, 10% reroll
            if roll < 40 {
                let gold = rng.gen_range(100..=200);
                TreasureRewardType::Gold(gold)
            } else if roll < 90 {
                let item = random_item_component(rng);
                TreasureRewardType::Item(item)
            } else {
                TreasureRewardType::Reroll
            }
        }
        TreasureIslandType::LargeIsland => {
            // Large island: 30% gold, 60% item, 10% reroll
            if roll < 30 {
                let gold = rng.gen_range(200..=400);
                TreasureRewardType::Gold(gold)
            } else if roll < 90 {
                let item = random_item_component(rng);
                TreasureRewardType::Item(item)
            } else {
                TreasureRewardType::Reroll
            }
        }
    }
}

/// Generate a random item component
fn random_item_component(rng: &mut impl Rng) -> ItemComponent {
    let items = [
        ItemComponent::Cutlass,
        ItemComponent::SniperGoggles,
        ItemComponent::ShellDial,
        ItemComponent::ToneDial,
        ItemComponent::SeastoneFragment,
        ItemComponent::TidalCloak,
        ItemComponent::EnergyDrink,
        ItemComponent::Meat,
    ];

    items[rng.gen_range(0..items.len())]
}

/// Claim a treasure island reward
pub fn claim_treasure_reward(
    ctx: &ReducerContext,
    player: Identity,
    treasure_island_id: u64,
) -> Result<(), String> {
    // Get treasure island
    let mut treasure_island = ctx.db.treasure_island()
        .id()
        .find(&treasure_island_id)
        .ok_or("Treasure island not found")?;

    // Verify ownership
    if treasure_island.player != player {
        return Err("Not your treasure island".to_string());
    }

    // Check if already claimed
    if treasure_island.claimed {
        return Err("Treasure island already claimed".to_string());
    }

    // Get player
    let mut player_data = ctx.db.player()
        .identity()
        .find(&player)
        .ok_or("Player not found")?;

    // Claim all rewards
    let rewards: Vec<_> = ctx.db.treasure_reward()
        .treasure_island_id()
        .filter(&treasure_island_id)
        .filter(|r| !r.claimed)
        .collect();

    for mut reward in rewards {
        match reward.reward_type {
            TreasureRewardType::Gold(amount) => {
                player_data.berries += amount;
                log::info!("Player claimed {} berries from treasure island", amount);
            }
            TreasureRewardType::Item(component) => {
                ctx.db.player_item().insert(PlayerItem {
                    id: 0,
                    owner: player,
                    component,
                });
                log::info!("Player claimed {:?} from treasure island", component);
            }
            TreasureRewardType::Reroll => {
                // Give player a free reroll (could add a counter to player table)
                player_data.berries += 2; // Or add free_rerolls field
                log::info!("Player claimed free reroll from treasure island");
            }
            TreasureRewardType::Experience => {
                // Future: Add XP to units
                log::info!("Player claimed experience from treasure island");
            }
        }

        // Mark reward as claimed
        reward.claimed = true;
        ctx.db.treasure_reward().id().update(reward);
    }

    // Mark island as claimed
    treasure_island.claimed = true;
    ctx.db.treasure_island().id().update(treasure_island);

    // Update player
    ctx.db.player().identity().update(player_data);

    Ok(())
}

/// Get description for reward type
pub fn get_reward_description(reward_type: &TreasureRewardType) -> String {
    match reward_type {
        TreasureRewardType::Gold(amount) => format!("{} Berries", amount),
        TreasureRewardType::Item(component) => format!("{:?}", component),
        TreasureRewardType::Reroll => "Free Shop Reroll".to_string(),
        TreasureRewardType::Experience => "Bonus Experience".to_string(),
    }
}
