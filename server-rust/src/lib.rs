use spacetimedb::{ReducerContext, log};

// Module declarations
pub mod types;
pub mod tables;
pub mod reducers;
pub mod systems;

// Re-export public items from modules
pub use types::*;
pub use tables::*;
pub use reducers::*;
pub use systems::*;

// ========== LIFECYCLE HOOKS ==========

/// Initialize the database - called once when the module is first published
#[spacetimedb::reducer(init)]
pub fn init(ctx: &ReducerContext) {
    log::info!("Initializing battle-with-friends database...");

    // Initialize crew template database (only happens once)
    init_crew_templates(ctx);

    log::info!("Database initialization complete!");
}
