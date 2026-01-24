//! SipHash Examples - Rust's Default Hasher
//!
//! SipHash 1-3 is Rust's default hasher for HashMap and HashSet.
//! It was designed by Jean-Philippe Aumasson and Daniel J. Bernstein in 2012
//! specifically to address hash-flooding denial-of-service attacks.
//!
//! Key properties:
//! - Cryptographically strong: resistant to collision attacks
//! - Keyed hash: uses a random seed, making output unpredictable
//! - Consistent speed: performance doesn't vary much with input
//!
//! The "1-3" in SipHash 1-3 refers to the number of compression rounds:
//! - 1 round per block during message processing
//! - 3 rounds during finalization
//! This is a speed-optimized variant; SipHash 2-4 is more conservative.

use std::collections::HashMap;
use std::hash::{BuildHasher, DefaultHasher, Hash, Hasher, RandomState};
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
        "default_hashmap_usage",
        "Default HashMap uses RandomState (keyed SipHash) under the hood",
        default_hashmap_usage,
    );

    section(
        "examining_siphash_output",
        "Hash outputs differ strongly for small input changes (avalanche effect)",
        examining_siphash_output,
    );

    section(
        "keyed_hash_demonstration",
        "Different RandomState instances typically hash the same value differently",
        keyed_hash_demonstration,
    );

    section(
        "performance_characteristics",
        "Rough timing across key sizes (not a benchmark)",
        performance_characteristics,
    );
}

/// Demonstrates the default HashMap using SipHash.
///
/// When you write `HashMap::new()`, you get a HashMap using RandomState,
/// which creates SipHash instances with a randomly generated key.
/// This is the most common way to use HashMaps in Rust.
pub fn default_hashmap_usage() {
    println!("\n  Default HashMap with SipHash:");

    // This HashMap uses SipHash via RandomState under the hood.
    // You don't need to specify anything - it's the default choice
    // that Rust made to protect against HashDoS attacks.
    let mut scores: HashMap<String, i8> = HashMap::new();

    scores.insert("Alice".to_string(), 100);
    scores.insert("Bob".to_string(), 85);
    scores.insert("Charlie".to_string(), 92);

    println!("    Scores: {:?}", scores);

    // The same operations work as always - SipHash is transparent to the user.
    // Under the hood, each operation:
    // 1. Creates a SipHash hasher with the map's random key
    // 2. Feeds the key data through the hasher
    // 3. Uses the resulting hash to find the bucket
    if let Some(score) = scores.get("Alice") {
        println!("    Alice's score: {}", score);
    }

    // You can also create with capacity for better performance
    // when you know approximately how many items you'll store.
    let mut with_capacity: HashMap<String, i8> = HashMap::with_capacity(100);
    with_capacity.insert("test".to_string(), 42);
    println!(
        "    Map with capacity: len={}, capacity={}",
        with_capacity.len(),
        with_capacity.capacity()
    );
}

/// Demonstrates how to examine the hash value SipHash produces.
///
/// This shows the avalanche effect - small changes in input produce
/// dramatically different hash outputs. This is a hallmark of good
/// hash functions and helps ensure uniform distribution.
pub fn examining_siphash_output() {
    println!("\n  Examining SipHash Output:");

    // RandomState is the BuildHasher that creates SipHash instances.
    // Each RandomState gets its own random 128-bit key.
    let build_hasher: RandomState = RandomState::new();

    // Hash some sample values and observe the outputs
    let samples: [&str; 4] = ["hello", "hallo", "Hello", "world"];

    println!("    String hashes:");
    for &value in &samples {
        let mut hasher: DefaultHasher = build_hasher.build_hasher();
        value.hash(&mut hasher);
        let hash: u64 = hasher.finish();
        // Show the full hash value - notice how different they are
        println!("      hash({:?}) = {}", value, hash);
    }

    println!();
    println!("    Notice how similar inputs produce very different hashes.");
    println!("    'hello' vs 'hallo' - just one character changed,");
    println!("    but the hash values are completely different.");
    println!("    This is the 'avalanche effect' - a hallmark of good hash functions.");

    // Also demonstrate with integers
    println!("\n    Integer hashes:");
    for value in [0i32, 1, 42, 100, 1_000, -1] {
        let mut hasher: DefaultHasher = build_hasher.build_hasher();
        value.hash(&mut hasher);
        println!("      hash({:5}) = {}", value, hasher.finish());
    }

    // Important note about reproducibility
    println!();
    println!("    Note: Hash values will differ between program runs.");
    println!("    This unpredictability is what protects against HashDoS attacks.");
}

/// Demonstrates that SipHash is keyed (seeded with random data).
///
/// Two different RandomState instances will produce different hashes
/// for the same input, because they have different random keys.
/// This is the key security feature that prevents HashDoS attacks.
pub fn keyed_hash_demonstration() {
    println!("\n  SipHash is a Keyed Hash:");

    // Create two different RandomState instances.
    // Each gets its own random 128-bit key from the OS.
    let state1: RandomState = RandomState::new();
    let state2: RandomState = RandomState::new();

    let value: &str = "test_value";

    // Hash with first state
    let mut hasher1: DefaultHasher = state1.build_hasher();
    value.hash(&mut hasher1);
    let hash1: u64 = hasher1.finish();

    // Hash with second state (different random key)
    let mut hasher2: DefaultHasher = state2.build_hasher();
    value.hash(&mut hasher2);
    let hash2: u64 = hasher2.finish();

    println!("    Same value, different RandomState instances:");
    println!("      State 1 hash: {}", hash1);
    println!("      State 2 hash: {}", hash2);
    println!("      Are they equal? {}", hash1 == hash2);

    // Explain the security implications
    println!();
    println!("    This is why an attacker can't pre-compute colliding keys:");
    println!("    they don't know which random seed your HashMap will use.");
    println!();
    println!("    Each HashMap instance gets its own RandomState, so even");
    println!("    if an attacker crashes one HashMap with collisions,");
    println!("    they'd need different keys for each HashMap instance.");

    // Demonstrate that the same RandomState produces consistent hashes
    println!();
    println!("    However, the SAME RandomState is consistent:");
    let mut hasher3: DefaultHasher = state1.build_hasher();
    value.hash(&mut hasher3);
    let hash3: u64 = hasher3.finish();
    println!("      State 1 hash (again): {}", hash3);
    println!("      Same as before? {}", hash1 == hash3);
}

/// Demonstrates SipHash performance characteristics.
///
/// SipHash has consistent performance regardless of input patterns.
/// This is important because some faster hashers can be exploited
/// with adversarial input to cause worst-case performance.
pub fn performance_characteristics() {
    println!("\n  SipHash Performance Characteristics:");

    let build_hasher: RandomState = RandomState::new();
    let iterations: i32 = 100_000;

    // Test with various input sizes to show how SipHash scales
    let small_key: &str = "hi";
    let medium_key: &str = "hello world, this is a medium length string";
    let large_key: String = "x".repeat(1_000);

    println!("    Testing {} iterations for each key size:", iterations);

    for (name, key) in [
        ("small (2 bytes)", small_key.to_string()),
        ("medium (44 bytes)", medium_key.to_string()),
        ("large (1000 bytes)", large_key),
    ] {
        let start: Instant = Instant::now();
        for _ in 0..iterations {
            let mut hasher: DefaultHasher = build_hasher.build_hasher();
            key.hash(&mut hasher);
            // black_box prevents the compiler from optimizing away our work
            let _ = std::hint::black_box(hasher.finish());
        }
        let elapsed: Duration = start.elapsed();

        // Calculate metrics
        let ns_per_hash: f64 = elapsed.as_nanos() as f64 / iterations as f64;
        let throughput_mbps: f64 =
            (key.len() as f64 * iterations as f64) / elapsed.as_secs_f64() / 1_000_000.0;

        println!(
            "      {} key: {:.1} ns/hash, {:.1} MB/s",
            name, ns_per_hash, throughput_mbps
        );
    }
}
