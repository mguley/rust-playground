//! Security Examples - Understanding HashDoS Attacks
//!
//! This module demonstrates why hash function choice matters for security,
//! and how different hashers protect (or fail to protect) against attacks.
//!
//! HashDoS attacks exploit predictable hash functions to cause worst-case
//! hash table performance. Understanding this threat is essential for
//! choosing the right hasher for your application.
//!
//! IMPORTANT: The examples here are educational.

use ahash::AHasher;
use nohash_hasher::BuildNoHashHasher;
use rustc_hash::FxHasher;
use std::collections::HashMap;
use std::collections::hash_map::RandomState as StdRandomState;
use std::hash::{BuildHasher, DefaultHasher, Hash, Hasher};
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
        "understanding_hashdos",
        "What happens when hash collisions are exploited",
        understanding_hashdos,
    );

    section(
        "collision_impact_demonstration",
        "Measuring the performance impact of hash collisions",
        collision_impact_demonstration,
    );

    section(
        "keyed_vs_unkeyed_hashers",
        "Why keyed hashers (SipHash, aHash) prevent prediction attacks",
        keyed_vs_unkeyed_hashers,
    );

    section(
        "vulnerable_hasher_demonstration",
        "Demonstrating why FxHash is vulnerable to HashDoS",
        vulnerable_hasher_demonstration,
    );

    section(
        "secure_hasher_demonstration",
        "How SipHash and aHash protect against HashDoS",
        secure_hasher_demonstration,
    );
}

/// Explains the mechanics of HashDoS attacks.
///
/// When an attacker can predict hash values, they can craft inputs
/// that all hash to the same bucket, turning O(1) operations into O(n).
pub fn understanding_hashdos() {
    println!("\n  Understanding HashDoS Attacks:");

    println!(
        "
    Hash tables achieve O(1) performance by distributing items across buckets:

    Normal distribution (random keys):
    ┌─────────────────────────────────────────────────────────────┐
    │ Bucket 0: [item_a]                                          │
    │ Bucket 1: [item_b, item_c]                                  │
    │ Bucket 2: [item_d]                                          │
    │ Bucket 3: [item_e]                                          │
    │ Bucket 4: [item_f, item_g]                                  │
    │ ...                                                         │
    └─────────────────────────────────────────────────────────────┘
    Lookup time: O(1) average - just hash and check one bucket

    HashDoS attack (crafted keys all collide):
    ┌─────────────────────────────────────────────────────────────┐
    │ Bucket 0: [item_a, item_b, item_c, item_d, item_e, ...]     │
    │ Bucket 1: empty                                             │
    │ Bucket 2: empty                                             │
    │ Bucket 3: empty                                             │
    │ ...                                                         │
    └─────────────────────────────────────────────────────────────┘
    Lookup time: O(n) - must scan entire chain!
    "
    );

    println!("    Impact:");
    println!("      - A single malicious HTTP request can exhaust server CPU");
    println!("      - Attack requires minimal bandwidth (small payload, huge impact)");
    println!("      - Led to CVEs and emergency patches across the industry");
}

/// Demonstrates the performance impact of hash collisions.
///
/// This simulation shows how performance degrades when items cluster
/// in the same bucket versus being well-distributed.
pub fn collision_impact_demonstration() {
    println!("\n  Collision Impact Demonstration:");

    // We'll simulate the effect of collisions by comparing lookup times
    // in a well-distributed map versus a poorly-distributed one.

    // For this demonstration, we use NoHash which lets us control distribution.
    // Keys that are multiples of the table size will cluster badly.

    let num_items: usize = 5_000;
    let num_lookups: usize = 500;

    // Well-distributed keys (sequential integers)
    let good_keys: Vec<u64> = (0..num_items as u64).collect();

    // Poorly-distributed keys (all multiples of 1024 - will cluster)
    // When table size is a power of 2, these keys hit the same buckets
    let bad_keys: Vec<u64> = (0..num_items as u64).map(|i| i * 1024).collect();

    // Build maps with NoHash (which uses keys directly as hashes)
    let mut good_map: HashMap<u64, i32, BuildNoHashHasher<u64>> = HashMap::default();
    let mut bad_map: HashMap<u64, i32, BuildNoHashHasher<u64>> = HashMap::default();

    for &key in &good_keys {
        good_map.insert(key, 1);
    }
    for &key in &bad_keys {
        bad_map.insert(key, 1);
    }

    // Measure lookup performance
    let start: Instant = Instant::now();
    for _ in 0..num_lookups {
        for &key in &good_keys {
            let _ = std::hint::black_box(good_map.get(&key));
        }
    }
    let good_time: Duration = start.elapsed();

    let start: Instant = Instant::now();
    for _ in 0..num_lookups {
        for &key in &bad_keys {
            let _ = std::hint::black_box(bad_map.get(&key));
        }
    }
    let bad_time: Duration = start.elapsed();

    println!(
        "    {} items, {} lookup iterations each:",
        num_items, num_lookups
    );
    println!("      Well-distributed keys: {:?}", good_time);
    println!("      Clustered keys:        {:?}", bad_time);

    if bad_time > good_time {
        let slowdown: f64 = bad_time.as_nanos() as f64 / good_time.as_nanos() as f64;
        println!("      Clustering caused {:.1}x slowdown!", slowdown);
    }

    println!();
    println!("    This demonstrates why key distribution matters.");
    println!("    An attacker who can control keys can exploit this.");
}

/// Explains the difference between keyed and unkeyed hashers.
///
/// Keyed hashers use a random seed, making hash values unpredictable
/// to attackers. Unkeyed hashers always produce the same output for
/// the same input, making them vulnerable to prediction attacks.
pub fn keyed_vs_unkeyed_hashers() {
    println!("\n  Keyed vs Unkeyed Hashers:");

    println!(
        "
    UNKEYED HASHERS:
    ┌─────────────────────────────────────────────────────────────┐
    │ hash(\"attack_key\") = 0x12345678  (always the same!)       │
    │                                                             │
    │ Attacker knows: If I send these specific keys, they will    │
    │ all hash to the same bucket in ANY program                  │
    └─────────────────────────────────────────────────────────────┘

    KEYED HASHERS:
    ┌─────────────────────────────────────────────────────────────┐
    │ Program A (random key 0xABCD...):                           │
    │   hash(\"attack_key\") = 0x11111111                         │
    │                                                             │
    │ Program B (different random key 0x9876...):                 │
    │   hash(\"attack_key\") = 0x99999999                         │
    │                                                             │
    │ Attacker doesn't know the key, can't predict hash values!   │
    └─────────────────────────────────────────────────────────────┘
    "
    );

    // Demonstrate with actual hashers
    println!("    Demonstration with real hashers:");

    let value: &str = "test_input";

    // FxHash - unkeyed, deterministic
    let fx_hash1: u64 = {
        let mut h: FxHasher = FxHasher::default();
        value.hash(&mut h);
        h.finish()
    };
    let fx_hash2: u64 = {
        let mut h: FxHasher = FxHasher::default();
        value.hash(&mut h);
        h.finish()
    };

    println!("      FxHash (unkeyed):");
    println!("        First call:  {:016x}", fx_hash1);
    println!("        Second call: {:016x}", fx_hash2);
    println!("        Same? {} - PREDICTABLE!", fx_hash1 == fx_hash2);

    // SipHash - keyed, random per instance
    let sip_state1: StdRandomState = StdRandomState::new();
    let sip_state2: StdRandomState = StdRandomState::new();

    let sip_hash1: u64 = {
        let mut h: DefaultHasher = sip_state1.build_hasher();
        value.hash(&mut h);
        h.finish()
    };
    let sip_hash2: u64 = {
        let mut h: DefaultHasher = sip_state2.build_hasher();
        value.hash(&mut h);
        h.finish()
    };

    println!();
    println!("      SipHash (keyed with random seed):");
    println!("        State 1: {:016x}", sip_hash1);
    println!("        State 2: {:016x}", sip_hash2);
    println!("        Same? {} - UNPREDICTABLE!", sip_hash1 == sip_hash2);
}

/// Demonstrates why FxHash is vulnerable to HashDoS.
///
/// Because FxHash is deterministic, an attacker can pre-compute
/// colliding keys offline and use them against any target.
pub fn vulnerable_hasher_demonstration() {
    println!("\n  FxHash Vulnerability Demonstration:");

    println!("    FxHash produces deterministic, predictable hashes.");
    println!("    An attacker can find colliding keys offline:");

    // Show that FxHash is completely predictable
    let test_keys: [&str; 5] = ["key1", "key2", "key3", "key4", "key5"];

    println!();
    println!("    FxHash values (same on every run, every machine):");
    for key in test_keys {
        let mut h: FxHasher = FxHasher::default();
        key.hash(&mut h);
        println!("      hash({:?}) = {:016x}", key, h.finish());
    }
}

/// Demonstrates how SipHash and aHash protect against HashDoS.
///
/// These hashers use random seeds, making it computationally infeasible
/// for attackers to predict hash values or find collisions.
pub fn secure_hasher_demonstration() {
    println!("\n  Secure Hasher Protection:");

    println!("    SipHash and aHash use random seeds from the OS.");
    println!("    Even if an attacker knows the algorithm, they can't");
    println!("    predict hash values without knowing the secret seed.");

    // Show that each HashMap gets different hash values
    let key: &str = "potentially_malicious_input";

    // SipHash - each HashMap has its own random seed
    let map1: HashMap<&str, i32> = HashMap::new();
    let map2: HashMap<&str, i32> = HashMap::new();

    let hash1: u64 = {
        let mut h: DefaultHasher = map1.hasher().build_hasher();
        key.hash(&mut h);
        h.finish()
    };
    let hash2: u64 = {
        let mut h: DefaultHasher = map2.hasher().build_hasher();
        key.hash(&mut h);
        h.finish()
    };

    println!();
    println!("    SipHash (default HashMap):");
    println!("      Map 1 hash: {:016x}", hash1);
    println!("      Map 2 hash: {:016x}", hash2);
    println!(
        "      Different? {} - each map has its own seed!",
        hash1 != hash2
    );

    // aHash - same protection
    let amap1: ahash::AHashMap<&str, i32> = ahash::AHashMap::new();
    let amap2: ahash::AHashMap<&str, i32> = ahash::AHashMap::new();

    let ahash1: u64 = {
        let mut h: AHasher = amap1.hasher().build_hasher();
        key.hash(&mut h);
        h.finish()
    };
    let ahash2: u64 = {
        let mut h: AHasher = amap2.hasher().build_hasher();
        key.hash(&mut h);
        h.finish()
    };

    println!();
    println!("    aHash:");
    println!("      Map 1 hash: {:016x}", ahash1);
    println!("      Map 2 hash: {:016x}", ahash2);
    println!(
        "      Different? {} - also uses random seeds!",
        ahash1 != ahash2
    );

    println!();
    println!("    Why this protects you:");
    println!("      - Attacker can't pre-compute collisions (unknown seed)");
    println!("      - Even if they crash one HashMap, they need new keys for others");
    println!("      - Brute-forcing collisions is computationally infeasible");
}
