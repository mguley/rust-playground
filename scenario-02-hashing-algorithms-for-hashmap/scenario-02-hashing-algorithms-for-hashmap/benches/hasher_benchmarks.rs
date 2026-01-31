//! benches/hasher_benchmarks.rs
//!
//! Benchmarks for comparing hash function performance.
//!
//! These benchmarks measure:
//!   1. Raw hashing throughput (bytes/second)
//!   2. HashMap insertion performance
//!   3. HashMap lookup performance
//!   4. Performance across different key sizes
//!   5. Performance with different key types
//!
//! To run these benchmarks:
//!   cargo bench
//!
//! To run a specific benchmark group:
//!   cargo bench -- Hashing
//!   cargo bench -- HashMap_Insert
//!   cargo bench -- HashMap_Lookup
//!
//! Results are saved to target/criterion/ with HTML reports.

use criterion::measurement::WallTime;
use criterion::{
    BenchmarkGroup, BenchmarkId, Criterion, Throughput, criterion_group, criterion_main,
};
use std::collections::HashMap;
use std::hash::{BuildHasher, BuildHasherDefault, DefaultHasher, Hash, Hasher};
use std::hint::black_box;

// Import all the hashers we're comparing
use ahash::{AHashMap, AHasher, RandomState as AHashRandomState};
use foldhash::fast::{FoldHasher, RandomState as FoldRandomState};
use foldhash::{HashMap as FoldHashMap, HashMapExt};
use nohash_hasher::{BuildNoHashHasher, IntMap, NoHashHasher};
use rustc_hash::{FxHashMap, FxHasher};
use std::collections::hash_map::RandomState as StdRandomState;
use twox_hash::XxHash64;
use xxhash_rust::xxh3::xxh3_64;

// ============================================================================
// RAW HASHING BENCHMARKS
// ============================================================================
// Measures the raw throughput of each hash function without HashMap overhead.
// This isolates the hash function performance from table operations.

fn bench_raw_hashing(c: &mut Criterion) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("Raw_Hashing");

    // Test with different key sizes to see how hashers scale
    for size in [8, 64, 256, 1024, 4096] {
        let data: Vec<u8> = (0..size).map(|i| i as u8).collect();
        group.throughput(Throughput::Bytes(size as u64));

        // SipHash (default)
        group.bench_with_input(BenchmarkId::new("SipHash", size), &data, |b, data| {
            let state: StdRandomState = StdRandomState::new();
            b.iter(|| {
                let mut h: DefaultHasher = state.build_hasher();
                data.hash(&mut h);
                black_box(h.finish())
            })
        });

        // FxHash
        group.bench_with_input(BenchmarkId::new("FxHash", size), &data, |b, data| {
            let state: BuildHasherDefault<FxHasher> = BuildHasherDefault::default();
            b.iter(|| {
                let mut h: FxHasher = state.build_hasher();
                data.hash(&mut h);
                black_box(h.finish())
            })
        });

        // aHash
        group.bench_with_input(BenchmarkId::new("aHash", size), &data, |b, data| {
            let state: AHashRandomState = AHashRandomState::new();
            b.iter(|| {
                let mut h: AHasher = state.build_hasher();
                data.hash(&mut h);
                black_box(h.finish())
            })
        });

        // Foldhash
        group.bench_with_input(BenchmarkId::new("Foldhash", size), &data, |b, data| {
            let state: FoldRandomState = FoldRandomState::default();
            b.iter(|| {
                let mut h: FoldHasher = state.build_hasher();
                data.hash(&mut h);
                black_box(h.finish())
            })
        });

        // xxHash64 (twox-hash)
        group.bench_with_input(BenchmarkId::new("xxHash64", size), &data, |b, data| {
            let state: BuildHasherDefault<XxHash64> = BuildHasherDefault::default();
            b.iter(|| {
                let mut h = state.build_hasher();
                data.hash(&mut h);
                black_box(h.finish())
            })
        });

        // xxHash3 (xxhash-rust) - direct API for comparison
        group.bench_with_input(BenchmarkId::new("xxHash3", size), &data, |b, data| {
            b.iter(|| black_box(xxh3_64(data)))
        });
    }

    group.finish();
}

// ============================================================================
// INTEGER KEY BENCHMARKS
// ============================================================================
// Measures performance with integer keys - the ideal case for NoHash.

fn bench_integer_hashing(c: &mut Criterion) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("Integer_Hashing");

    let iterations: u64 = 100_000;
    group.throughput(Throughput::Elements(iterations));

    // SipHash
    group.bench_function("SipHash", |b| {
        let state: StdRandomState = StdRandomState::new();
        b.iter(|| {
            for i in 0u64..iterations {
                let mut h: DefaultHasher = state.build_hasher();
                i.hash(&mut h);
                black_box(h.finish());
            }
        })
    });

    // FxHash
    group.bench_function("FxHash", |b| {
        let state: BuildHasherDefault<FxHasher> = BuildHasherDefault::default();
        b.iter(|| {
            for i in 0u64..iterations {
                let mut h: FxHasher = state.build_hasher();
                i.hash(&mut h);
                black_box(h.finish());
            }
        })
    });

    // aHash
    group.bench_function("aHash", |b| {
        let state: AHashRandomState = AHashRandomState::new();
        b.iter(|| {
            for i in 0u64..iterations {
                let mut h: AHasher = state.build_hasher();
                i.hash(&mut h);
                black_box(h.finish());
            }
        })
    });

    // Foldhash
    group.bench_function("Foldhash", |b| {
        let state: FoldRandomState = FoldRandomState::default();
        b.iter(|| {
            for i in 0u64..iterations {
                let mut h: FoldHasher = state.build_hasher();
                i.hash(&mut h);
                black_box(h.finish());
            }
        })
    });

    // NoHash
    group.bench_function("NoHash", |b| {
        let state: BuildNoHashHasher<u64> = BuildNoHashHasher::default();
        b.iter(|| {
            for i in 0u64..iterations {
                let mut h: NoHashHasher<u64> = state.build_hasher();
                i.hash(&mut h);
                black_box(h.finish());
            }
        })
    });

    group.finish();
}

// ============================================================================
// HASHMAP INSERTION BENCHMARKS
// ============================================================================
// Measures the full cost of inserting items into a HashMap,
// including hashing, bucket lookup, and memory allocation.

fn bench_hashmap_insert(c: &mut Criterion) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("HashMap_Insert");

    for size in [1_000, 10_000, 100_000] {
        group.throughput(Throughput::Elements(size as u64));

        // Generate test keys
        let string_keys: Vec<String> = (0..size).map(|i| format!("key_{:08}", i)).collect();
        let int_keys: Vec<u64> = (0..size as u64).collect();

        // === String keys ===

        // SipHash (default HashMap)
        group.bench_with_input(
            BenchmarkId::new("SipHash_String", size),
            &string_keys,
            |b, keys| {
                b.iter(|| {
                    let mut map: HashMap<String, i32> = HashMap::with_capacity(size);
                    for (i, key) in keys.iter().enumerate() {
                        map.insert(key.clone(), i as i32);
                    }
                    map
                })
            },
        );

        // FxHash
        group.bench_with_input(
            BenchmarkId::new("FxHash_String", size),
            &string_keys,
            |b, keys| {
                b.iter(|| {
                    let mut map: FxHashMap<String, i32> = FxHashMap::default();
                    map.reserve(size);
                    for (i, key) in keys.iter().enumerate() {
                        map.insert(key.clone(), i as i32);
                    }
                    map
                })
            },
        );

        // aHash
        group.bench_with_input(
            BenchmarkId::new("aHash_String", size),
            &string_keys,
            |b, keys| {
                b.iter(|| {
                    let mut map: AHashMap<String, i32> = AHashMap::with_capacity(size);
                    for (i, key) in keys.iter().enumerate() {
                        map.insert(key.clone(), i as i32);
                    }
                    map
                })
            },
        );

        // Foldhash
        group.bench_with_input(
            BenchmarkId::new("Foldhash_String", size),
            &string_keys,
            |b, keys| {
                b.iter(|| {
                    let mut map: FoldHashMap<String, i32> = FoldHashMap::with_capacity(size);
                    for (i, key) in keys.iter().enumerate() {
                        map.insert(key.clone(), i as i32);
                    }
                    map
                })
            },
        );

        // === Integer keys ===

        // SipHash
        group.bench_with_input(
            BenchmarkId::new("SipHash_Int", size),
            &int_keys,
            |b, keys| {
                b.iter(|| {
                    let mut map: HashMap<u64, i32> = HashMap::with_capacity(size);
                    for (i, &key) in keys.iter().enumerate() {
                        map.insert(key, i as i32);
                    }
                    map
                })
            },
        );

        // FxHash
        group.bench_with_input(
            BenchmarkId::new("FxHash_Int", size),
            &int_keys,
            |b, keys| {
                b.iter(|| {
                    let mut map: FxHashMap<u64, i32> = FxHashMap::default();
                    map.reserve(size);
                    for (i, &key) in keys.iter().enumerate() {
                        map.insert(key, i as i32);
                    }
                    map
                })
            },
        );

        // NoHash (integer keys only)
        group.bench_with_input(
            BenchmarkId::new("NoHash_Int", size),
            &int_keys,
            |b, keys| {
                b.iter(|| {
                    let mut map: IntMap<u64, i32> = IntMap::default();
                    map.reserve(size);
                    for (i, &key) in keys.iter().enumerate() {
                        map.insert(key, i as i32);
                    }
                    map
                })
            },
        );
    }

    group.finish();
}

// ============================================================================
// HASHMAP LOOKUP BENCHMARKS
// ============================================================================
// Measures lookup performance with pre-populated HashMaps.
// This isolates lookup cost from insertion/allocation.

fn bench_hashmap_lookup(c: &mut Criterion) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("HashMap_Lookup");

    for size in [1_000, 10_000, 100_000] {
        // Pre-generate keys
        let string_keys: Vec<String> = (0..size).map(|i| format!("key_{:08}", i)).collect();
        let int_keys: Vec<u64> = (0..size as u64).collect();

        // Pre-build all maps
        let sip_string: HashMap<String, i32> = string_keys
            .iter()
            .enumerate()
            .map(|(i, k)| (k.clone(), i as i32))
            .collect();
        let fx_string: FxHashMap<String, i32> = string_keys
            .iter()
            .enumerate()
            .map(|(i, k)| (k.clone(), i as i32))
            .collect();
        let ahash_string: AHashMap<String, i32> = string_keys
            .iter()
            .enumerate()
            .map(|(i, k)| (k.clone(), i as i32))
            .collect();
        let fold_string: FoldHashMap<String, i32> = string_keys
            .iter()
            .enumerate()
            .map(|(i, k)| (k.clone(), i as i32))
            .collect();

        let sip_int: HashMap<u64, i32> = int_keys
            .iter()
            .enumerate()
            .map(|(i, &k)| (k, i as i32))
            .collect();
        let fx_int: FxHashMap<u64, i32> = int_keys
            .iter()
            .enumerate()
            .map(|(i, &k)| (k, i as i32))
            .collect();
        let nohash_int: IntMap<u64, i32> = int_keys
            .iter()
            .enumerate()
            .map(|(i, &k)| (k, i as i32))
            .collect();

        // === String key lookups ===

        group.bench_with_input(
            BenchmarkId::new("SipHash_String", size),
            &string_keys,
            |b, keys| {
                b.iter(|| {
                    let mut sum: i32 = 0;
                    for key in keys {
                        if let Some(&v) = sip_string.get(key) {
                            sum += v;
                        }
                    }
                    black_box(sum)
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("FxHash_String", size),
            &string_keys,
            |b, keys| {
                b.iter(|| {
                    let mut sum: i32 = 0;
                    for key in keys {
                        if let Some(&v) = fx_string.get(key) {
                            sum += v;
                        }
                    }
                    black_box(sum)
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("aHash_String", size),
            &string_keys,
            |b, keys| {
                b.iter(|| {
                    let mut sum: i32 = 0;
                    for key in keys {
                        if let Some(&v) = ahash_string.get(key) {
                            sum += v;
                        }
                    }
                    black_box(sum)
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("Foldhash_String", size),
            &string_keys,
            |b, keys| {
                b.iter(|| {
                    let mut sum: i32 = 0;
                    for key in keys {
                        if let Some(&v) = fold_string.get(key) {
                            sum += v;
                        }
                    }
                    black_box(sum)
                })
            },
        );

        // === Integer key lookups ===

        group.bench_with_input(
            BenchmarkId::new("SipHash_Int", size),
            &int_keys,
            |b, keys| {
                b.iter(|| {
                    let mut sum: i32 = 0;
                    for &key in keys {
                        if let Some(&v) = sip_int.get(&key) {
                            sum += v;
                        }
                    }
                    black_box(sum)
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("FxHash_Int", size),
            &int_keys,
            |b, keys| {
                b.iter(|| {
                    let mut sum: i32 = 0;
                    for &key in keys {
                        if let Some(&v) = fx_int.get(&key) {
                            sum += v;
                        }
                    }
                    black_box(sum)
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("NoHash_Int", size),
            &int_keys,
            |b, keys| {
                b.iter(|| {
                    let mut sum: i32 = 0;
                    for &key in keys {
                        if let Some(&v) = nohash_int.get(&key) {
                            sum += v;
                        }
                    }
                    black_box(sum)
                })
            },
        );
    }

    group.finish();
}

// ============================================================================
// ENTRY API BENCHMARKS
// ============================================================================
// Measures the common pattern of "get or insert" using the Entry API.

fn bench_entry_api(c: &mut Criterion) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("Entry_API");

    // Simulate word counting - a common Entry API use case
    let words: Vec<&str> = vec![
        "the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog", "the", "fox", "is",
        "quick", "and", "the", "dog", "is", "lazy",
    ];
    let text: Vec<&str> = (0..10_000).map(|i| words[i % words.len()]).collect();

    // SipHash
    group.bench_function("SipHash", |b| {
        b.iter(|| {
            let mut counts: HashMap<&str, i32> = HashMap::new();
            for &word in &text {
                *counts.entry(word).or_insert(0) += 1;
            }
            counts
        })
    });

    // FxHash
    group.bench_function("FxHash", |b| {
        b.iter(|| {
            let mut counts: FxHashMap<&str, i32> = FxHashMap::default();
            for &word in &text {
                *counts.entry(word).or_insert(0) += 1;
            }
            counts
        })
    });

    // aHash
    group.bench_function("aHash", |b| {
        b.iter(|| {
            let mut counts: AHashMap<&str, i32> = AHashMap::new();
            for &word in &text {
                *counts.entry(word).or_insert(0) += 1;
            }
            counts
        })
    });

    // Foldhash
    group.bench_function("Foldhash", |b| {
        b.iter(|| {
            let mut counts: FoldHashMap<&str, i32> = FoldHashMap::new();
            for &word in &text {
                *counts.entry(word).or_insert(0) += 1;
            }
            counts
        })
    });

    group.finish();
}

// ============================================================================
// LARGE KEY BENCHMARKS
// ============================================================================
// Tests performance with large keys where xxHash should excel.

fn bench_large_keys(c: &mut Criterion) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("Large_Keys");

    // Create large string keys (simulating file paths, URLs, etc.)
    let large_keys: Vec<String> = (0..1_000)
        .map(|i| format!("/very/long/path/to/some/resource/item_{:08}/data.json", i))
        .collect();

    group.throughput(Throughput::Elements(large_keys.len() as u64));

    // SipHash
    group.bench_function("SipHash", |b| {
        b.iter(|| {
            let mut map: HashMap<String, i32> = HashMap::new();
            for (i, key) in large_keys.iter().enumerate() {
                map.insert(key.clone(), i as i32);
            }
            map
        })
    });

    // FxHash
    group.bench_function("FxHash", |b| {
        b.iter(|| {
            let mut map: FxHashMap<String, i32> = FxHashMap::default();
            for (i, key) in large_keys.iter().enumerate() {
                map.insert(key.clone(), i as i32);
            }
            map
        })
    });

    // aHash
    group.bench_function("aHash", |b| {
        b.iter(|| {
            let mut map: AHashMap<String, i32> = AHashMap::new();
            for (i, key) in large_keys.iter().enumerate() {
                map.insert(key.clone(), i as i32);
            }
            map
        })
    });

    // Foldhash
    group.bench_function("Foldhash", |b| {
        b.iter(|| {
            let mut map: FoldHashMap<String, i32> = FoldHashMap::new();
            for (i, key) in large_keys.iter().enumerate() {
                map.insert(key.clone(), i as i32);
            }
            map
        })
    });

    group.finish();
}

// ============================================================================
// CRITERION CONFIGURATION
// ============================================================================

criterion_group!(
    benches,
    bench_raw_hashing,
    bench_integer_hashing,
    bench_hashmap_insert,
    bench_hashmap_lookup,
    bench_entry_api,
    bench_large_keys,
);

criterion_main!(benches);