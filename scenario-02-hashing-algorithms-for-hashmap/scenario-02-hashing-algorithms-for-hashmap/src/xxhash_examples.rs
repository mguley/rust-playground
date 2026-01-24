//! xxHash Examples - The Established Performer
//!
//! xxHash is a family of extremely fast hash functions developed by
//! Yann Collet (also known for the LZ4 and Zstandard compression algorithms).
//! It's been battle-tested in production systems worldwide for over a decade.
//!
//! Key properties:
//! - Extremely fast for large data: 10+ GB/s on modern CPUs
//! - Multiple variants: xxHash32, xxHash64, xxHash3
//! - Battle-tested: used in production systems worldwide
//! - Good quality: passes SMHasher test suite
//! - NOT cryptographically secure
//!
//! Two main Rust implementations:
//! - twox-hash: Mature, stable implementation
//! - xxhash-rust: Pure Rust, more variants including xxHash3

use std::collections::HashMap;
use std::hash::{BuildHasher, BuildHasherDefault, DefaultHasher, Hash, Hasher};
use std::time::{Duration, Instant};

// Using twox-hash crate for xxHash32 and xxHash64
use twox_hash::xxhash32::Hasher as TwoxHasher32;
use twox_hash::xxhash64::Hasher as TwoxHasher64;
use twox_hash::{XxHash32, XxHash64};

use rustc_hash::FxHasher;
use std::collections::hash_map::RandomState;

// Using xxhash-rust for xxHash3 (newest, fastest variant)
use xxhash_rust::xxh3::{xxh3_64, xxh3_128};

fn section(name: &str, what: &str, f: impl FnOnce()) {
    println!("\n{:=<80}", "");
    println!("DEMO: {name}");
    println!("  {what}");
    println!("{:=<80}", "");

    f();
}

pub fn run_all() {
    section(
        "basic_xxhash_usage",
        "Use xxHash64 (twox-hash) as a HashMap hasher for fast lookups on trusted data",
        basic_xxhash_usage,
    );

    section(
        "xxhash32_usage",
        "Use xxHash32 when 32-bit hashes are sufficient (lower memory footprint)",
        xxhash32_usage,
    );

    section(
        "direct_hashing",
        "Compute xxHash values directly (strings, raw bytes, streaming/chunked hashing)",
        direct_hashing,
    );

    section(
        "seeded_hashing",
        "Seeded hashing for consistent sharding / multiple hash functions / reproducibility",
        seeded_hashing,
    );

    section(
        "performance_comparison",
        "Rough timing: xxHash64 vs SipHash vs FxHash (small keys vs large keys)",
        performance_comparison,
    );

    section(
        "xxhash3_demonstration",
        "xxHash3 (xxhash-rust): 64-bit and 128-bit, optimized for modern SIMD",
        xxhash3_demonstration,
    );

    section(
        "file_checksum_example",
        "Practical demo: incremental checksumming of chunked data",
        file_checksum_example,
    );

    section(
        "content_addressable_example",
        "Practical demo: content-addressable storage (hash-as-key, deduplication)",
        content_addressable_example,
    );
}

/// Demonstrates basic usage with xxHash64 as a HashMap hasher.
///
/// This shows how to use xxHash with Rust's standard HashMap,
/// giving you blazing fast performance for trusted data.
pub fn basic_xxhash_usage() {
    println!("\n  Basic xxHash Usage (twox-hash crate):");

    // Create a type alias for convenience.
    // XxHashMap uses xxHash64 as its hasher.
    type XxHashMap<K, V> = HashMap<K, V, BuildHasherDefault<XxHash64>>;

    let mut map: XxHashMap<String, i8> = HashMap::default();

    map.insert("one".to_string(), 1);
    map.insert("two".to_string(), 2);
    map.insert("three".to_string(), 3);

    println!("    XxHashMap: {:?}", map);

    if let Some(value) = map.get("two") {
        println!("    Get 'two': {}", value);
    }
}

/// Demonstrates using xxHash32.
///
/// xxHash32 is useful for memory-constrained systems or when
/// you only need a 32-bit hash value.
pub fn xxhash32_usage() {
    println!("\n  xxHash32 Usage:");

    // xxHash32 produces 32-bit hashes.
    // This can save memory when storing many hash values.
    type XxHash32Map<K, V> = HashMap<K, V, BuildHasherDefault<XxHash32>>;

    let mut map: XxHash32Map<&str, i8> = HashMap::default();

    map.insert("alpha", 1);
    map.insert("beta", 2);
    map.insert("gamma", 3);

    println!("    XxHash32Map: {:?}", map);

    // Directly examine the 32-bit hash values
    let mut hasher: TwoxHasher32 = XxHash32::default();
    "hello".hash(&mut hasher);
    let hash32 = hasher.finish();
    // Note: finish() returns u64, but only lower 32 bits are meaningful
    println!("    xxHash32(\"hello\") = {:08x}", hash32 as u32);

    println!();
    println!("    Use xxHash32 when:");
    println!("      - Memory is constrained");
    println!("      - 32 bits is sufficient (< 4 billion items)");
    println!("      - Compatibility with 32-bit systems");
}

/// Demonstrates computing hash values directly.
///
/// Sometimes you need the hash value itself, not just a HashMap.
/// This is common for checksums, sharding, and deduplication.
pub fn direct_hashing() {
    println!("\n  Direct Hashing with xxHash:");

    // === xxHash64 ===
    let mut hasher64: TwoxHasher64 = XxHash64::default();
    "hello world".hash(&mut hasher64);
    let hash64: u64 = hasher64.finish();
    println!("    xxHash64(\"hello world\") = {:016x}", hash64);

    // === xxHash32 ===
    let mut hasher32: TwoxHasher32 = XxHash32::default();
    "hello world".hash(&mut hasher32);
    let hash32: u64 = hasher32.finish();
    println!("    xxHash32(\"hello world\") = {:08x}", hash32 as u32);

    // === Hashing raw bytes ===
    // Sometimes you have raw bytes, not a Rust type
    let data: &[u8] = b"some binary data";
    let mut hasher: TwoxHasher64 = XxHash64::default();
    hasher.write(data);
    let hash: u64 = hasher.finish();
    println!("    xxHash64(binary data) = {:016x}", hash);

    // You can also hash in chunks (streaming mode)
    let mut streaming_hasher: TwoxHasher64 = XxHash64::default();
    streaming_hasher.write(b"some ");
    streaming_hasher.write(b"binary ");
    streaming_hasher.write(b"data");
    let streaming_hash: u64 = streaming_hasher.finish();
    println!("    xxHash64(streamed)     = {:016x}", streaming_hash);
    println!("    Same result? {}", hash == streaming_hash);
}

/// Demonstrates xxHash with a seed value.
///
/// Seeded hashing is useful for:
/// - Creating multiple independent hash functions
/// - Consistent hashing across runs (with a fixed seed)
/// - Partitioning data across shards
pub fn seeded_hashing() {
    println!("\n  Seeded xxHash:");

    // xxHash supports seeded hashing - different seeds produce
    // completely different hash outputs for the same input.
    let seed1: u64 = 12345;
    let seed2: u64 = 67890;

    let data: &str = "test data";

    // Hash with first seed
    let mut h1: TwoxHasher64 = XxHash64::with_seed(seed1);
    data.hash(&mut h1);
    let hash1: u64 = h1.finish();

    // Hash with second seed
    let mut h2: TwoxHasher64 = XxHash64::with_seed(seed2);
    data.hash(&mut h2);
    let hash2: u64 = h2.finish();

    // Hash with first seed again (should be reproducible)
    let mut h3: TwoxHasher64 = XxHash64::with_seed(seed1);
    data.hash(&mut h3);
    let hash3: u64 = h3.finish();

    println!("    Same data, different seeds:");
    println!("      Seed {}: {:016x}", seed1, hash1);
    println!("      Seed {}: {:016x}", seed2, hash2);
    println!("      Seed {} again: {:016x}", seed1, hash3);
    println!("      hash1 == hash3? {}", hash1 == hash3);

    println!();
    println!("    Use seeded hashing for:");
    println!("      - Consistent sharding (use shard number as seed)");
    println!("      - Multiple independent hash functions (Bloom filters)");
    println!("      - Reproducible results (use fixed seed)");
}

/// Compares xxHash performance to other hashers.
///
/// xxHash really shines for large data - this is where its
/// design for throughput pays off.
pub fn performance_comparison() {
    println!("\n  xxHash Performance Comparison:");

    let iterations: i32 = 500_000;

    // Build hashers
    let xx64_build: BuildHasherDefault<TwoxHasher64> = BuildHasherDefault::<XxHash64>::default();
    let siphash_build: RandomState = RandomState::new();
    let fxhash_build: BuildHasherDefault<FxHasher> = BuildHasherDefault::<FxHasher>::default();

    // === Test with small keys (integers) ===
    println!("    Small keys - integers ({} iterations):", iterations);

    let start: Instant = Instant::now();
    for i in 0..iterations {
        let mut h: TwoxHasher64 = xx64_build.build_hasher();
        i.hash(&mut h);
        let _ = std::hint::black_box(h.finish());
    }
    let xx_int_time: Duration = start.elapsed();

    let start: Instant = Instant::now();
    for i in 0..iterations {
        let mut h: DefaultHasher = siphash_build.build_hasher();
        i.hash(&mut h);
        let _ = std::hint::black_box(h.finish());
    }
    let sip_int_time: Duration = start.elapsed();

    println!("      xxHash64: {:?}", xx_int_time);
    println!("      SipHash:  {:?}", sip_int_time);

    // === Test with larger keys (xxHash shines here) ===
    println!("\n    Large keys - 1KB strings:");

    let large_key: String = "x".repeat(1024);
    let test_iterations: i32 = 100_000;

    let start: Instant = Instant::now();
    for _ in 0..test_iterations {
        let mut h: TwoxHasher64 = xx64_build.build_hasher();
        large_key.hash(&mut h);
        let _ = std::hint::black_box(h.finish());
    }
    let xx_large_time: Duration = start.elapsed();

    let start: Instant = Instant::now();
    for _ in 0..test_iterations {
        let mut h: DefaultHasher = siphash_build.build_hasher();
        large_key.hash(&mut h);
        let _ = std::hint::black_box(h.finish());
    }
    let sip_large_time: Duration = start.elapsed();

    let start: Instant = Instant::now();
    for _ in 0..test_iterations {
        let mut h: FxHasher = fxhash_build.build_hasher();
        large_key.hash(&mut h);
        let _ = std::hint::black_box(h.finish());
    }
    let fx_large_time: Duration = start.elapsed();

    println!("      xxHash64: {:?}", xx_large_time);
    println!("      SipHash:  {:?}", sip_large_time);
    println!("      FxHash:   {:?}", fx_large_time);

    let throughput_mb: f64 =
        (1024.0 * test_iterations as f64) / xx_large_time.as_secs_f64() / 1_000_000.0;
    println!("\n      xxHash64 throughput: {:.0} MB/s", throughput_mb);
    println!("      xxHash excels at large data - designed for throughput!");
}

/// Demonstrates xxHash3 from the xxhash-rust crate.
///
/// xxHash3 is the newest and fastest variant, designed to take
/// advantage of modern CPU features like SIMD.
pub fn xxhash3_demonstration() {
    println!("\n  xxHash3 (xxhash-rust crate):");

    let data: &[u8; 15] = b"Hello, xxHash3!";

    // === 64-bit hash ===
    let hash64: u64 = xxh3_64(data);
    println!("    xxh3_64:  {:016x}", hash64);

    // === 128-bit hash ===
    // 128-bit hashes are useful when you need extremely low collision probability
    let hash128: u128 = xxh3_128(data);
    println!("    xxh3_128: {:032x}", hash128);

    // xxHash3 is the newest and fastest variant
    println!();
    println!("    xxHash3 features:");
    println!("      - Fastest xxHash variant (newer algorithm)");
    println!("      - Uses SIMD when available (AVX2, SSE2)");
    println!("      - 64-bit and 128-bit output options");
    println!("      - Excellent for large data hashing");

    // Quick performance demonstration
    let large_data = vec![0xABu8; 1_000_000]; // 1 MB
    let iterations = 100;

    let start = Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(xxh3_64(&large_data));
    }
    let elapsed = start.elapsed();

    let throughput_gb =
        (large_data.len() as f64 * iterations as f64) / elapsed.as_secs_f64() / 1_000_000_000.0;
    println!("\n    1MB hashing throughput: {:.1} GB/s", throughput_gb);
}

/// Practical example: File/data checksumming.
///
/// xxHash is ideal for computing checksums for data integrity.
/// It's fast enough to verify large files without being a bottleneck.
pub fn file_checksum_example() {
    println!("\n  Practical Example: Data Checksumming");

    // Simulate checksumming chunks of data (like file blocks).
    // In real code, you'd read from a file in chunks.
    let chunks: Vec<Vec<u8>> = (0..100)
        .map(|i| vec![i as u8; 4096]) // 4KB chunks
        .collect();

    let start: Instant = Instant::now();
    let mut combined_hasher: TwoxHasher64 = XxHash64::default();

    for chunk in &chunks {
        // Incrementally hash each chunk.
        // This is efficient because xxHash maintains internal state.
        combined_hasher.write(chunk);
    }

    let checksum: u64 = combined_hasher.finish();
    let elapsed: Duration = start.elapsed();

    let total_size = chunks.len() * 4096;
    println!(
        "    Hashed {} chunks ({} KB total) in {:?}",
        chunks.len(),
        total_size / 1024,
        elapsed
    );
    println!("    Combined checksum: {:016x}", checksum);

    let throughput_mb: f64 = (total_size) as f64 / elapsed.as_secs_f64() / 1_000_000.0;
    println!("    Throughput: {:.0} MB/s", throughput_mb);
}

/// Practical example: Content-addressable storage.
///
/// Content-addressable storage uses the hash of content as its address.
/// This enables automatic deduplication - identical content has identical hash.
pub fn content_addressable_example() {
    println!("\n  Practical Example: Content-Addressable Storage");

    type ContentHash = u64;
    type ContentStore = HashMap<ContentHash, Vec<u8>, BuildHasherDefault<XxHash64>>;

    let mut store: ContentStore = HashMap::default();

    // Helper function to compute content hash
    fn compute_hash(data: &[u8]) -> ContentHash {
        xxh3_64(data)
    }

    // Store some content
    let content1: &[u8; 13] = b"Hello, World!";
    let content2: &[u8; 16] = b"Rust is awesome!";
    let content3: &[u8; 13] = b"Hello, World!"; // Intentional duplicate of content1

    let hash1: ContentHash = compute_hash(content1);
    let hash2: ContentHash = compute_hash(content2);
    let hash3: ContentHash = compute_hash(content3);

    // Store unique content
    store.insert(hash1, content1.to_vec());
    store.insert(hash2, content2.to_vec());
    // Note: content3 has the same hash as content1, so it would overwrite
    // In a real CAS, we'd check first and skip duplicates

    println!("    Stored content:");
    println!("      {:016x} -> \"Hello, World!\"", hash1);
    println!("      {:016x} -> \"Rust is awesome!\"", hash2);
    println!("      {:016x} -> (duplicate of first)", hash3);

    println!("\n    Deduplication:");
    println!("      hash1 == hash3? {}", hash1 == hash3);
    println!("      Duplicate content automatically detected!");

    // Retrieve by hash
    if let Some(data) = store.get(&hash1) {
        println!(
            "\n    Retrieved by hash: \"{}\"",
            String::from_utf8_lossy(data)
        );
    }

    println!();
    println!("    Content-addressable storage is used in:");
    println!("      - Git (blob storage)");
    println!("      - Backup systems (deduplication)");
    println!("      - Distributed file systems");
    println!("      - Docker (image layers)");
}
