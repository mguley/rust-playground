//! Foldhash Examples - The Modern Contender
//!
//! Foldhash is a modern hash function focused on quality and speed.
//! It was designed specifically for hash table use cases with modern
//! insights into what makes hash functions both fast and well-distributed.
//!
//! Key properties:
//! - Excellent hash quality: passes stringent statistical tests
//! - Fast on modern CPUs: leverages wide registers and ILP
//! - Good for varied key sizes: handles small and large keys well
//! - Uses "folding" technique: multiply to 128-bit, then XOR-fold
//! - Provides both "fast" and "quality" variants
//!
//! Foldhash aims to be a "no compromises" hasher for general use.

use ahash::{AHasher, RandomState as AHashRandomState};
use foldhash::fast::{FoldHasher, RandomState as FoldRandomState};
use foldhash::{
    HashMap as FoldHashMap, HashMapExt, HashSet as FoldHashSet, HashSetExt, SharedSeed, fast,
    quality,
};
use rustc_hash::FxHasher;
use std::collections::hash_map::RandomState as StdRandomState;
use std::hash::{BuildHasher, BuildHasherDefault, DefaultHasher, Hash, Hasher};
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
        "basic_foldhashmap_usage",
        "Basic FoldHashMap API usage (modern, high-quality hash map)",
        basic_foldhashmap_usage,
    );

    section(
        "foldhashset_usage",
        "FoldHashSet usage and set operations (membership, dedup, intersection)",
        foldhashset_usage,
    );

    section(
        "hash_quality_demonstration",
        "Inspect distribution on sequential inputs (looking for patterns)",
        hash_quality_demonstration,
    );

    section(
        "performance_comparison",
        "Rough timing: Foldhash vs aHash vs SipHash vs FxHash (not a benchmark)",
        performance_comparison,
    );

    section(
        "variants_demonstration",
        "Fast vs Quality variants and when to use each",
        variants_demonstration,
    );

    section(
        "deduplication_example",
        "Practical demo: fast deduplication with FoldHashSet",
        deduplication_example,
    );

    section(
        "group_by_example",
        "Practical demo: group-by aggregation with FoldHashMap + entry()",
        group_by_example,
    );
}

/// Demonstrates basic FoldHashMap usage.
///
/// FoldHashMap provides a modern, high-quality hash map implementation
/// that's both fast and has excellent distribution properties.
pub fn basic_foldhashmap_usage() {
    println!("\n  Basic FoldHashMap Usage:");

    // Create a new FoldHashMap using the HashMapExt trait
    let mut map: FoldHashMap<String, i8> = FoldHashMap::new();

    map.insert("one".to_string(), 1);
    map.insert("two".to_string(), 2);
    map.insert("three".to_string(), 3);

    println!("    FoldHashMap: {:?}", map);

    // Standard HashMap operations work identically
    if let Some(value) = map.get("two") {
        println!("    Get 'two': {}", value);
    }

    // With capacity - useful when you know the approximate size
    let with_cap: FoldHashMap<i8, i8> = FoldHashMap::with_capacity(1_000);
    println!("    With capacity: {}", with_cap.capacity());
}

/// Demonstrates FoldHashSet usage.
///
/// FoldHashSet provides the same benefits as FoldHashMap for
/// set operations like membership testing and deduplication.
pub fn foldhashset_usage() {
    println!("\n  FoldHashSet Usage:");

    let mut set: FoldHashSet<String> = FoldHashSet::new();

    set.insert("apple".to_string());
    set.insert("banana".to_string());
    set.insert("cherry".to_string());
    set.insert("apple".to_string()); // Duplicate, will be ignored

    println!("    FoldHashSet: {:?}", set);
    println!("    Contains 'apple': {}", set.contains("apple"));
    println!("    Contains 'grape': {}", set.contains("grape"));

    // Set operations work as expected
    let mut other: FoldHashSet<String> = FoldHashSet::new();
    other.insert("banana".to_string());
    other.insert("date".to_string());

    let intersection: FoldHashSet<_> = set.intersection(&other).cloned().collect();
    println!("    Intersection: {:?}", intersection);
}

/// Demonstrates hash quality by examining distribution.
///
/// Good hash functions should produce random-looking outputs even for
/// sequential or patterned inputs. This is crucial for hash table
/// performance because it minimizes collisions.
pub fn hash_quality_demonstration() {
    println!("\n  Foldhash Quality Demonstration:");

    let state: FoldRandomState = FoldRandomState::default();

    // Hash sequential integers and examine the outputs.
    // A poor hash function might show patterns here (like all outputs
    // differing by a constant). A good one looks random.
    println!("    Sequential integer hashes (looking for patterns):");
    let mut hashes: Vec<u64> = Vec::new();
    for i in 0..10 {
        let mut hasher: FoldHasher = state.build_hasher();
        i.hash(&mut hasher);
        let hash: u64 = hasher.finish();
        hashes.push(hash);
        // Display in hex to see bit patterns more clearly
        println!("      hash({}) = {:016x}", i, hash);
    }

    // Check for obvious patterns (good hashers should show none).
    // If all differences between consecutive hashes are the same,
    // that's a bad sign - it means the hash is just a linear function.
    let mut sequential_diffs: bool = true;
    for i in 1..hashes.len() {
        let diff: u64 = hashes[i].wrapping_sub(hashes[i - 1]);
        if diff != hashes[1].wrapping_sub(hashes[0]) {
            sequential_diffs = false;
            break;
        }
    }

    println!("\n    Pattern analysis:");
    println!(
        "      Sequential differences constant: {}",
        sequential_diffs
    );
    println!("      (Good hashers should show 'false' - random-looking output)");

    if !sequential_diffs {
        println!("      Foldhash produces well-distributed, random-looking hashes");
    }
}

/// Compares Foldhash performance to other hashers.
///
/// This benchmark helps you understand where Foldhash fits in the
/// performance spectrum relative to other popular hashers.
pub fn performance_comparison() {
    println!("\n  Foldhash Performance Comparison:");

    let iterations: i32 = 500_000;

    // Build hashers for each algorithm
    let fold_build: FoldRandomState = FoldRandomState::default();
    let ahash_build: AHashRandomState = AHashRandomState::new();
    let siphash_build: StdRandomState = StdRandomState::new();
    let fxhash_build: BuildHasherDefault<FxHasher> = BuildHasherDefault::<FxHasher>::default();

    // === Test integer hashing ===
    println!("    Integer keys ({} iterations):", iterations);

    let start: Instant = Instant::now();
    for i in 0..iterations {
        let mut h: FoldHasher = fold_build.build_hasher();
        i.hash(&mut h);
        let _ = std::hint::black_box(h.finish());
    }
    let fold_time: Duration = start.elapsed();

    let start: Instant = Instant::now();
    for i in 0..iterations {
        let mut h: AHasher = ahash_build.build_hasher();
        i.hash(&mut h);
        let _ = std::hint::black_box(h.finish());
    }
    let ahash_time: Duration = start.elapsed();

    let start: Instant = Instant::now();
    for i in 0..iterations {
        let mut h: DefaultHasher = siphash_build.build_hasher();
        i.hash(&mut h);
        let _ = std::hint::black_box(h.finish());
    }
    let siphash_time: Duration = start.elapsed();

    let start: Instant = Instant::now();
    for i in 0..iterations {
        let mut h: FxHasher = fxhash_build.build_hasher();
        i.hash(&mut h);
        let _ = std::hint::black_box(h.finish());
    }
    let fxhash_time: Duration = start.elapsed();

    println!("      Foldhash: {:?}", fold_time);
    println!("      aHash:    {:?}", ahash_time);
    println!("      SipHash:  {:?}", siphash_time);
    println!("      FxHash:   {:?}", fxhash_time);

    // === String hashing ===
    let test_strings: Vec<String> = (0..1_000)
        .map(|i| format!("test_string_key_{}", i))
        .collect();

    println!("\n    String keys ({} iterations):", iterations);

    let start: Instant = Instant::now();
    for _ in 0..iterations / 1_000 {
        for s in &test_strings {
            let mut h: FoldHasher = fold_build.build_hasher();
            s.hash(&mut h);
            let _ = std::hint::black_box(h.finish());
        }
    }
    let fold_str_time: Duration = start.elapsed();

    let start: Instant = Instant::now();
    for _ in 0..iterations / 1_000 {
        for s in &test_strings {
            let mut h: DefaultHasher = siphash_build.build_hasher();
            s.hash(&mut h);
            let _ = std::hint::black_box(h.finish());
        }
    }
    let siphash_str_time: Duration = start.elapsed();

    println!("      Foldhash: {:?}", fold_str_time);
    println!("      SipHash:  {:?}", siphash_str_time);
    println!(
        "      Speedup:  {:.2}x faster than SipHash",
        siphash_str_time.as_nanos() as f64 / fold_str_time.as_nanos() as f64
    );
}

/// Demonstrates the "fast" vs "quality" variants.
///
/// Foldhash provides two variants optimized for different use cases:
/// - fast: optimized for hash table use (default)
/// - quality: better statistical properties for sketches, bloom filters
pub fn variants_demonstration() {
    println!("\n  Fast vs Quality Variants:");

    // Both variants are available through different modules.
    // We use a shared seed to make the comparison fair.
    let shared = SharedSeed::global_fixed();
    let per_hasher_seed: u64 = 42;

    // Create both variants with the same seed
    let fast_state = fast::SeedableRandomState::with_seed(per_hasher_seed, shared);
    let quality_state = quality::SeedableRandomState::with_seed(per_hasher_seed, shared);

    let value: &str = "test";

    // Hash with both variants
    let fast_hash = fast_state.hash_one(value);
    let quality_hash = quality_state.hash_one(value);

    println!("    Same value, different variants:");
    println!("      Fast:    {:016x}", fast_hash);
    println!("      Quality: {:016x}", quality_hash);

    println!();
    println!("    When to use each variant:");
    println!("      fast:    HashMap, HashSet, general hash tables (default)");
    println!("      quality: Bloom filters, count-min sketches, HyperLogLog");
    println!();
    println!("    The 'quality' variant has better avalanche properties,");
    println!("    which matters for probabilistic data structures.");
}

/// Practical example: Fast deduplication.
///
/// Deduplication is a common operation that benefits greatly from
/// fast hash table performance. Foldhash makes this very efficient.
pub fn deduplication_example() {
    println!("\n  Practical Example: Fast Deduplication");

    // Simulate a dataset with many duplicates.
    // This is common when processing logs, events, or user actions.
    let data: Vec<String> = (0..10_000)
        .map(|i| format!("item_{}", i % 1_000)) // 10x duplicates each
        .collect();

    let start: Instant = Instant::now();

    // Deduplicate by collecting into a FoldHashSet.
    // This is a very common pattern for removing duplicates.
    let unique: FoldHashSet<String> = data.into_iter().collect();

    let elapsed: Duration = start.elapsed();

    println!(
        "    Deduplicated 10,000 items (with 10x duplicates) in {:?}",
        elapsed
    );
    println!("    Unique count: {}", unique.len());

    // Show a sample of the results
    let sample: Vec<_> = unique.iter().take(5).collect();
    println!("    Sample: {:?}", sample);
}

/// Practical example: Group-by operation.
///
/// Group-by is fundamental in data processing. FoldHashMap provides
/// fast grouping with excellent distribution properties.
pub fn group_by_example() {
    println!("\n  Practical Example: Group-By Operation");

    // Sample data representing sales records
    #[derive(Debug)]
    struct Record {
        category: String,
        value: i8,
    }

    let records: Vec<Record> = vec![
        Record {
            category: "A".to_string(),
            value: 10,
        },
        Record {
            category: "B".to_string(),
            value: 20,
        },
        Record {
            category: "A".to_string(),
            value: 30,
        },
        Record {
            category: "C".to_string(),
            value: 40,
        },
        Record {
            category: "B".to_string(),
            value: 50,
        },
        Record {
            category: "A".to_string(),
            value: 60,
        },
    ];

    // Group by category using FoldHashMap.
    // The entry() API is perfect for this pattern.
    let mut groups: FoldHashMap<String, Vec<i8>> = FoldHashMap::new();

    for record in records {
        groups
            .entry(record.category)
            .or_insert_with(Vec::new)
            .push(record.value);
    }

    println!("    Grouped records:");
    for (category, values) in &groups {
        let sum: i8 = values.iter().sum();
        let count = values.len();
        let avg = sum as f64 / count as f64;
        println!(
            "      {}: {:?} (count: {}, sum: {}, avg: {:.1})",
            category, values, count, sum, avg
        );
    }
}
