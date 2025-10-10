use spacetimedb::{ReducerContext, log};

// Module declarations
pub mod types;
pub mod tables;
pub mod reducers;
pub mod systems;
pub mod admin;

// Re-export public items from modules
pub use types::*;
pub use tables::*;
pub use reducers::*;
pub use systems::*;

// Re-export battle_tick for scheduled reducer
pub use systems::battle::battle_tick;
pub use admin::*;

// ========== LIFECYCLE HOOKS ==========

/// Initialize the database - called once when the module is first published
#[spacetimedb::reducer(init)]
pub fn init(ctx: &ReducerContext) {
    log::info!("Initializing battle-with-friends database...");

    // Initialize crew template database (only happens once)
    init_crew_templates(ctx);

    // Initialize item system tables (only happens once)
    init_item_component_stats(ctx);
    init_item_combination_recipes(ctx);
    init_completed_item_stats(ctx);

    // Initialize ship upgrade system (only happens once)
    init_ship_upgrades(ctx);

    log::info!("Database initialization complete!");
}
