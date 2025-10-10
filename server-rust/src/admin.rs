use spacetimedb::{ReducerContext, Table, log};
use crate::types::*;
use crate::tables::*;

/// Check if the caller is an admin
/// For now, we'll use a hardcoded admin identity approach
/// You can replace this with a proper admin table or role system
fn is_admin(ctx: &ReducerContext) -> bool {
    // Get player to check if they're admin
    if let Some(player) = ctx.db.player().identity().find(ctx.sender) {
        // For now, check if player name is "Admin" or similar
        // TODO: Replace with proper admin role system
        player.name == "Admin" || player.name.starts_with("Admin")
    } else {
        false
    }
}

/// Require admin access or panic
fn require_admin(ctx: &ReducerContext, operation: &str) {
    if !is_admin(ctx) {
        panic!("{}: Admin access required. Only admins can modify game balance.", operation);
    }
}

// ========== CREW TEMPLATE ADMIN REDUCERS ==========

#[spacetimedb::reducer]
pub fn admin_update_crew_template(
    ctx: &ReducerContext,
    template_id: u64,
    name: Option<String>,
    max_hp: Option<u32>,
    attack: Option<u32>,
    defense: Option<u32>,
    cost: Option<u32>,
) -> Result<(), String> {
    require_admin(ctx, "admin_update_crew_template");

    let mut template = ctx.db.crew_template().id().find(template_id)
        .ok_or("Crew template not found")?;

    // Update only provided fields
    if let Some(n) = name { template.name = n; }
    if let Some(hp) = max_hp { template.max_hp = hp; }
    if let Some(atk) = attack { template.attack = atk; }
    if let Some(def) = defense { template.defense = def; }
    if let Some(c) = cost { template.cost = c; }

    ctx.db.crew_template().id().update(template);
    log::info!("Admin updated crew template ID {}", template_id);
    Ok(())
}

// ========== ITEM COMPONENT STATS ADMIN REDUCERS ==========

#[spacetimedb::reducer]
pub fn admin_update_item_component_stats(
    ctx: &ReducerContext,
    component: ItemComponent,
    name: Option<String>,
    description: Option<String>,
    bonus_ad: Option<i32>,
    bonus_crit_chance: Option<f32>,
    bonus_attack_speed: Option<f32>,
    bonus_ap: Option<i32>,
    bonus_armor: Option<i32>,
    bonus_mr: Option<i32>,
    bonus_mana: Option<i32>,
    bonus_hp: Option<i32>,
) -> Result<(), String> {
    require_admin(ctx, "admin_update_item_component_stats");

    let mut stats = ctx.db.item_component_stats().component().find(component)
        .ok_or("Item component stats not found")?;

    // Update only provided fields
    if let Some(n) = name { stats.name = n; }
    if let Some(d) = description { stats.description = d; }
    if let Some(v) = bonus_ad { stats.bonus_ad = v; }
    if let Some(v) = bonus_crit_chance { stats.bonus_crit_chance = v; }
    if let Some(v) = bonus_attack_speed { stats.bonus_attack_speed = v; }
    if let Some(v) = bonus_ap { stats.bonus_ap = v; }
    if let Some(v) = bonus_armor { stats.bonus_armor = v; }
    if let Some(v) = bonus_mr { stats.bonus_mr = v; }
    if let Some(v) = bonus_mana { stats.bonus_mana = v; }
    if let Some(v) = bonus_hp { stats.bonus_hp = v; }

    ctx.db.item_component_stats().component().update(stats);
    log::info!("Admin updated item component stats: {:?}", component);
    Ok(())
}

// ========== COMPLETED ITEM STATS ADMIN REDUCERS ==========

#[spacetimedb::reducer]
pub fn admin_update_completed_item_stats(
    ctx: &ReducerContext,
    item: CompletedItem,
    name: Option<String>,
    description: Option<String>,
    bonus_ad: Option<i32>,
    bonus_crit_chance: Option<f32>,
    bonus_crit_damage: Option<f32>,
    bonus_attack_speed: Option<f32>,
    bonus_ap: Option<i32>,
    bonus_armor: Option<i32>,
    bonus_mr: Option<i32>,
    bonus_mana: Option<i32>,
    bonus_hp: Option<i32>,
    bonus_hp_regen: Option<f32>,
    has_splash: Option<bool>,
    armor_shred: Option<i32>,
) -> Result<(), String> {
    require_admin(ctx, "admin_update_completed_item_stats");

    let mut stats = ctx.db.completed_item_stats().item().find(item)
        .ok_or("Completed item stats not found")?;

    // Update only provided fields
    if let Some(n) = name { stats.name = n; }
    if let Some(d) = description { stats.description = d; }
    if let Some(v) = bonus_ad { stats.bonus_ad = v; }
    if let Some(v) = bonus_crit_chance { stats.bonus_crit_chance = v; }
    if let Some(v) = bonus_crit_damage { stats.bonus_crit_damage = v; }
    if let Some(v) = bonus_attack_speed { stats.bonus_attack_speed = v; }
    if let Some(v) = bonus_ap { stats.bonus_ap = v; }
    if let Some(v) = bonus_armor { stats.bonus_armor = v; }
    if let Some(v) = bonus_mr { stats.bonus_mr = v; }
    if let Some(v) = bonus_mana { stats.bonus_mana = v; }
    if let Some(v) = bonus_hp { stats.bonus_hp = v; }
    if let Some(v) = bonus_hp_regen { stats.bonus_hp_regen = v; }
    if let Some(v) = has_splash { stats.has_splash = v; }
    if let Some(v) = armor_shred { stats.armor_shred = v; }

    ctx.db.completed_item_stats().item().update(stats);
    log::info!("Admin updated completed item stats: {:?}", item);
    Ok(())
}

// ========== ITEM COMBINATION RECIPE ADMIN REDUCERS ==========

#[spacetimedb::reducer]
pub fn admin_update_recipe(
    ctx: &ReducerContext,
    recipe_id: u64,
    component1: Option<ItemComponent>,
    component2: Option<ItemComponent>,
    result: Option<CompletedItem>,
) -> Result<(), String> {
    require_admin(ctx, "admin_update_recipe");

    let mut recipe = ctx.db.item_combination_recipe().id().find(recipe_id)
        .ok_or("Recipe not found")?;

    // Update only provided fields
    if let Some(c1) = component1 { recipe.component1 = c1; }
    if let Some(c2) = component2 { recipe.component2 = c2; }
    if let Some(r) = result { recipe.result = r; }

    ctx.db.item_combination_recipe().id().update(recipe);
    log::info!("Admin updated recipe ID {}", recipe_id);
    Ok(())
}

#[spacetimedb::reducer]
pub fn admin_add_recipe(
    ctx: &ReducerContext,
    component1: ItemComponent,
    component2: ItemComponent,
    result: CompletedItem,
) -> Result<(), String> {
    require_admin(ctx, "admin_add_recipe");

    ctx.db.item_combination_recipe().insert(ItemCombinationRecipe {
        id: 0,
        component1,
        component2,
        result,
    });

    log::info!("Admin added new recipe: {:?} + {:?} = {:?}", component1, component2, result);
    Ok(())
}

#[spacetimedb::reducer]
pub fn admin_delete_recipe(
    ctx: &ReducerContext,
    recipe_id: u64,
) -> Result<(), String> {
    require_admin(ctx, "admin_delete_recipe");

    ctx.db.item_combination_recipe().id().delete(recipe_id);
    log::info!("Admin deleted recipe ID {}", recipe_id);
    Ok(())
}
