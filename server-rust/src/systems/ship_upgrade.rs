use spacetimedb::{ReducerContext, Table, log, rand::Rng};
use crate::types::*;
use crate::tables::*;
use std::collections::HashMap;

/// Initialize ship upgrade database - called once on server initialization
pub fn init_ship_upgrades(ctx: &ReducerContext) {
    if ctx.db.ship_upgrade_data().count() > 0 {
        log::info!("Ship upgrades already initialized, skipping...");
        return;
    }

    log::info!("Initializing ship upgrade database...");

    let upgrades = vec![
        // Common upgrades (rarity 1)
        ShipUpgradeData {
            id: 0,
            upgrade_type: ShipUpgradeType::BonusGold,
            name: "Treasure Hunter".to_string(),
            description: "Gain +1 gold per round".to_string(),
            rarity: 1,
        },
        ShipUpgradeData {
            id: 0,
            upgrade_type: ShipUpgradeType::BonusHealth,
            name: "Hardened Crew".to_string(),
            description: "All units gain +10 HP".to_string(),
            rarity: 1,
        },
        ShipUpgradeData {
            id: 0,
            upgrade_type: ShipUpgradeType::BonusAttack,
            name: "Weapons Master".to_string(),
            description: "All units gain +2 attack".to_string(),
            rarity: 1,
        },
        ShipUpgradeData {
            id: 0,
            upgrade_type: ShipUpgradeType::BonusDefense,
            name: "Iron Defense".to_string(),
            description: "All units gain +2 defense".to_string(),
            rarity: 1,
        },
        ShipUpgradeData {
            id: 0,
            upgrade_type: ShipUpgradeType::BonusSpeed,
            name: "Swift Strikes".to_string(),
            description: "All units gain +10% attack speed".to_string(),
            rarity: 1,
        },

        // Uncommon upgrades (rarity 2)
        ShipUpgradeData {
            id: 0,
            upgrade_type: ShipUpgradeType::StrawHatBuff,
            name: "Straw Hat Pride".to_string(),
            description: "Straw Hat units gain +20% stats".to_string(),
            rarity: 2,
        },
        ShipUpgradeData {
            id: 0,
            upgrade_type: ShipUpgradeType::MarineBuff,
            name: "Marine Discipline".to_string(),
            description: "Marine units gain +20% stats".to_string(),
            rarity: 2,
        },
        ShipUpgradeData {
            id: 0,
            upgrade_type: ShipUpgradeType::RevolutionaryBuff,
            name: "Revolutionary Spirit".to_string(),
            description: "Revolutionary units gain +20% stats".to_string(),
            rarity: 2,
        },
        ShipUpgradeData {
            id: 0,
            upgrade_type: ShipUpgradeType::LogiaBuff,
            name: "Logia Mastery".to_string(),
            description: "Logia units gain +5 defense".to_string(),
            rarity: 2,
        },
        ShipUpgradeData {
            id: 0,
            upgrade_type: ShipUpgradeType::ParameciaBuff,
            name: "Paramecia Power".to_string(),
            description: "Paramecia units gain +20 HP".to_string(),
            rarity: 2,
        },
        ShipUpgradeData {
            id: 0,
            upgrade_type: ShipUpgradeType::ZoanBuff,
            name: "Zoan Transformation".to_string(),
            description: "Zoan units gain +50 HP".to_string(),
            rarity: 2,
        },

        // Rare upgrades (rarity 3)
        ShipUpgradeData {
            id: 0,
            upgrade_type: ShipUpgradeType::GamblerLuck,
            name: "Lucky Dice".to_string(),
            description: "Rerolls cost 1 gold (down from 2)".to_string(),
            rarity: 3,
        },
        ShipUpgradeData {
            id: 0,
            upgrade_type: ShipUpgradeType::FastLearner,
            name: "Training Montage".to_string(),
            description: "Units gain +50% XP".to_string(),
            rarity: 3,
        },
        ShipUpgradeData {
            id: 0,
            upgrade_type: ShipUpgradeType::Plunderer,
            name: "Pirate's Bounty".to_string(),
            description: "Win streaks give +1 extra gold per streak level".to_string(),
            rarity: 3,
        },
        ShipUpgradeData {
            id: 0,
            upgrade_type: ShipUpgradeType::Medic,
            name: "Ship Doctor".to_string(),
            description: "Units heal 5 HP per second in combat".to_string(),
            rarity: 3,
        },
        ShipUpgradeData {
            id: 0,
            upgrade_type: ShipUpgradeType::Arsenal,
            name: "Weapon Cache".to_string(),
            description: "Units start with +1 item slot".to_string(),
            rarity: 3,
        },
    ];

    for upgrade in upgrades {
        ctx.db.ship_upgrade_data().insert(upgrade);
    }

    log::info!("Ship upgrade database initialized with {} upgrades", ctx.db.ship_upgrade_data().count());
}

/// Calculate the active trait (highest level trait) for a player
/// Returns (trait, level) where level is the count of units with that trait
pub fn calculate_active_trait(ctx: &ReducerContext, player: spacetimedb::Identity) -> (Option<CrewTrait>, u32) {
    // Count all traits from player's crew (only units on the field)
    let mut trait_counts: HashMap<CrewTrait, u32> = HashMap::new();

    for crew in ctx.db.crew().owner().filter(&player) {
        // Only count crew members placed on the ship (have slot_index)
        if crew.slot_index.is_none() {
            continue;
        }

        // Count primary trait
        *trait_counts.entry(crew.trait1).or_insert(0) += 1;

        // Count secondary trait if exists
        if let Some(trait2) = crew.trait2 {
            *trait_counts.entry(trait2).or_insert(0) += 1;
        }

        // For star level (if you implement leveling), could multiply by stars
        // *trait_counts.entry(crew.trait1).or_insert(0) += crew.level as u32;
    }

    // Find the trait with highest count
    let mut max_trait: Option<CrewTrait> = None;
    let mut max_count = 0;

    for (trait_type, count) in trait_counts.iter() {
        if *count > max_count {
            max_count = *count;
            max_trait = Some(*trait_type);
        } else if *count == max_count && max_trait.is_some() {
            // Tiebreaker: use combined star levels and item values
            // For now, just keep the first one (could be extended)
        }
    }

    (max_trait, max_count)
}

/// Update player's ship type based on active trait level
pub fn update_player_ship(ctx: &ReducerContext, player_identity: spacetimedb::Identity) {
    let mut player = match ctx.db.player().identity().find(&player_identity) {
        Some(p) => p,
        None => return,
    };

    // Calculate active trait and level
    let (active_trait, trait_level) = calculate_active_trait(ctx, player_identity);

    // Update ship type based on trait and level
    let new_ship_type = ShipType::from_trait_and_level(active_trait, trait_level);

    if new_ship_type != player.ship_type {
        log::info!(
            "Player {} upgraded ship from {} to {} (trait: {:?}, level: {})",
            player.name,
            player.ship_type.display_name(),
            new_ship_type.display_name(),
            active_trait,
            trait_level
        );
    }

    player.ship_type = new_ship_type;
    player.active_trait = active_trait;
    player.trait_level = trait_level;

    ctx.db.player().identity().update(player);
}

/// Check if player should visit Franky's Workshop (every 10 fights)
pub fn should_visit_workshop(fights_completed: u32) -> bool {
    fights_completed > 0 && fights_completed % 10 == 0
}

/// Generate 3 random ship upgrade offers for player (weighted by rarity)
pub fn generate_upgrade_offers(ctx: &ReducerContext, player_identity: spacetimedb::Identity, fight_number: u32) {
    // Clear any existing offers for this player
    let old_offers: Vec<_> = ctx.db.ship_upgrade_offer()
        .player()
        .filter(&player_identity)
        .collect();

    for offer in old_offers {
        ctx.db.ship_upgrade_offer().id().delete(&offer.id);
    }

    // Get player's existing upgrades to avoid duplicates
    let existing_upgrades: Vec<_> = ctx.db.player_ship_upgrade()
        .owner()
        .filter(&player_identity)
        .map(|u| u.upgrade_type)
        .collect();

    // Get all available upgrades from database
    let all_upgrades: Vec<_> = ctx.db.ship_upgrade_data()
        .iter()
        .filter(|u| !existing_upgrades.contains(&u.upgrade_type))
        .collect();

    if all_upgrades.is_empty() {
        log::warn!("No upgrades available for player (all owned or none initialized)");
        return;
    }

    // Build weighted pool based on rarity
    // Common (rarity 1): 60% chance
    // Uncommon (rarity 2): 30% chance
    // Rare (rarity 3): 10% chance
    let mut weighted_pool = Vec::new();
    for upgrade in &all_upgrades {
        let weight = match upgrade.rarity {
            1 => 60, // Common
            2 => 30, // Uncommon
            3 => 10, // Rare
            _ => 30, // Default
        };
        for _ in 0..weight {
            weighted_pool.push(upgrade);
        }
    }

    // Generate 3 random offers (weighted)
    let mut rng = ctx.rng();
    let offer_count = all_upgrades.len().min(3);

    let mut selected_upgrade_types = Vec::new();
    while selected_upgrade_types.len() < offer_count {
        let idx = rng.gen_range(0..weighted_pool.len());
        let upgrade = weighted_pool[idx];

        // Avoid duplicates in the same offer
        if !selected_upgrade_types.contains(&upgrade.upgrade_type) {
            selected_upgrade_types.push(upgrade.upgrade_type);

            ctx.db.ship_upgrade_offer().insert(ShipUpgradeOffer {
                id: 0,
                player: player_identity,
                upgrade_type: upgrade.upgrade_type,
                fight_number,
            });
        }
    }

    log::info!(
        "Generated {} upgrade offers for player at fight {}",
        offer_count,
        fight_number
    );
}

/// Apply ship upgrade effects to battle stats
pub fn apply_upgrade_effects_to_unit(
    ctx: &ReducerContext,
    owner: spacetimedb::Identity,
    unit: &mut BattleUnit,
    crew: &Crew,
) {
    // Get all upgrades for this player
    let upgrades: Vec<_> = ctx.db.player_ship_upgrade()
        .owner()
        .filter(&owner)
        .collect();

    for upgrade in upgrades {
        match upgrade.upgrade_type {
            ShipUpgradeType::BonusGold => {
                // Applied at round end, not to units
            }
            ShipUpgradeType::BonusHealth => {
                unit.max_hp += 10;
                unit.current_hp += 10;
            }
            ShipUpgradeType::BonusAttack => {
                unit.attack += 2;
            }
            ShipUpgradeType::BonusDefense => {
                unit.defense += 2;
            }
            ShipUpgradeType::BonusSpeed => {
                unit.attack_speed *= 1.1;
            }
            ShipUpgradeType::StrawHatBuff => {
                if crew.trait1 == CrewTrait::StrawHat || crew.trait2 == Some(CrewTrait::StrawHat) {
                    unit.max_hp = (unit.max_hp as f32 * 1.2) as u32;
                    unit.current_hp = (unit.current_hp as f32 * 1.2) as u32;
                    unit.attack = (unit.attack as f32 * 1.2) as u32;
                    unit.defense = (unit.defense as f32 * 1.2) as u32;
                }
            }
            ShipUpgradeType::MarineBuff => {
                if crew.trait1 == CrewTrait::Marine || crew.trait2 == Some(CrewTrait::Marine) {
                    unit.max_hp = (unit.max_hp as f32 * 1.2) as u32;
                    unit.current_hp = (unit.current_hp as f32 * 1.2) as u32;
                    unit.attack = (unit.attack as f32 * 1.2) as u32;
                    unit.defense = (unit.defense as f32 * 1.2) as u32;
                }
            }
            ShipUpgradeType::RevolutionaryBuff => {
                if crew.trait1 == CrewTrait::Revolutionary || crew.trait2 == Some(CrewTrait::Revolutionary) {
                    unit.max_hp = (unit.max_hp as f32 * 1.2) as u32;
                    unit.current_hp = (unit.current_hp as f32 * 1.2) as u32;
                    unit.attack = (unit.attack as f32 * 1.2) as u32;
                    unit.defense = (unit.defense as f32 * 1.2) as u32;
                }
            }
            ShipUpgradeType::LogiaBuff => {
                if crew.trait1 == CrewTrait::Logia || crew.trait2 == Some(CrewTrait::Logia) {
                    // Could add shield mechanic here
                    unit.defense += 5;
                }
            }
            ShipUpgradeType::ParameciaBuff => {
                if crew.trait1 == CrewTrait::Paramecia || crew.trait2 == Some(CrewTrait::Paramecia) {
                    // Could add lifesteal mechanic here
                    unit.max_hp += 20;
                    unit.current_hp += 20;
                }
            }
            ShipUpgradeType::ZoanBuff => {
                if crew.trait1 == CrewTrait::Zoan || crew.trait2 == Some(CrewTrait::Zoan) {
                    unit.max_hp += 50;
                    unit.current_hp += 50;
                }
            }
            ShipUpgradeType::GamblerLuck => {
                // Applied to shop costs, not units
            }
            ShipUpgradeType::FastLearner => {
                // Applied to XP gain, not units
            }
            ShipUpgradeType::Plunderer => {
                // Applied to win streak rewards, not units
            }
            ShipUpgradeType::Medic => {
                // TODO: Add HP regen per second
            }
            ShipUpgradeType::Arsenal => {
                // Applied to item slots, not directly to units
            }
        }
    }
}

/// Get upgrade description for UI
pub fn get_upgrade_description(upgrade_type: ShipUpgradeType) -> String {
    match upgrade_type {
        ShipUpgradeType::BonusGold => "Treasure Hunter: Gain +1 gold per round".to_string(),
        ShipUpgradeType::BonusHealth => "Hardened Crew: All units gain +10 HP".to_string(),
        ShipUpgradeType::BonusAttack => "Weapons Master: All units gain +2 attack".to_string(),
        ShipUpgradeType::BonusDefense => "Iron Defense: All units gain +2 defense".to_string(),
        ShipUpgradeType::BonusSpeed => "Swift Strikes: All units gain +10% attack speed".to_string(),
        ShipUpgradeType::StrawHatBuff => "Straw Hat Pride: Straw Hat units gain +20% stats".to_string(),
        ShipUpgradeType::MarineBuff => "Marine Discipline: Marine units gain +20% stats".to_string(),
        ShipUpgradeType::RevolutionaryBuff => "Revolutionary Spirit: Revolutionary units gain +20% stats".to_string(),
        ShipUpgradeType::LogiaBuff => "Logia Mastery: Logia units gain +5 defense".to_string(),
        ShipUpgradeType::ParameciaBuff => "Paramecia Power: Paramecia units gain +20 HP".to_string(),
        ShipUpgradeType::ZoanBuff => "Zoan Transformation: Zoan units gain +50 HP".to_string(),
        ShipUpgradeType::GamblerLuck => "Lucky Dice: Rerolls cost 1 gold (down from 2)".to_string(),
        ShipUpgradeType::FastLearner => "Training Montage: Units gain +50% XP".to_string(),
        ShipUpgradeType::Plunderer => "Pirate's Bounty: Win streaks give +1 extra gold".to_string(),
        ShipUpgradeType::Medic => "Ship Doctor: Units heal 5 HP per second".to_string(),
        ShipUpgradeType::Arsenal => "Weapon Cache: Units start with +1 item slot".to_string(),
    }
}
