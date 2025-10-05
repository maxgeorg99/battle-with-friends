use spacetimedb::{ReducerContext, log};

// Module declarations
pub mod types;
pub mod tables;
pub mod crew_data;
pub mod items;
pub mod reducers;

// Re-export public items from modules
pub use types::*;
pub use tables::*;
pub use crew_data::*;
pub use items::*;
pub use reducers::*;

// ========== LIFECYCLE HOOKS ==========

/// Initialize the database - called once when the module is first published
#[spacetimedb::reducer(init)]
pub fn init(ctx: &ReducerContext) {
    log::info!("Initializing battle-with-friends database...");

    // Initialize crew template database (only happens once)
    init_crew_templates(ctx);

    log::info!("Database initialization complete!");
}
