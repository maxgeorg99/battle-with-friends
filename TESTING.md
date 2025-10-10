# Testing Strategy for SpacetimeDB Modules

## Current Limitations

SpacetimeDB modules **cannot be tested with standard `cargo test`** because:
- They require WASM runtime symbols
- They need SpacetimeDB `ReducerContext` to run
- No official test harness exists yet (see [issue #2833](https://github.com/clockworklabs/SpacetimeDB/issues/2833))

## ✅ What You CAN Test

### **1. Pure Logic Functions** (Recommended)

Test math, collision detection, and business logic that doesn't need `ReducerContext`:

```bash
# Run logic tests
cargo test --test logic_tests
```

**Example test file:** `tests/logic_tests.rs`

```rust
#[test]
fn test_circle_collision() {
    assert!(circle_collision(0.0, 0.0, 10.0, 15.0, 0.0, 10.0));
}

#[test]
fn test_damage_calculation() {
    let damage = 100.0;
    let armor = 50.0;
    let reduction = armor / (armor + 100.0);
    let final_damage = damage * (1.0 - reduction);
    assert!((final_damage - 66.66).abs() < 0.1);
}
```

### **2. Manual Integration Testing** (Current Approach)

1. **Build & publish** the module to SpacetimeDB
2. **Use the client** to trigger reducers
3. **Inspect database state** with SpacetimeDB CLI

```bash
# Build module
cargo build --target wasm32-unknown-unknown --release

# Publish to local SpacetimeDB
spacetimedb publish battle-with-friends --clear-database

# Connect with client and test manually
```

## 🚧 Future: Official Test Framework

Once SpacetimeDB releases a test framework, you'll be able to:

```rust
#[spacetimedb::test]
fn test_battle_reducer() {
    // Create isolated database
    let db = TestDb::new();

    // Pre-populate data
    db.insert_player(alice, "Alice");
    db.insert_crew(alice, zoro);

    // Call reducer as client
    db.reducer(start_battle).call_as(alice);

    // Assert database state
    assert_eq!(db.battle_units().count(), 2);
}
```

## Best Practices

### ✅ DO:
- Test pure logic in `tests/` directory
- Extract complex calculations into testable functions
- Use manual testing for reducer integration
- Document test scenarios in this file

### ❌ DON'T:
- Try to run `cargo test` on files with `ReducerContext`
- Mock `ReducerContext` (it's tightly coupled to WASM runtime)
- Skip testing entirely (test what you can!)

## Test Coverage

### ✅ Currently Tested:
- Vector math (normalize, magnitude, operations)
- Collision detection (circle-circle)
- Grid cell calculation (spatial hash)
- Damage calculation formulas

### 🚧 Manual Testing Required:
- Battle reducer flow
- Crew spawning
- Attack mechanics
- Status effects
- Victory conditions

## Running Tests

```bash
# Run all testable logic
cargo test --test logic_tests

# Run specific test
cargo test --test logic_tests test_circle_collision

# Run with output
cargo test --test logic_tests -- --nocapture
```

## Adding New Tests

When adding new game logic:

1. **Extract pure functions** that don't need `ReducerContext`
2. **Add tests** to `tests/logic_tests.rs`
3. **Document** expected behavior
4. **Manual test** the reducer integration

Example:

```rust
// In battle.rs - extract pure function
pub fn calculate_crit_damage(base_damage: f32, crit_mult: f32) -> f32 {
    base_damage * crit_mult
}

// In tests/logic_tests.rs - add test
#[test]
fn test_crit_damage() {
    assert_eq!(calculate_crit_damage(100.0, 2.0), 200.0);
}
```
