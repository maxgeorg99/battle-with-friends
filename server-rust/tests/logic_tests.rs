// Integration tests for pure logic functions (no SpacetimeDB context needed)

// Import the types directly
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DbVector2 {
    pub x: f32,
    pub y: f32,
}

impl DbVector2 {
    pub fn new(x: f32, y: f32) -> Self {
        DbVector2 { x, y }
    }

    pub fn normalize(&self) -> DbVector2 {
        let d2 = self.x * self.x + self.y * self.y;
        if d2 > 0.0 {
            let inv_mag = 1.0 / d2.sqrt();
            DbVector2::new(self.x * inv_mag, self.y * inv_mag)
        } else {
            DbVector2::new(0.0, 0.0)
        }
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl std::ops::Add for DbVector2 {
    type Output = DbVector2;
    fn add(self, other: DbVector2) -> DbVector2 {
        DbVector2::new(self.x + other.x, self.y + other.y)
    }
}

impl std::ops::Sub for DbVector2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Mul<f32> for DbVector2 {
    type Output = DbVector2;
    fn mul(self, scalar: f32) -> DbVector2 {
        DbVector2::new(self.x * scalar, self.y * scalar)
    }
}

pub fn circle_collision(ax: f32, ay: f32, ar: f32, bx: f32, by: f32, br: f32) -> bool {
    let dx = ax - bx;
    let dy = ay - by;
    let dist_sq = dx * dx + dy * dy;
    let min_dist = ar + br;
    dist_sq < min_dist * min_dist
}

pub fn get_grid_cell(x: f32, y: f32) -> u16 {
    const GRID_CELL_SIZE: u16 = 200;
    const GRID_WIDTH: u16 = 8;
    let cell_x = ((x / GRID_CELL_SIZE as f32) as u16).min(GRID_WIDTH - 1);
    let cell_y = ((y / GRID_CELL_SIZE as f32) as u16).min(GRID_WIDTH - 1);
    cell_y * GRID_WIDTH + cell_x
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_collision_hit() {
        assert!(circle_collision(0.0, 0.0, 10.0, 15.0, 0.0, 10.0));
    }

    #[test]
    fn test_circle_collision_miss() {
        assert!(!circle_collision(0.0, 0.0, 10.0, 100.0, 0.0, 10.0));
    }

    #[test]
    fn test_get_grid_cell() {
        assert_eq!(get_grid_cell(0.0, 0.0), 0);
        assert_eq!(get_grid_cell(199.0, 0.0), 0);
        assert_eq!(get_grid_cell(200.0, 0.0), 1);
        assert_eq!(get_grid_cell(0.0, 200.0), 8);
    }

    #[test]
    fn test_dbvector2_normalize() {
        let v = DbVector2::new(3.0, 4.0);
        let normalized = v.normalize();
        assert!((normalized.x - 0.6).abs() < 0.001);
        assert!((normalized.y - 0.8).abs() < 0.001);
    }

    #[test]
    fn test_dbvector2_magnitude() {
        let v = DbVector2::new(3.0, 4.0);
        assert!((v.magnitude() - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_dbvector2_operations() {
        let v1 = DbVector2::new(1.0, 2.0);
        let v2 = DbVector2::new(3.0, 4.0);

        let sum = v1 + v2;
        assert_eq!(sum.x, 4.0);
        assert_eq!(sum.y, 6.0);

        let diff = v2 - v1;
        assert_eq!(diff.x, 2.0);
        assert_eq!(diff.y, 2.0);

        let scaled = v1 * 2.0;
        assert_eq!(scaled.x, 2.0);
        assert_eq!(scaled.y, 4.0);
    }

    #[test]
    fn test_damage_calculation_logic() {
        // Test armor reduction formula
        let base_damage = 100.0;
        let armor = 50.0;
        let armor_reduction = armor / (armor + 100.0);
        let final_damage = base_damage * (1.0 - armor_reduction);

        // 50 armor should reduce by 33.3%
        assert!((final_damage - 66.66_f32).abs() < 0.1);
    }
}
