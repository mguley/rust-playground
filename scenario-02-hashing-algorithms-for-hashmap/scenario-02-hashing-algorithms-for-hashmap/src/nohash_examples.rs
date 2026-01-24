//! NoHash Examples - When Hashing Is Unnecessary
//!
//! NoHash is a "hasher" that doesn't actually hash - it uses integer
//! values directly as their hash. This is optimal when keys are already
//! well-distributed integers.
//!
//! Key properties:
//! - Zero hashing overhead: just uses the integer value directly
//! - Only works for integer types (via IsEnabled trait)
//! - Requires well-distributed keys to work well
//! - Can cause severe performance degradation with clustered keys
//!
//! Supported types: i8, i16, i32, i64, isize, u8, u16, u32, u64, usize
//! NOT supported by default: i128, u128

use nohash_hasher::{BuildNoHashHasher, IntMap, IntSet, IsEnabled, NoHashHasher};
use rustc_hash::FxHasher;
use std::collections::HashMap;
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, BuildHasherDefault, Hash, Hasher};
use std::time::{Duration, Instant};

fn section(name: &str, what: &str, f: impl FnOnce()) {
    println!("\n{:=<80}", "");
    println!("DEMO: {name}");
    println!("  {what}");
    println!("{:=<80}", "");

    f();
}

pub fn run_all() {
    section(
        "basic_intmap_usage",
        "Basic IntMap usage (integer keys, zero hashing overhead)",
        basic_intmap_usage,
    );

    section(
        "intset_usage",
        "IntSet usage for tracking seen integer IDs and deduplication",
        intset_usage,
    );

    section(
        "performance_comparison",
        "Rough timing: NoHash vs FxHash vs SipHash for integer hashing",
        performance_comparison,
    );

    section(
        "good_key_distribution",
        "When NoHash works well: already well-distributed integer keys",
        good_key_distribution,
    );

    section(
        "poor_key_distribution",
        "When NoHash performs poorly: clustered keys (power-of-two patterns)",
        poor_key_distribution,
    );

    section(
        "custom_type_with_nohash",
        "Using NoHash with custom ID wrapper types via IsEnabled",
        custom_type_with_nohash,
    );

    section(
        "ecs_example",
        "Practical demo: ECS-style component storage with IntMap lookups",
        ecs_example,
    );
}

/// Demonstrates basic IntMap usage.
///
/// IntMap is a HashMap that uses NoHash - it only accepts integer keys
/// and uses them directly as hash values, eliminating hashing overhead.
pub fn basic_intmap_usage() {
    println!("\n  Basic IntMap Usage (NoHash):");

    // IntMap only accepts integer keys - this is enforced at compile time.
    // The integer value IS the hash value, so there's no hashing overhead.
    let mut entity_names: IntMap<u8, String> = IntMap::default();

    entity_names.insert(1, "Player".to_string());
    entity_names.insert(2, "Enemy".to_string());
    entity_names.insert(3, "NPC".to_string());

    println!("    IntMap<u8, String>: {:?}", entity_names);

    // Standard HashMap operations work identically
    if let Some(name) = entity_names.get(&2) {
        println!("    Entity 2: {}", name);
    }

    // Different integer types work too
    let mut large_ids: IntMap<u64, &str> = IntMap::default();
    large_ids.insert(1_000_000_001, "item_a");
    large_ids.insert(1_000_000_002, "item_b");
    println!("    IntMap<u64, &str>: {} entries", large_ids.len());
}

/// Demonstrates IntSet usage.
///
/// IntSet is a HashSet that uses NoHash - perfect for tracking
/// which integer IDs you've seen.
pub fn intset_usage() {
    println!("\n  IntSet Usage:");

    // IntSet for tracking seen IDs
    let mut seen_ids: IntSet<u16> = IntSet::default();

    seen_ids.insert(1_001);
    seen_ids.insert(1_002);
    seen_ids.insert(1_003);
    seen_ids.insert(1_001); // Duplicate, will be ignored

    println!("    IntSet<u16>: {:?}", seen_ids);
    println!("    Contains 1002: {}", seen_ids.contains(&1_002));
    println!("    Contains 9999: {}", seen_ids.contains(&9_999));

    // Common use case: tracking processed items
    let items_to_process = [1001, 1002, 1003, 1001, 1004, 1002];
    let mut processed: IntSet<u16> = IntSet::default();

    println!("\n    Processing items:");
    for &item in &items_to_process {
        if processed.insert(item) {
            println!("      Processing item {}", item);
        } else {
            println!("      Skipping {} (already processed)", item);
        }
    }
}

/// Demonstrates the performance advantage of NoHash.
///
/// When you eliminate hashing entirely, you get the fastest possible
/// HashMap performance for integer keys.
pub fn performance_comparison() {
    println!("\n  NoHash Performance Comparison:");

    let iterations: u64 = 1_000_000;

    // Build hashers for comparison
    let nohash_build: BuildHasherDefault<NoHashHasher<u64>> = BuildNoHashHasher::<u64>::default();
    let siphash_build: RandomState = RandomState::new();
    let fxhash_build: BuildHasherDefault<FxHasher> = BuildHasherDefault::<FxHasher>::default();

    println!("    Integer key hashing ({} iterations):", iterations);

    // NoHash timing - should be fastest
    let start: Instant = Instant::now();
    for i in 0u64..iterations {
        let mut h: NoHashHasher<u64> = nohash_build.build_hasher();
        i.hash(&mut h);
        let _ = std::hint::black_box(h.finish());
    }
    let nohash_time: Duration = start.elapsed();

    // SipHash timing (default)
    let start: Instant = Instant::now();
    for i in 0u64..iterations {
        let mut h = siphash_build.build_hasher();
        i.hash(&mut h);
        let _ = std::hint::black_box(h.finish());
    }
    let siphash_time: Duration = start.elapsed();

    // FxHash timing
    let start: Instant = Instant::now();
    for i in 0u64..iterations {
        let mut h: FxHasher = fxhash_build.build_hasher();
        i.hash(&mut h);
        let _ = std::hint::black_box(h.finish());
    }
    let fxhash_time: Duration = start.elapsed();

    println!("      NoHash:  {:?}", nohash_time);
    println!("      FxHash:  {:?}", fxhash_time);
    println!("      SipHash: {:?}", siphash_time);
    println!(
        "\n      NoHash speedup vs SipHash: {:.1}x",
        siphash_time.as_nanos() as f64 / nohash_time.as_nanos() as f64
    );
    println!(
        "      NoHash speedup vs FxHash: {:.1}x",
        fxhash_time.as_nanos() as f64 / nohash_time.as_nanos() as f64
    );
}

/// Demonstrates when NoHash works well.
///
/// NoHash works great when your integer keys are already reasonably
/// distributed. This includes sequential IDs, random IDs, and pre-hashed values.
pub fn good_key_distribution() {
    println!("\n  NoHash Works Well With:");

    // === 1. Sequential IDs (like database primary keys) ===
    // Sequential IDs are already well-distributed for hash table purposes
    // because they spread across buckets evenly.
    let mut db_records: IntMap<u64, &str> = IntMap::default();
    for id in 1..=100u64 {
        db_records.insert(id, "record");
    }
    println!(
        "    1. Sequential IDs (1, 2, 3, ...): {} entries",
        db_records.len()
    );

    // === 2. Random IDs ===
    // Random values are inherently well-distributed
    let random_state: RandomState = RandomState::new();
    let mut random_ids: IntMap<u64, &str> = IntMap::default();
    for i in 0..100 {
        // Generate a pseudo-random ID
        let mut h = random_state.build_hasher();
        i.hash(&mut h);
        let random_id: u64 = h.finish();
        random_ids.insert(random_id, "random");
    }
    println!("    2. Random IDs: {} entries", random_ids.len());

    // === 3. UUIDs as u64 (high bits) ===
    // If you have UUIDs, taking the high or low 64 bits works well
    // because UUIDs are designed to be unique and well-distributed.
    let mut uuid_map: IntMap<u64, &str> = IntMap::default();
    // Simulating UUIDs with a spreading multiplier
    for i in 0u64..100 {
        let fake_uuid: u64 = i.wrapping_mul(0x9E37_79B9_7F4A_7C15u64);
        uuid_map.insert(fake_uuid, "uuid_data");
    }
    println!("    3. UUID-like values: {} entries", uuid_map.len());

    // === 4. Pre-hashed values ===
    // If you've already computed a hash elsewhere (e.g., file content hash),
    // just use that hash directly as the key!
    println!("    4. Pre-hashed values (already distributed)");
}

/// Demonstrates when NoHash performs poorly.
///
/// NoHash can cause severe performance degradation when keys cluster.
/// This happens with certain patterns like multiples of powers of 2.
pub fn poor_key_distribution() {
    println!("\n  NoHash Performs Poorly With Clustered Keys:");
    println!("    (This demonstrates the danger of using NoHash carelessly)");

    // Measure lookup time with clustered keys vs sequential keys
    println!("\n    Measuring lookup time with 1000 keys:");

    // === Clustered keys (multiples of 64) ===
    // Keys that are multiples of a power of 2 can cluster badly
    // because hash table sizes are typically powers of 2.
    let clustered_keys: Vec<u64> = (0..1_000).map(|i| i * 64).collect();

    let mut bad_map: IntMap<u64, i32> = IntMap::default();
    for &key in &clustered_keys {
        bad_map.insert(key, 1);
    }

    let start: Instant = Instant::now();
    for _ in 0..10_000 {
        for &key in &clustered_keys {
            let _ = std::hint::black_box(bad_map.get(&key));
        }
    }
    let clustered_time: Duration = start.elapsed();

    // === Sequential keys (well-distributed) ===
    let sequential_keys: Vec<u64> = (0..1_000).collect();

    let mut good_map: IntMap<u64, i32> = IntMap::default();
    for &key in &sequential_keys {
        good_map.insert(key, 1);
    }

    let start: Instant = Instant::now();
    for _ in 0..10_000 {
        for &key in &sequential_keys {
            let _ = std::hint::black_box(good_map.get(&key));
        }
    }
    let sequential_time: Duration = start.elapsed();

    println!(
        "      Clustered keys (multiples of 64): {:?}",
        clustered_time
    );
    println!(
        "      Sequential keys (0, 1, 2, ...):   {:?}",
        sequential_time
    );

    if clustered_time > sequential_time {
        println!(
            "      Clustering caused {:.1}x slowdown!",
            clustered_time.as_nanos() as f64 / sequential_time.as_nanos() as f64
        );
    }

    println!();
    println!("     ️  Avoid using NoHash with:");
    println!("       - Multiples of powers of 2 (8, 16, 32, 64, ...)");
    println!("       - Pointer addresses (often 8 or 16-byte aligned)");
    println!("       - Any systematically clustered values");
}

/// Demonstrates using NoHash with custom wrapper types.
///
/// If you have a custom type that wraps an integer, you can enable
/// NoHash for it by implementing the IsEnabled marker trait.
pub fn custom_type_with_nohash() {
    println!("\n  Using NoHash with Custom Types:");

    // Your type must implement IsEnabled to use with NoHash.
    // This is a safety guard to prevent accidental misuse.

    // Example: Entity ID wrapper type
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct EntityId(u32);

    // Implement Hash to use the inner value directly
    impl Hash for EntityId {
        fn hash<H: Hasher>(&self, state: &mut H) {
            // Just hash the inner u32 - NoHash will use it directly
            self.0.hash(state);
        }
    }

    // Implement IsEnabled to allow use with NoHash.
    // This is a marker trait that says "I know what I'm doing -
    // my type's Hash implementation writes exactly one integer."
    impl IsEnabled for EntityId {}

    // Now we can use it with NoHash!
    type EntityMap<V> = HashMap<EntityId, V, BuildNoHashHasher<EntityId>>;

    let mut entities: EntityMap<String> = HashMap::default();

    entities.insert(EntityId(1), "Player".to_string());
    entities.insert(EntityId(2), "Enemy".to_string());
    entities.insert(EntityId(100), "Boss".to_string());

    println!("    EntityMap with custom EntityId:");
    for (id, name) in &entities {
        println!("      {:?} -> {}", id, name);
    }

    println!();
    println!("    Custom types with NoHash are useful for:");
    println!("      - Type-safe ID wrappers (prevent mixing different ID types)");
    println!("      - Newtype patterns with zero-cost abstraction");
}

/// Practical example: Entity Component System (ECS).
///
/// In an ECS, entities are identified by integer IDs, and components
/// are looked up by entity ID. This is a perfect use case for NoHash
/// because entity IDs are typically sequential integers.
pub fn ecs_example() {
    println!("\n  Practical Example: Entity Component System (ECS)");

    // In game engines, an ECS stores components in separate maps
    // indexed by entity ID. NoHash makes these lookups extremely fast.

    // Component types
    #[derive(Debug)]
    struct Position {
        x: f32,
        y: f32,
    }

    #[derive(Debug)]
    struct Velocity {
        dx: f32,
        dy: f32,
    }

    #[derive(Debug)]
    struct Health {
        current: i32,
        max: i32,
    }

    // A simple ECS world using IntMaps for component storage
    struct World {
        positions: IntMap<u32, Position>,
        velocities: IntMap<u32, Velocity>,
        healths: IntMap<u32, Health>,
        next_entity: u32,
    }

    impl World {
        fn new() -> Self {
            World {
                positions: IntMap::default(),
                velocities: IntMap::default(),
                healths: IntMap::default(),
                next_entity: 0,
            }
        }

        // Spawn a new entity and return its ID
        fn spawn_entity(&mut self) -> u32 {
            let id: u32 = self.next_entity;
            self.next_entity += 1;
            id
        }

        // Movement system: update positions based on velocities
        fn movement_system(&mut self) {
            for (&entity, pos) in self.positions.iter_mut() {
                if let Some(vel) = self.velocities.get(&entity) {
                    pos.x += vel.dx;
                    pos.y += vel.dy;
                }
            }
        }
    }

    let mut world: World = World::new();

    // Spawn some entities with components
    let player: u32 = world.spawn_entity();
    world.positions.insert(player, Position { x: 0.0, y: 0.0 });
    world
        .velocities
        .insert(player, Velocity { dx: 1.0, dy: 0.0 });
    world.healths.insert(
        player,
        Health {
            current: 100,
            max: 100,
        },
    );

    let enemy: u32 = world.spawn_entity();
    world.positions.insert(enemy, Position { x: 10.0, y: 5.0 });
    world
        .velocities
        .insert(enemy, Velocity { dx: -0.5, dy: 0.0 });
    world.healths.insert(
        enemy,
        Health {
            current: 50,
            max: 50,
        },
    );

    println!("    Initial state:");
    println!(
        "      Player (entity {}): pos={:?}, health={:?}",
        player,
        world.positions.get(&player),
        world.healths.get(&player)
    );
    println!(
        "      Enemy (entity {}): pos={:?}, health={:?}",
        enemy,
        world.positions.get(&enemy),
        world.healths.get(&enemy)
    );

    // Run the movement system
    world.movement_system();

    println!("\n    After running movement system:");
    println!(
        "      Player (entity {}): pos={:?}",
        player,
        world.positions.get(&player)
    );
    println!(
        "      Enemy (entity {}): pos={:?}",
        enemy,
        world.positions.get(&enemy)
    );

    println!();
    println!("    NoHash is ideal for ECS because:");
    println!("      - Entity IDs are sequential integers (well-distributed)");
    println!("      - Component lookups happen millions of times per frame");
    println!("      - Zero hashing overhead means maximum performance");
}
