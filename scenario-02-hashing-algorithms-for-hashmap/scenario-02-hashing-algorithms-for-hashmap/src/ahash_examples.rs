//! aHash Examples - Speed Meets Security
//!
//! aHash is designed for high performance while maintaining HashDoS resistance.
//! It uses AES-NI hardware instructions when available for extremely fast hashing.
//!
//! Key properties:
//! - Very fast: 2-10x faster than SipHash on most workloads
//! - Uses hardware AES instructions when available (AES-NI, ARM crypto)
//! - Keyed hash: uses random seeds to prevent collision attacks
//! - Quality hash: good distribution, passes SMHasher tests
//! - Falls back to software implementation on older CPUs
//!
//! aHash is a popular choice for applications that need both speed and safety.

use ahash::{AHashMap, AHashSet, AHasher, RandomState};
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
        "basic_ahashmap_usage",
        "Basic AHashMap API usage (drop-in replacement for std::HashMap, keyed by default)",
        basic_ahashmap_usage,
    );

    section(
        "ahashset_usage",
        "AHashSet usage for membership testing and deduplication",
        ahashset_usage,
    );

    section(
        "random_seeding",
        "Random seeds: different RandomState instances typically hash the same input differently",
        random_seeding,
    );

    section(
        "deterministic_ahash",
        "Fixed seeds: reproducible hashing using RandomState::with_seeds(...)",
        deterministic_ahash,
    );

    section(
        "performance_comparison",
        "Rough timing: aHash vs SipHash vs FxHash (not a real benchmark)",
        performance_comparison,
    );

    section(
        "hardware_detection",
        "Backend notes: compile-time AES selection vs runtime CPU feature detection",
        hardware_detection,
    );

    section(
        "cache_example",
        "Practical demo: high-performance cache with expiration using AHashMap",
        cache_example,
    );

    section(
        "counting_example",
        "Practical demo: word frequency counting using AHashMap",
        counting_example,
    );
}

/// Demonstrates basic AHashMap usage.
///
/// AHashMap is just HashMap with aHash as the hasher.
/// The API is identical to standard HashMap, making it easy to adopt.
pub fn basic_ahashmap_usage() {
    println!("\n  Basic AHashMap Usage:");

    // Using the provided type alias - the simplest way to use aHash.
    // AHashMap::new() creates a new map with a random seed for security.
    let mut cache: AHashMap<String, Vec<u8>> = AHashMap::new();

    cache.insert("key1".to_string(), vec![1, 2, 3]);
    cache.insert("key2".to_string(), vec![4, 5, 6]);

    println!("    AHashMap: {:?}", cache);

    // All standard HashMap operations work exactly the same
    if let Some(data) = cache.get("key1") {
        println!("    Retrieved key1: {:?}", data);
    }

    // Can also create with capacity for better performance.
    // This pre-allocates space for approximately 1_000 entries,
    // reducing the need for reallocations as the map grows.
    let with_capacity: AHashMap<i16, i16> = AHashMap::with_capacity(1_000);
    println!(
        "    With capacity: len={}, capacity={}",
        with_capacity.len(),
        with_capacity.capacity()
    );
}

/// Demonstrates AHashSet usage.
///
/// AHashSet provides the same performance benefits as AHashMap
/// for set operations like membership testing and deduplication.
pub fn ahashset_usage() {
    println!("\n  AHashSet Usage:");

    let mut seen: AHashSet<String> = AHashSet::new();

    // Typical use case: tracking seen items to detect duplicates.
    // The insert method returns false if the item was already present.
    let items: [&str; 5] = ["apple", "banana", "apple", "cherry", "banana"];

    for item in items {
        if !seen.insert(item.to_string()) {
            println!("    Duplicate detected: {}", item);
        }
    }

    println!("    Unique items: {:?}", seen);
    println!("    Count: {}", seen.len());
}

/// Demonstrates aHash's random seeding behavior.
///
/// Like SipHash, aHash uses random seeds to prevent HashDoS attacks.
/// Unlike FxHash, the same input produces different hashes across
/// different HashMap instances.
pub fn random_seeding() {
    println!("\n  aHash Uses Random Seeds:");

    // Each RandomState gets its own random seed.
    // This is the default behavior when you create an AHashMap.
    let state1: RandomState = RandomState::new();
    let state2: RandomState = RandomState::new();

    let value: &str = "test";

    // Hash with first state
    let mut h1: AHasher = state1.build_hasher();
    value.hash(&mut h1);
    let hash1: u64 = h1.finish();

    // Hash with second state (different random seed)
    let mut h2: AHasher = state2.build_hasher();
    value.hash(&mut h2);
    let hash2: u64 = h2.finish();

    println!("    Same value, different RandomState:");
    println!("      State 1: {}", hash1);
    println!("      State 2: {}", hash2);
    println!("      Equal? {} (expected: false)", hash1 == hash2);

    println!();
    println!("    This randomness prevents attackers from pre-computing");
    println!("    colliding keys, similar to how SipHash protects you.");
    println!("    But aHash does it much faster!");
}

/// Demonstrates aHash with fixed seeds for reproducible results.
///
/// For testing or when you need deterministic behavior (like
/// reproducible builds), aHash allows creating hashers with specific seeds.
pub fn deterministic_ahash() {
    println!("\n  Deterministic aHash (Fixed Seeds):");

    // Create two RandomStates with the same seeds.
    // The four u64 values are the seed material for the hasher.
    let state1: RandomState = RandomState::with_seeds(1, 2, 3, 4);
    let state2: RandomState = RandomState::with_seeds(1, 2, 3, 4);

    let value: &str = "reproducible";

    // Hash with first state
    let mut h1: AHasher = state1.build_hasher();
    value.hash(&mut h1);
    let hash1: u64 = h1.finish();

    // Hash with second state (same seeds)
    let mut h2: AHasher = state2.build_hasher();
    value.hash(&mut h2);
    let hash2: u64 = h2.finish();

    println!("    With identical seeds:");
    println!("      Hash 1: {}", hash1);
    println!("      Hash 2: {}", hash2);
    println!("      Equal? {}", hash1 == hash2);

    // Different seeds produce different hashes
    let state3: RandomState = RandomState::with_seeds(5, 6, 7, 8);
    let mut h3: AHasher = state3.build_hasher();
    value.hash(&mut h3);
    let hash3: u64 = h3.finish();

    println!("\n    With different seeds:");
    println!("      Hash 3: {}", hash3);
    println!("      Equal to hash1? {}", hash1 == hash3);

    println!();
    println!("    Use fixed seeds for:");
    println!("    - Unit tests that need deterministic behavior");
    println!("    - Reproducible builds");
    println!("    - Debugging hash-related issues");
}

/// Compares aHash performance to SipHash and FxHash.
///
/// This demonstrates why aHash is a good middle ground: it's much
/// faster than SipHash while still providing security.
pub fn performance_comparison() {
    println!("\n  aHash Performance Comparison:");

    let iterations: i32 = 500_000;

    // Build hashers for each algorithm
    let ahash_build: RandomState = RandomState::new();
    let siphash_build: StdRandomState = StdRandomState::new();
    let fxhash_build: BuildHasherDefault<FxHasher> = BuildHasherDefault::<FxHasher>::default();

    // === Test integer hashing ===
    println!("    Integer keys ({} iterations):", iterations);

    // aHash timing
    let start: Instant = Instant::now();
    for i in 0..iterations {
        let mut h: AHasher = ahash_build.build_hasher();
        i.hash(&mut h);
        let _ = std::hint::black_box(h.finish());
    }
    let ahash_time: Duration = start.elapsed();

    // SipHash timing
    let start: Instant = Instant::now();
    for i in 0..iterations {
        let mut h: DefaultHasher = siphash_build.build_hasher();
        i.hash(&mut h);
        let _ = std::hint::black_box(h.finish());
    }
    let siphash_time: Duration = start.elapsed();

    // FxHash timing
    let start: Instant = Instant::now();
    for i in 0..iterations {
        let mut h: FxHasher = fxhash_build.build_hasher();
        i.hash(&mut h);
        let _ = std::hint::black_box(h.finish());
    }
    let fxhash_time: Duration = start.elapsed();

    println!("      aHash:   {:?}", ahash_time);
    println!("      SipHash: {:?}", siphash_time);
    println!("      FxHash:  {:?}", fxhash_time);
    println!(
        "      aHash vs SipHash: {:.2}x faster",
        siphash_time.as_nanos() as f64 / ahash_time.as_nanos() as f64
    );

    // === Test string hashing ===
    let test_strings: Vec<String> = (0..1_000).map(|i| format!("string_key_{}", i)).collect();

    println!("\n    String keys ({} iterations):", iterations);

    let start: Instant = Instant::now();
    for _ in 0..iterations / 1_000 {
        for s in &test_strings {
            let mut h: AHasher = ahash_build.build_hasher();
            s.hash(&mut h);
            let _ = std::hint::black_box(h.finish());
        }
    }
    let ahash_str_time: Duration = start.elapsed();

    let start: Instant = Instant::now();
    for _ in 0..iterations / 1_000 {
        for s in &test_strings {
            let mut h: DefaultHasher = siphash_build.build_hasher();
            s.hash(&mut h);
            let _ = std::hint::black_box(h.finish());
        }
    }
    let siphash_str_time: Duration = start.elapsed();

    println!("      aHash:   {:?}", ahash_str_time);
    println!("      SipHash: {:?}", siphash_str_time);
    println!(
        "      aHash vs SipHash: {:.2}x faster",
        siphash_str_time.as_nanos() as f64 / ahash_str_time.as_nanos() as f64
    );
}

/// Demonstrates CPU capability (runtime) vs what aHash can actually use (compile-time).
///
/// aHash uses AES-NI instructions when available for maximum speed.
/// Understanding this helps you optimize your builds.
pub fn hardware_detection() {
    println!("\n  aHash Hardware / Backend Notes:");

    // aHash selects its backend at compile time based on target features.
    // This means the decision is made when you compile, not when you run.
    let ahash_aes_backend_compiled_in: bool = cfg!(all(
        any(target_arch = "x86", target_arch = "x86_64"),
        target_feature = "aes",
        not(miri)
    ));

    println!(
        "    aHash AES backend compiled in: {}",
        ahash_aes_backend_compiled_in
    );

    // On x86/x86_64, we can check if the CPU actually supports AES-NI
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        let cpu_has_aes: bool = std::arch::is_x86_feature_detected!("aes");
        println!("    CPU advertises AES-NI (runtime): {}", cpu_has_aes);

        if cpu_has_aes && !ahash_aes_backend_compiled_in {
            println!();
            println!("    Note: CPU supports AES-NI, but this binary wasn't compiled");
            println!("    with +aes, so aHash will use its fallback backend.");
            println!();
            println!("    To enable AES acceleration, compile with:");
            println!("      RUSTFLAGS='-C target-feature=+aes' cargo build --release");
            println!("    Or add to .cargo/config.toml:");
            println!("      [build]");
            println!("      rustflags = [\"-C\", \"target-feature=+aes\"]");
        }
    }

    #[cfg(target_arch = "aarch64")]
    {
        let cpu_has_aes = std::arch::is_aarch64_feature_detected!("aes");
        println!("    CPU advertises ARM AES (runtime): {}", cpu_has_aes);
        println!("    Note: aHash 0.8.x documents acceleration as x86/x86_64-only.");
    }
}

/// Practical example: High-performance cache with expiration.
///
/// aHash is ideal for caches that need both speed and safety,
/// like web server response caches.
pub fn cache_example() {
    println!("\n  Practical Example: High-Performance Cache");

    // A simple time-based cache entry
    struct CacheEntry<V> {
        value: V,
        expires_at: Instant,
    }

    // A cache that automatically expires old entries
    struct Cache<K, V> {
        entries: AHashMap<K, CacheEntry<V>>,
        default_ttl: Duration,
    }

    impl<K: Hash + Eq, V> Cache<K, V> {
        fn new(ttl: Duration) -> Self {
            Cache {
                entries: AHashMap::new(),
                default_ttl: ttl,
            }
        }

        fn insert(&mut self, key: K, value: V) {
            self.entries.insert(
                key,
                CacheEntry {
                    value,
                    expires_at: Instant::now() + self.default_ttl,
                },
            );
        }

        fn get(&self, key: &K) -> Option<&V> {
            self.entries.get(key).and_then(|entry| {
                if Instant::now() < entry.expires_at {
                    Some(&entry.value)
                } else {
                    None // Entry has expired
                }
            })
        }

        fn len(&self) -> usize {
            self.entries.len()
        }
    }

    // Create a cache with 60 second TTL
    let mut cache: Cache<String, String> = Cache::new(Duration::from_secs(60));

    // Simulate caching API responses
    cache.insert("user:123".to_string(), "Alice".to_string());
    cache.insert("user:456".to_string(), "Bob".to_string());

    println!("    Cache size: {}", cache.len());
    println!("    Get user:123: {:?}", cache.get(&"user:123".to_string()));
    println!("    Get user:789: {:?}", cache.get(&"user:789".to_string()));

    println!();
    println!("    aHash makes this cache fast while protecting against");
    println!("    attackers who might try to cause cache collisions.");
}

/// Practical example: Word frequency counting.
///
/// Counting word frequencies is a common task that benefits from
/// fast hash table operations.
pub fn counting_example() {
    println!("\n  Practical Example: Word Frequency Counter");

    let text: &str = "the quick brown fox jumps over the lazy dog the fox is quick";

    // Use AHashMap for fast counting
    let mut counts: AHashMap<&str, u32> = AHashMap::new();

    for word in text.split_whitespace() {
        // entry() API is very efficient for counting patterns
        *counts.entry(word).or_insert(0) += 1;
    }

    // Sort by count for display (most frequent first)
    let mut sorted: Vec<_> = counts.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));

    println!("    Word frequencies (top 5):");
    for (word, count) in sorted.iter().take(5) {
        println!("      {}: {}", word, count);
    }

    println!("\n    Total unique words: {}", counts.len());
}
