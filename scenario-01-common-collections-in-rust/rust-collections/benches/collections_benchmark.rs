// Benchmarks for Rust collections using Criterion.
//
// Criterion provides statistically rigorous benchmarking with:
//   - Warm-up runs to stabilize measurements
//   - Statistical analysis (mean, median, standard deviation)
//   - Outlier detection and handling
//   - Comparison between runs (regression detection)
//   - HTML reports with graphs
//
// To run these benchmarks:
//   cargo bench
//
// To run a specific benchmark group:
//   cargo bench -- Insertions
//   cargo bench -- Lookups
//
// Results are saved to target/criterion/ with HTML reports.
//
// ============================================================================
// BENCHMARKING PRACTICES
// ============================================================================
//
// 1. USE `black_box()` - Prevents the compiler from optimizing away code
//    that doesn't produce observable side effects. Without it, the compiler
//    might eliminate your entire benchmark!
//
// 2. WARM-UP MATTERS - Criterion automatically warms up before measuring,
//    which primes CPU caches and triggers any lazy initialization.
//
// 3. ISOLATE WHAT YOU MEASURE - If you want to measure lookup time, don't
//    include collection construction in the timing loop.
//
// 4. TEST MULTIPLE SIZES - O(1) vs O(log n) vs O(n) differences become
//    dramatic at larger sizes. Always test at multiple scales.
//
// 5. RUN MULTIPLE TIMES - System noise (other processes, CPU throttling)
//    affects results. Criterion handles this with statistical analysis.
//
// ============================================================================

use criterion::measurement::WallTime;
use criterion::{
    BenchmarkGroup, BenchmarkId, Criterion, Throughput, criterion_group, criterion_main,
};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
use std::hint::black_box;

// ============================================================================
// INSERTION BENCHMARKS
// ============================================================================
// Measures how fast we can add elements to each collection type.
// This includes both the operation itself and any reallocation overhead.

fn bench_insertions(c: &mut Criterion) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("Insertions");

    // Test at multiple sizes to see how performance scales
    for size in [1_00, 1_000, 10_000] {
        // Set throughput so Criterion reports elements/second
        group.throughput(Throughput::Elements(size as u64));

        // -----------------------------------------------------------------
        // Vec: The baseline - contiguous memory, cache-friendly
        // -----------------------------------------------------------------

        // Vec without pre-allocation - must reallocate as it grows
        // Capacity doubles each time: 0 → 4 → 8 → 16 → 32 → ...
        group.bench_with_input(BenchmarkId::new("Vec", size), &size, |b, &size| {
            b.iter(|| {
                let mut v: Vec<i32> = Vec::new();
                for i in 0..size {
                    v.push(black_box(i));
                }
                v
            })
        });

        // Vec with pre-allocation - single allocation upfront
        // This avoids all reallocation overhead
        group.bench_with_input(
            BenchmarkId::new("Vec::with_capacity", size),
            &size,
            |b, &size| {
                b.iter(|| {
                    let mut v: Vec<i32> = Vec::with_capacity(size as usize);
                    for i in 0..size {
                        v.push(black_box(i));
                    }
                    v
                })
            },
        );

        // -----------------------------------------------------------------
        // VecDeque: Ring buffer - O(1) at both ends
        // -----------------------------------------------------------------

        group.bench_with_input(
            BenchmarkId::new("VecDeque::push_back", size),
            &size,
            |b, &size| {
                b.iter(|| {
                    let mut d: VecDeque<i32> = VecDeque::new();
                    for i in 0..size {
                        d.push_back(black_box(i));
                    }
                    d
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("VecDeque::push_front", size),
            &size,
            |b, &size| {
                b.iter(|| {
                    let mut d: VecDeque<i32> = VecDeque::new();
                    for i in 0..size {
                        d.push_front(black_box(i));
                    }
                    d
                })
            },
        );

        // -----------------------------------------------------------------
        // LinkedList: Per-element allocation overhead
        // -----------------------------------------------------------------

        group.bench_with_input(
            BenchmarkId::new("LinkedList::push_back", size),
            &size,
            |b, &size| {
                b.iter(|| {
                    let mut l: LinkedList<i32> = LinkedList::new();
                    for i in 0..size {
                        l.push_back(black_box(i));
                    }
                    l
                })
            },
        );

        // -----------------------------------------------------------------
        // HashMap: Hashing overhead + potential rehashing
        // -----------------------------------------------------------------

        group.bench_with_input(BenchmarkId::new("HashMap", size), &size, |b, &size| {
            b.iter(|| {
                let mut m: HashMap<i32, i32> = HashMap::new();
                for i in 0..size {
                    m.insert(black_box(i), i);
                }
                m
            })
        });

        group.bench_with_input(
            BenchmarkId::new("HashMap::with_capacity", size),
            &size,
            |b, &size| {
                b.iter(|| {
                    let mut m: HashMap<i32, i32> = HashMap::with_capacity(size as usize);
                    for i in 0..size {
                        m.insert(black_box(i), i);
                    }
                    m
                })
            },
        );

        // -----------------------------------------------------------------
        // BTreeMap: Tree rebalancing overhead, O(log n) per insert
        // -----------------------------------------------------------------

        group.bench_with_input(BenchmarkId::new("BTreeMap", size), &size, |b, &size| {
            b.iter(|| {
                let mut m: BTreeMap<i32, i32> = BTreeMap::new();
                for i in 0..size {
                    m.insert(black_box(i), i);
                }
                m
            })
        });

        // -----------------------------------------------------------------
        // HashSet: Same as HashMap without values
        // -----------------------------------------------------------------

        group.bench_with_input(BenchmarkId::new("HashSet", size), &size, |b, &size| {
            b.iter(|| {
                let mut s: HashSet<i32> = HashSet::new();
                for i in 0..size {
                    s.insert(black_box(i));
                }
                s
            })
        });

        // -----------------------------------------------------------------
        // BTreeSet: Same as BTreeMap without values
        // -----------------------------------------------------------------

        group.bench_with_input(BenchmarkId::new("BTreeSet", size), &size, |b, &size| {
            b.iter(|| {
                let mut s: BTreeSet<i32> = BTreeSet::new();
                for i in 0..size {
                    s.insert(black_box(i));
                }
                s
            })
        });

        // -----------------------------------------------------------------
        // BinaryHeap: O(log n) per push to maintain heap property
        // -----------------------------------------------------------------

        group.bench_with_input(BenchmarkId::new("BinaryHeap", size), &size, |b, &size| {
            b.iter(|| {
                let mut h: BinaryHeap<i32> = BinaryHeap::new();
                for i in 0..size {
                    h.push(black_box(i));
                }
                h
            })
        });
    }

    group.finish();
}

// ============================================================================
// LOOKUP BENCHMARKS
// ============================================================================
// Measures how fast we can find elements in each collection type.
// Collections are pre-built outside the timing loop to isolate lookup cost.

fn bench_lookups(c: &mut Criterion) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("Lookups");

    for size in [1_00, 1_000, 10_000, 100_000] {
        // Pre-build all collections OUTSIDE the benchmark loop
        // This ensures we're only measuring lookup time, not construction
        let vec: Vec<i32> = (0..size).collect();
        let hashset: HashSet<i32> = (0..size).collect();
        let btreeset: BTreeSet<i32> = (0..size).collect();
        let hashmap: HashMap<i32, i32> = (0..size).map(|i| (i, i * 2)).collect();
        let btreemap: BTreeMap<i32, i32> = (0..size).map(|i| (i, i * 2)).collect();

        // Target is the last element - worst case for linear search
        // This highlights the difference between O(n) and O(1)
        let target: i32 = size - 1;

        // -----------------------------------------------------------------
        // Vec: Linear search O(n) - must scan every element
        // -----------------------------------------------------------------

        group.bench_with_input(BenchmarkId::new("Vec::contains", size), &size, |b, _| {
            b.iter(|| vec.contains(black_box(&target)))
        });

        // Vec: Binary search O(log n) - requires sorted data
        group.bench_with_input(
            BenchmarkId::new("Vec::binary_search", size),
            &size,
            |b, _| b.iter(|| vec.binary_search(black_box(&target))),
        );

        // -----------------------------------------------------------------
        // HashSet/HashMap: O(1) average - hash and lookup bucket
        // -----------------------------------------------------------------

        group.bench_with_input(
            BenchmarkId::new("HashSet::contains", size),
            &size,
            |b, _| b.iter(|| hashset.contains(black_box(&target))),
        );

        group.bench_with_input(BenchmarkId::new("HashMap::get", size), &size, |b, _| {
            b.iter(|| hashmap.get(black_box(&target)))
        });

        // -----------------------------------------------------------------
        // BTreeSet/BTreeMap: O(log n) - tree traversal
        // -----------------------------------------------------------------

        group.bench_with_input(
            BenchmarkId::new("BTreeSet::contains", size),
            &size,
            |b, _| b.iter(|| btreeset.contains(black_box(&target))),
        );

        group.bench_with_input(BenchmarkId::new("BTreeMap::get", size), &size, |b, _| {
            b.iter(|| btreemap.get(black_box(&target)))
        });
    }

    group.finish();
}

// ============================================================================
// FRONT OPERATIONS BENCHMARKS
// ============================================================================
// Demonstrates the dramatic difference between Vec and VecDeque for front ops.
// Vec::insert(0, x) is O(n) per operation, VecDeque::push_front is O(1).

fn bench_front_operations(c: &mut Criterion) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("Front_Operations");

    // Vec insert at front - O(n) per insert = O(n²) total
    // We use smaller sizes because this is VERY slow
    for size in [1_00, 5_00, 1_000] {
        group.bench_with_input(
            BenchmarkId::new("Vec::insert(0,x)", size),
            &size,
            |b, &size| {
                b.iter(|| {
                    let mut v: Vec<i32> = Vec::new();
                    for i in 0..size {
                        v.insert(0, black_box(i)); // Shifts ALL elements!
                    }
                    v
                })
            },
        );
    }

    // VecDeque push_front - O(1) per insert
    // Can use much larger sizes because it's fast
    for size in [1_00, 1_000, 10_000, 100_000] {
        group.bench_with_input(
            BenchmarkId::new("VecDeque::push_front", size),
            &size,
            |b, &size| {
                b.iter(|| {
                    let mut d: VecDeque<i32> = VecDeque::new();
                    for i in 0..size {
                        d.push_front(black_box(i));
                    }
                    d
                })
            },
        );
    }

    group.finish();
}

// ============================================================================
// ITERATION BENCHMARKS
// ============================================================================
// Demonstrates how cache locality affects iteration performance.
// Vec is fastest because elements are contiguous in memory.

fn bench_iteration(c: &mut Criterion) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("Iteration");

    let size: i32 = 100_000;

    // Pre-build collections
    let vec: Vec<i32> = (0..size).collect();
    let vecdeque: VecDeque<i32> = (0..size).collect();
    let linkedlist: LinkedList<i32> = (0..size).collect();
    let hashset: HashSet<i32> = (0..size).collect();
    let btreeset: BTreeSet<i32> = (0..size).collect();
    let binaryheap: BinaryHeap<i32> = (0..size).collect();

    group.throughput(Throughput::Elements(size as u64));

    // Vec - contiguous memory, excellent cache locality
    group.bench_function("Vec", |b| {
        b.iter(|| {
            let sum: i32 = vec.iter().sum();
            black_box(sum)
        })
    });

    // VecDeque - also contiguous (ring buffer)
    group.bench_function("VecDeque", |b| {
        b.iter(|| {
            let sum: i32 = vecdeque.iter().sum();
            black_box(sum)
        })
    });

    // LinkedList - scattered memory, poor cache locality
    group.bench_function("LinkedList", |b| {
        b.iter(|| {
            let sum: i32 = linkedlist.iter().sum();
            black_box(sum)
        })
    });

    // HashSet - bucket-based storage
    group.bench_function("HashSet", |b| {
        b.iter(|| {
            let sum: i32 = hashset.iter().sum();
            black_box(sum)
        })
    });

    // BTreeSet - tree nodes, decent locality within nodes
    group.bench_function("BTreeSet", |b| {
        b.iter(|| {
            let sum: i32 = btreeset.iter().sum();
            black_box(sum)
        })
    });

    // BinaryHeap - Vec-backed, but iter() is NOT sorted!
    group.bench_function("BinaryHeap::iter", |b| {
        b.iter(|| {
            let sum: i32 = binaryheap.iter().sum();
            black_box(sum)
        })
    });

    group.finish();
}

// ============================================================================
// RANGE QUERY BENCHMARKS
// ============================================================================
// Demonstrates BTreeMap/BTreeSet's range query advantage.
// HashMap has no efficient range query - must scan everything.

fn bench_range_queries(c: &mut Criterion) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("Range_Queries");

    let size: i32 = 10_000i32;

    // Pre-build collections
    let hashmap: HashMap<i32, i32> = (0..size).map(|i| (i, i * 2)).collect();
    let btreemap: BTreeMap<i32, i32> = (0..size).map(|i| (i, i * 2)).collect();
    let hashset: HashSet<i32> = (0..size).collect();
    let btreeset: BTreeSet<i32> = (0..size).collect();

    let range_start: i32 = size / 4; // 2500
    let range_end: i32 = 3 * size / 4; // 7500

    // HashMap: Must filter all entries - O(n)
    group.bench_function("HashMap_filter_range", |b| {
        b.iter(|| {
            hashmap
                .iter()
                .filter(|&(k, _)| *k >= range_start && *k <= range_end)
                .count()
        })
    });

    // BTreeMap: Native range query - O(log n + k)
    group.bench_function("BTreeMap::range", |b| {
        b.iter(|| btreemap.range(range_start..=range_end).count())
    });

    // HashSet: Must filter all entries - O(n)
    group.bench_function("HashSet_filter_range", |b| {
        b.iter(|| {
            hashset
                .iter()
                .filter(|&x| *x >= range_start && *x <= range_end)
                .count()
        })
    });

    // BTreeSet: Native range query - O(log n + k)
    group.bench_function("BTreeSet::range", |b| {
        b.iter(|| btreeset.range(range_start..=range_end).count())
    });

    group.finish();
}

// ============================================================================
// PRIORITY QUEUE BENCHMARKS
// ============================================================================
// Compares different approaches to priority-based processing.

fn bench_priority_operations(c: &mut Criterion) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("Priority_Operations");

    let size: i32 = 10_000i32;

    // BinaryHeap: Purpose-built for priority queue operations
    group.bench_function("BinaryHeap_push_pop", |b| {
        b.iter(|| {
            let mut heap: BinaryHeap<i32> = BinaryHeap::new();
            for i in 0..size {
                heap.push(black_box(i));
            }
            let mut sum: i32 = 0i32;
            while let Some(max) = heap.pop() {
                sum = sum.wrapping_add(max);
            }
            sum
        })
    });

    // Vec + sort: Batch approach
    group.bench_function("Vec_sort_iterate", |b| {
        b.iter(|| {
            let mut v: Vec<i32> = (0..size).collect();
            v.sort_by(|a, b| b.cmp(a)); // Descending
            let sum: i32 = v.iter().sum();
            black_box(sum)
        })
    });

    // BTreeSet: Always sorted, but no duplicates
    group.bench_function("BTreeSet_insert_iterate_rev", |b| {
        b.iter(|| {
            let set: BTreeSet<i32> = (0..size).collect();
            let sum: i32 = set.iter().rev().sum();
            black_box(sum)
        })
    });

    // BinaryHeap with Reverse for min-heap
    group.bench_function("BinaryHeap_min_heap", |b| {
        b.iter(|| {
            let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
            for i in 0..size {
                heap.push(Reverse(black_box(i)));
            }
            let mut sum: i32 = 0i32;
            while let Some(Reverse(min)) = heap.pop() {
                sum = sum.wrapping_add(min);
            }
            sum
        })
    });

    group.finish();
}

// ============================================================================
// ENTRY API BENCHMARKS
// ============================================================================
// Demonstrates why the Entry API is more efficient than contains_key + insert.

fn bench_entry_api(c: &mut Criterion) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("Entry_API");

    // Generate sample text for word counting
    let words: Vec<&str> = vec![
        "the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog", "the", "fox", "is",
        "quick", "and", "the", "dog", "is", "lazy",
    ];
    let text: Vec<&str> = (0..10_000).map(|i| words[i % words.len()]).collect();

    // GOOD: Entry API - single lookup per word
    group.bench_function("HashMap_entry_api", |b| {
        b.iter(|| {
            let mut counts: HashMap<&str, i32> = HashMap::new();
            for word in &text {
                *counts.entry(*word).or_insert(0) += 1;
            }
            counts
        })
    });

    // BAD: contains_key + get_mut - two lookups per word
    group.bench_function("HashMap_contains_key", |b| {
        b.iter(|| {
            let mut counts: HashMap<&str, i32> = HashMap::new();
            for word in &text {
                if counts.contains_key(word) {
                    *counts.get_mut(word).unwrap() += 1;
                } else {
                    counts.insert(*word, 1);
                }
            }
            counts
        })
    });

    // BTreeMap Entry API for comparison
    group.bench_function("BTreeMap_entry_api", |b| {
        b.iter(|| {
            let mut counts: BTreeMap<&str, i32> = BTreeMap::new();
            for word in &text {
                *counts.entry(*word).or_insert(0) += 1;
            }
            counts
        })
    });

    group.finish();
}

// ============================================================================
// REMOVAL BENCHMARKS
// ============================================================================
// Measures the cost of removing elements from different collections.

fn bench_removals(c: &mut Criterion) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("Removals");

    let size: i32 = 1_000i32;

    // Vec: Remove from end (O(1)) vs remove from front (O(n))
    group.bench_function("Vec_pop_back", |b| {
        b.iter_batched(
            || (0..size).collect::<Vec<i32>>(),
            |mut v: Vec<i32>| {
                while v.pop().is_some() {}
                v
            },
            criterion::BatchSize::SmallInput,
        )
    });

    group.bench_function("Vec_remove_front", |b| {
        b.iter_batched(
            || (0..size).collect::<Vec<i32>>(),
            |mut v: Vec<i32>| {
                while !v.is_empty() {
                    v.remove(0);
                }
                v
            },
            criterion::BatchSize::SmallInput,
        )
    });

    // VecDeque: O(1) from both ends
    group.bench_function("VecDeque_pop_front", |b| {
        b.iter_batched(
            || (0..size).collect::<VecDeque<i32>>(),
            |mut d: VecDeque<i32>| {
                while d.pop_front().is_some() {}
                d
            },
            criterion::BatchSize::SmallInput,
        )
    });

    // HashMap: O(1) average removal
    group.bench_function("HashMap_remove", |b| {
        b.iter_batched(
            || (0..size).map(|i| (i, i)).collect::<HashMap<i32, i32>>(),
            |mut m: HashMap<i32, i32>| {
                for i in 0..size {
                    m.remove(&i);
                }
                m
            },
            criterion::BatchSize::SmallInput,
        )
    });

    // BTreeMap: O(log n) removal
    group.bench_function("BTreeMap_remove", |b| {
        b.iter_batched(
            || (0..size).map(|i| (i, i)).collect::<BTreeMap<i32, i32>>(),
            |mut m: BTreeMap<i32, i32>| {
                for i in 0..size {
                    m.remove(&i);
                }
                m
            },
            criterion::BatchSize::SmallInput,
        )
    });

    // BinaryHeap: O(log n) pop
    group.bench_function("BinaryHeap_pop_all", |b| {
        b.iter_batched(
            || (0..size).collect::<BinaryHeap<i32>>(),
            |mut h: BinaryHeap<i32>| {
                while h.pop().is_some() {}
                h
            },
            criterion::BatchSize::SmallInput,
        )
    });

    group.finish();
}

// ============================================================================
// SCALING BENCHMARKS
// ============================================================================
// Shows how complexity classes differ as size increases.

fn bench_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("Scaling");

    // Test lookup scaling at different sizes
    for size in [1_00, 1_000, 10_000, 100_000, 1_000_000] {
        let vec: Vec<i32> = (0..size).collect();
        let hashset: HashSet<i32> = (0..size).collect();
        let btreeset: BTreeSet<i32> = (0..size).collect();
        let target: i32 = size - 1; // Worst case for linear search

        // O(n) - linear search
        group.bench_with_input(BenchmarkId::new("Vec_linear", size), &size, |b, _| {
            b.iter(|| vec.contains(black_box(&target)))
        });

        // O(1) - hash lookup
        group.bench_with_input(BenchmarkId::new("HashSet_O1", size), &size, |b, _| {
            b.iter(|| hashset.contains(black_box(&target)))
        });

        // O(log n) - tree lookup
        group.bench_with_input(BenchmarkId::new("BTreeSet_logn", size), &size, |b, _| {
            b.iter(|| btreeset.contains(black_box(&target)))
        });
    }

    group.finish();
}

// ============================================================================
// CRITERION CONFIGURATION
// ============================================================================

criterion_group!(
    benches,
    bench_insertions,
    bench_lookups,
    bench_front_operations,
    bench_iteration,
    bench_range_queries,
    bench_priority_operations,
    bench_entry_api,
    bench_removals,
    bench_scaling,
);

criterion_main!(benches);
