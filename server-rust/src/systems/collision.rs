use crate::types::*;
use std::collections::HashMap;

pub const MAX_UNITS_PER_SIDE: usize = 15; // Max 15 units per player
pub const MAX_TOTAL_UNITS: usize = MAX_UNITS_PER_SIDE * 2; // 30 total

/// Spatial hash collision cache for efficient collision detection
/// Uses a grid-based spatial hash to only check nearby entities
pub struct BattleCollisionCache {
    // Unit data arrays
    pub unit_ids: Box<[u64]>,
    pub unit_id_to_index: HashMap<u64, usize>,
    pub pos_x: Box<[f32]>,
    pub pos_y: Box<[f32]>,
    pub radius: Box<[f32]>,
    pub side: Box<[u8]>,
    pub current_hp: Box<[u32]>,
    pub damage_to_unit: Box<[u32]>,

    // Spatial hash grid
    pub heads: Box<[i32]>,      // Head of linked list for each grid cell
    pub nexts: Box<[i32]>,      // Next entity in the linked list
    pub cell: Box<[i32]>,       // Which cell each entity is in

    pub cached_count: usize,
}

impl Default for BattleCollisionCache {
    fn default() -> Self {
        Self {
            unit_ids: vec![0; MAX_TOTAL_UNITS].into_boxed_slice(),
            unit_id_to_index: HashMap::with_capacity(MAX_TOTAL_UNITS),
            pos_x: vec![0.0; MAX_TOTAL_UNITS].into_boxed_slice(),
            pos_y: vec![0.0; MAX_TOTAL_UNITS].into_boxed_slice(),
            radius: vec![0.0; MAX_TOTAL_UNITS].into_boxed_slice(),
            side: vec![0; MAX_TOTAL_UNITS].into_boxed_slice(),
            current_hp: vec![0; MAX_TOTAL_UNITS].into_boxed_slice(),
            damage_to_unit: vec![0; MAX_TOTAL_UNITS].into_boxed_slice(),
            heads: vec![-1; NUM_GRID_CELLS as usize].into_boxed_slice(),
            nexts: vec![-1; MAX_TOTAL_UNITS].into_boxed_slice(),
            cell: vec![0; MAX_TOTAL_UNITS].into_boxed_slice(),
            cached_count: 0,
        }
    }
}

impl BattleCollisionCache {
    pub fn clear(&mut self) {
        self.cached_count = 0;
        self.unit_ids.fill(0);
        self.pos_x.fill(0.0);
        self.pos_y.fill(0.0);
        self.radius.fill(0.0);
        self.side.fill(0);
        self.current_hp.fill(0);
        self.damage_to_unit.fill(0);
        self.heads.fill(-1);
        self.nexts.fill(-1);
        self.cell.fill(0);
        self.unit_id_to_index.clear();
    }
}

/// Get grid cell from world position
#[inline]
pub fn get_grid_cell(x: f32, y: f32) -> u16 {
    let cell_x = ((x / GRID_CELL_SIZE as f32) as u16).min(GRID_WIDTH - 1);
    let cell_y = ((y / GRID_CELL_SIZE as f32) as u16).min(GRID_HEIGHT - 1);
    cell_y * GRID_WIDTH + cell_x
}

/// Check collision between two circles
#[inline]
pub fn circle_collision(ax: f32, ay: f32, ar: f32, bx: f32, by: f32, br: f32) -> bool {
    let dx = ax - bx;
    let dy = ay - by;
    let dist_sq = dx * dx + dy * dy;
    let min_dist = ar + br;
    dist_sq < min_dist * min_dist
}

/// Find nearest enemy within range
pub fn find_nearest_enemy(
    cache: &BattleCollisionCache,
    pos_x: f32,
    pos_y: f32,
    my_side: u8,
    max_range: f32,
) -> Option<usize> {
    let max_range_sq = max_range * max_range;
    let mut nearest_idx: Option<usize> = None;
    let mut nearest_dist_sq = f32::MAX;

    for i in 0..cache.cached_count {
        // Skip same side or dead units
        if cache.side[i] == my_side || cache.current_hp[i] == 0 {
            continue;
        }

        let dx = cache.pos_x[i] - pos_x;
        let dy = cache.pos_y[i] - pos_y;
        let dist_sq = dx * dx + dy * dy;

        if dist_sq < nearest_dist_sq && dist_sq <= max_range_sq {
            nearest_dist_sq = dist_sq;
            nearest_idx = Some(i);
        }
    }

    nearest_idx
}
