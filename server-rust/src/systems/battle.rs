use spacetimedb::{ReducerContext, Table, log, rand::Rng};
use std::time::Duration;
use crate::types::*;
use crate::tables::*;
use crate::systems::collision::*;

/// Start a battle and create BattleUnits from Crew
pub fn start_battle(ctx: &ReducerContext, battle_id: u64, player1: spacetimedb::Identity, player2: spacetimedb::Identity) {
    log::info!("Starting battle {} between {} and {}", battle_id, player1, player2);

    // Get all crew members for player1 with slot_index (on the field)
    let player1_crew: Vec<_> = ctx.db.crew()
        .owner()
        .filter(&player1)
        .filter(|c| c.slot_index.is_some())
        .collect();

    // Get all crew members for player2
    let player2_crew: Vec<_> = ctx.db.crew()
        .owner()
        .filter(&player2)
        .filter(|c| c.slot_index.is_some())
        .collect();

    // Create BattleUnits for player1 (left side)
    for (i, crew) in player1_crew.iter().enumerate() {
        let position = get_spawn_position(i, 0); // side 0 = left
        create_battle_unit(ctx, battle_id, crew, player1, 0, position);
    }

    // Create BattleUnits for player2 (right side)
    for (i, crew) in player2_crew.iter().enumerate() {
        let position = get_spawn_position(i, 1); // side 1 = right
        create_battle_unit(ctx, battle_id, crew, player2, 1, position);
    }

    // Schedule the first battle tick
    schedule_battle_tick(ctx, battle_id);

    log::info!("Battle {} started with {} vs {} units",
        battle_id, player1_crew.len(), player2_crew.len());
}

/// Get spawn position for a unit based on index and side
fn get_spawn_position(index: usize, side: u8) -> DbVector2 {
    let row = index / 5; // 5 units per row
    let col = index % 5;

    let x = if side == 0 {
        200.0 + (col as f32 * 80.0) // Left side
    } else {
        1400.0 - (col as f32 * 80.0) // Right side
    };

    let y = 300.0 + (row as f32 * 150.0);

    DbVector2::new(x, y)
}

/// Create a BattleUnit from a Crew
fn create_battle_unit(
    ctx: &ReducerContext,
    battle_id: u64,
    crew: &Crew,
    owner: spacetimedb::Identity,
    side: u8,
    position: DbVector2,
) {
    // Calculate stats with item bonuses
    let (total_stats, crit_chance, crit_damage, attack_speed, max_mana) = calculate_unit_stats(ctx, crew);

    let mut unit = BattleUnit {
        id: 0,
        battle_id,
        crew_id: crew.id,
        owner,
        side,
        position,
        velocity: DbVector2::new(0.0, 0.0),
        radius: 32.0, // Standard collision radius
        max_hp: total_stats.hp,
        current_hp: total_stats.hp,
        attack: total_stats.ad,
        defense: total_stats.armor,
        ability_power: total_stats.ap,
        magic_resist: total_stats.mr,
        attack_speed,
        crit_chance,
        crit_damage,
        max_mana,
        current_mana: max_mana, // Start with full mana
        mana_per_attack: 20, // Gain 20 mana per attack
        attack_cooldown: 0.0,
        target_unit_id: None,
        ability_ready: false,
        ability_cooldown: 0.0,
        is_stunned: false,
        stun_duration: 0.0,
    };

    // Apply ship upgrade effects
    crate::systems::ship_upgrade::apply_upgrade_effects_to_unit(ctx, owner, &mut unit, crew);

    ctx.db.battle_unit().insert(unit);
}

struct UnitStats {
    hp: u32,
    ad: u32,
    armor: u32,
    ap: u32,
    mr: u32,
}

/// Calculate total stats including item bonuses
fn calculate_unit_stats(ctx: &ReducerContext, crew: &Crew) -> (UnitStats, f32, f32, f32, u32) {
    let mut hp = crew.max_hp;
    let mut ad = crew.attack;
    let mut armor = crew.defense;
    let mut ap = 0u32;
    let mut mr = 0u32;
    let mut crit_chance = 0.0f32;
    let mut crit_damage = 1.5f32; // Base crit damage
    let mut attack_speed = 1.0f32; // Base attack speed
    let mut max_mana = 100u32; // Base mana

    // Apply item1 bonuses
    if let Some(item) = crew.item1 {
        if let Some(stats) = ctx.db.item_component_stats().component().find(&item) {
            hp = hp.saturating_add(stats.bonus_hp as u32);
            ad = ad.saturating_add(stats.bonus_ad as u32);
            armor = armor.saturating_add(stats.bonus_armor as u32);
            ap = ap.saturating_add(stats.bonus_ap as u32);
            mr = mr.saturating_add(stats.bonus_mr as u32);
            crit_chance += stats.bonus_crit_chance;
            attack_speed += stats.bonus_attack_speed;
            max_mana = max_mana.saturating_add(stats.bonus_mana as u32);
        }
    }

    // Apply item2 bonuses
    if let Some(item) = crew.item2 {
        if let Some(stats) = ctx.db.item_component_stats().component().find(&item) {
            hp = hp.saturating_add(stats.bonus_hp as u32);
            ad = ad.saturating_add(stats.bonus_ad as u32);
            armor = armor.saturating_add(stats.bonus_armor as u32);
            ap = ap.saturating_add(stats.bonus_ap as u32);
            mr = mr.saturating_add(stats.bonus_mr as u32);
            crit_chance += stats.bonus_crit_chance;
            attack_speed += stats.bonus_attack_speed;
            max_mana = max_mana.saturating_add(stats.bonus_mana as u32);
        }
    }

    // Apply completed item bonuses
    if let Some(completed) = crew.completed_item {
        if let Some(stats) = ctx.db.completed_item_stats().item().find(&completed) {
            hp = hp.saturating_add(stats.bonus_hp as u32);
            ad = ad.saturating_add(stats.bonus_ad as u32);
            armor = armor.saturating_add(stats.bonus_armor as u32);
            ap = ap.saturating_add(stats.bonus_ap as u32);
            mr = mr.saturating_add(stats.bonus_mr as u32);
            crit_chance += stats.bonus_crit_chance;
            crit_damage += stats.bonus_crit_damage;
            attack_speed += stats.bonus_attack_speed;
            max_mana = max_mana.saturating_add(stats.bonus_mana as u32);
        }
    }

    let stats = UnitStats { hp, ad, armor, ap, mr };
    (stats, crit_chance, crit_damage, attack_speed, max_mana)
}

/// Schedule the next battle tick
fn schedule_battle_tick(ctx: &ReducerContext, battle_id: u64) {
    ctx.db.battle_tick_timer().insert(BattleTickTimer {
        scheduled_id: 0,
        scheduled_at: spacetimedb::ScheduleAt::Time(
            ctx.timestamp + Duration::from_millis((1000 / BATTLE_TICK_RATE) as u64)
        ),
        battle_id,
    });
}

/// Main battle tick reducer - runs every 50ms (20 ticks/sec)
#[spacetimedb::reducer]
pub fn battle_tick(ctx: &ReducerContext, timer: BattleTickTimer) {
    // Only allow scheduled execution
    if ctx.sender != ctx.identity() {
        log::error!("battle_tick may only be invoked by the scheduler");
        return;
    }

    let battle_id = timer.battle_id;

    // Check if battle still exists
    let battle = match ctx.db.battle().id().find(&battle_id) {
        Some(b) => b,
        None => {
            log::info!("Battle {} no longer exists, stopping tick", battle_id);
            return;
        }
    };

    // Check if battle is finished
    if battle.status == BattleStatus::Finished {
        log::info!("Battle {} is finished, stopping tick", battle_id);
        return;
    }

    // Build collision cache
    let mut cache = BattleCollisionCache::default();
    build_collision_cache(ctx, battle_id, &mut cache);

    // Process AI and combat for each unit
    process_unit_ai_and_combat(ctx, &mut cache);

    // Apply movement
    process_unit_movement(ctx, &cache);

    // Commit damage and check for deaths
    commit_damage_and_check_deaths(ctx, battle_id, &cache);

    // Check for battle end condition
    check_battle_end(ctx, battle_id);

    // Schedule next tick if battle still ongoing
    if let Some(battle) = ctx.db.battle().id().find(&battle_id) {
        if battle.status == BattleStatus::InProgress {
            schedule_battle_tick(ctx, battle_id);
        }
    }
}

/// Build collision cache from BattleUnits
fn build_collision_cache(ctx: &ReducerContext, battle_id: u64, cache: &mut BattleCollisionCache) {
    cache.clear();

    for unit in ctx.db.battle_unit().battle_id().filter(&battle_id) {
        if unit.current_hp == 0 {
            continue; // Skip dead units
        }

        let idx = cache.cached_count;
        cache.unit_ids[idx] = unit.id;
        cache.unit_id_to_index.insert(unit.id, idx);
        cache.pos_x[idx] = unit.position.x;
        cache.pos_y[idx] = unit.position.y;
        cache.radius[idx] = unit.radius;
        cache.side[idx] = unit.side;
        cache.current_hp[idx] = unit.current_hp;

        // Spatial hash
        let cell = get_grid_cell(unit.position.x, unit.position.y) as usize;
        cache.cell[idx] = cell as i32;
        cache.nexts[idx] = cache.heads[cell];
        cache.heads[cell] = idx as i32;

        cache.cached_count += 1;
    }
}

/// Process AI and combat for all units
fn process_unit_ai_and_combat(ctx: &ReducerContext, cache: &mut BattleCollisionCache) {
    // Find targets and execute attacks
    for i in 0..cache.cached_count {
        let unit_id = cache.unit_ids[i];
        let mut unit = match ctx.db.battle_unit().id().find(&unit_id) {
            Some(u) => u,
            None => continue,
        };

        // Skip stunned units
        if unit.is_stunned {
            unit.stun_duration -= DELTA_TIME;
            if unit.stun_duration <= 0.0 {
                unit.is_stunned = false;
                unit.stun_duration = 0.0;
            }
            ctx.db.battle_unit().id().update(unit);
            continue;
        }

        // Decrease attack cooldown
        if unit.attack_cooldown > 0.0 {
            unit.attack_cooldown -= DELTA_TIME;
        }

        // Find nearest enemy
        let target_idx = find_nearest_enemy(
            cache,
            cache.pos_x[i],
            cache.pos_y[i],
            cache.side[i],
            800.0, // Attack range
        );

        if let Some(target_idx) = target_idx {
            // Attack if cooldown ready
            if unit.attack_cooldown <= 0.0 {
                let damage = calculate_damage(&unit, cache, target_idx);
                cache.damage_to_unit[target_idx] += damage;

                // Gain mana
                unit.current_mana = (unit.current_mana + unit.mana_per_attack).min(unit.max_mana);

                // Reset attack cooldown
                unit.attack_cooldown = 1.0 / unit.attack_speed;

                // Check if ability is ready (100 mana)
                if unit.current_mana >= 100 && !unit.ability_ready {
                    unit.ability_ready = true;
                }
            }
        }

        ctx.db.battle_unit().id().update(unit);
    }
}

/// Calculate damage from attacker to target
fn calculate_damage(attacker: &BattleUnit, cache: &BattleCollisionCache, target_idx: usize) -> u32 {
    let mut damage = attacker.attack as f32;

    // Check for critical hit
    let mut rng = spacetimedb::rand::thread_rng();
    if rng.gen::<f32>() < attacker.crit_chance {
        damage *= attacker.crit_damage;
    }

    // Apply armor reduction
    let target_armor = cache.current_hp[target_idx] as f32; // Placeholder - should get actual armor
    let armor_reduction = target_armor / (target_armor + 100.0);
    damage *= 1.0 - armor_reduction;

    damage.max(1.0) as u32 // Minimum 1 damage
}

/// Process unit movement (simple approach to target)
fn process_unit_movement(ctx: &ReducerContext, cache: &BattleCollisionCache) {
    for i in 0..cache.cached_count {
        let unit_id = cache.unit_ids[i];
        let mut unit = match ctx.db.battle_unit().id().find(&unit_id) {
            Some(u) => u,
            None => continue,
        };

        // Find nearest enemy to move toward
        if let Some(target_idx) = find_nearest_enemy(
            cache,
            cache.pos_x[i],
            cache.pos_y[i],
            cache.side[i],
            2000.0,
        ) {
            let dx = cache.pos_x[target_idx] - cache.pos_x[i];
            let dy = cache.pos_y[target_idx] - cache.pos_y[i];
            let dist = (dx * dx + dy * dy).sqrt();

            if dist > 600.0 {
                // Move closer if too far
                let move_speed = 100.0 * DELTA_TIME;
                let dir_x = dx / dist;
                let dir_y = dy / dist;
                unit.position.x += dir_x * move_speed;
                unit.position.y += dir_y * move_speed;

                // Clamp to arena bounds
                unit.position.x = unit.position.x.clamp(unit.radius, BATTLE_ARENA_SIZE - unit.radius);
                unit.position.y = unit.position.y.clamp(unit.radius, BATTLE_ARENA_SIZE - unit.radius);
            }
        }

        ctx.db.battle_unit().id().update(unit);
    }
}

/// Commit damage and remove dead units
fn commit_damage_and_check_deaths(ctx: &ReducerContext, battle_id: u64, cache: &BattleCollisionCache) {
    for i in 0..cache.cached_count {
        let damage = cache.damage_to_unit[i];
        if damage == 0 {
            continue;
        }

        let unit_id = cache.unit_ids[i];
        let mut unit = match ctx.db.battle_unit().id().find(&unit_id) {
            Some(u) => u,
            None => continue,
        };

        if unit.current_hp > damage {
            unit.current_hp -= damage;
        } else {
            unit.current_hp = 0;
            log::info!("Unit {} died in battle {}", unit_id, battle_id);
        }

        ctx.db.battle_unit().id().update(unit);
    }
}

/// Check if battle should end
fn check_battle_end(ctx: &ReducerContext, battle_id: u64) {
    let mut side0_alive = false;
    let mut side1_alive = false;

    for unit in ctx.db.battle_unit().battle_id().filter(&battle_id) {
        if unit.current_hp > 0 {
            if unit.side == 0 {
                side0_alive = true;
            } else {
                side1_alive = true;
            }
        }
    }

    // End battle if one side is eliminated
    if !side0_alive || !side1_alive {
        let mut battle = match ctx.db.battle().id().find(&battle_id) {
            Some(b) => b,
            None => return,
        };

        battle.status = BattleStatus::Finished;

        if side0_alive {
            battle.winner = Some(battle.player1);
            log::info!("Battle {} won by player1 ({})", battle_id, battle.player1);
        } else if side1_alive {
            battle.winner = battle.player2;
            log::info!("Battle {} won by player2 ({:?})", battle_id, battle.player2);
        }

        ctx.db.battle().id().update(battle);
    }
}
