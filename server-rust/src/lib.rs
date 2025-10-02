use spacetimedb::{ReducerContext, Identity, Table};

#[spacetimedb::table(name = player, public)]
pub struct Player {
    #[primary_key]
    pub identity: Identity,
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub online: bool,
}

#[spacetimedb::reducer]
pub fn register_player(ctx: &ReducerContext, name: String) -> Result<(), String> {
    let identity = ctx.sender;
    
    if ctx.db.player().identity().find(identity).is_some() {
        return Err("Player already registered".to_string());
    }
    
    ctx.db.player().insert(Player {
        identity,
        name,
        x: 400.0,
        y: 300.0,
        online: true,
    });
    
    Ok(())
}

#[spacetimedb::reducer]
pub fn update_position(ctx: &ReducerContext, x: f32, y: f32) -> Result<(), String> {
    let identity = ctx.sender;
    
    if let Some(player) = ctx.db.player().identity().find(identity) {
        ctx.db.player().identity().update(Player {
            x,
            y,
            ..player
        });
        Ok(())
    } else {
        Err("Player not found".to_string())
    }
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