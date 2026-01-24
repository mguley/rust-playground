# Hashing Algorithms for HashMap in Rust

## Table of Contents
- [Introduction](#introduction)
- [What is a hash function?](#what-is-a-hash-function)
- [Why does the hasher matter?](#why-does-the-hasher-matter)
- [Prerequisites](#prerequisites)
- [Step 1: Setting up our environment](#step-1-setting-up-our-environment)
- [Step 2: SipHash - The Default Hasher](#step-2-siphash---the-default-hasher)
- [Step 3: FxHash - The Compiler's Choice](#step-3-fxhash---the-compilers-choice)
- [Step 4: aHash - Speed Meets Security](#step-4-ahash---speed-meets-security)
- [Step 5: Foldhash - The Modern Contender](#step-5-foldhash---the-modern-contender)
- [Step 6: xxHash - The Established Performer](#step-6-xxhash---the-established-performer)
- [Step 7: NoHash - When Hashing is Unnecessary](#step-7-nohash---when-hashing-is-unnecessary)
- TODO

---

#### Introduction

When you create a `HashMap` in Rust, you rarely think about *how* it finds and stores your data. Behind the scenes,
every key you insert gets transformed by a hash function - an algorithm that converts your key into a number that
determines where the value lives in memory.

Rust's standard library ships with a hash function called `SipHash 1-3`, chosen specifically for its resistance to
certain types of attacks. But this security comes at a cost: `SipHash` isn't the fastest hasher available. For many
applications - especially those processing trusted data - this trade-off may not be ideal.

Consider these scenarios:

- A compiler processing millions of symbol lookups needs the fastest possible hash function. The input comes from
  source files it controls, so security against malicious input is less critical than raw speed.
- A web server handling user-provided query parameters must defend against denial-of-service attacks where adversaries
  craft inputs designed to trigger worst case hash table performance.
- A game engine tracking entity IDs (simple integers) might not need a hash function at all - the IDs themselves could
  serve directly as hash values.

In this deep dive, we'll explore the hashing algorithms available in the Rust ecosystem, understand their design
trade-offs, and learn when to choose each one. By the end, you'll be equipped to make informed decisions that can
significantly impact your application's performance.

---

#### What is a hash function?

A hash function transforms input data of arbitrary size into a fixed-size output, typically called a hash value,
hash code, or digest. For HashMap usage, we need hash functions with specific properties.

**Determinism**: The same input must always produce the same output. If `hash("hello")` returns `42` once, it must
return `42` every time. Without this property, you'd never find your data again after inserting it.

**Uniform distribution**: Hash values should spread evenly across the output space. If all your keys hash to similar
values, they'll cluster in the same buckets, degrading HashMap's O(1) lookup to O(n).

**Speed**: Since every HashMap operation requires computing a hash, the function's speed directly impacts performance.
A hasher that takes 10x longer makes your HashMap 10x slower for small values.

**Avalanche effect**: Small changes in input should produce dramatically different outputs. If "hello" and "hallo"
hash to similar values, they might collide more often than random chance would suggest.

Here's how a hash function fits into HashMap's operation:

```
    insert("apple", 100)
           │
           ▼
    ┌──────────────┐
    │ hash("apple")│ = 7823491...
    └──────────────┘
           │
           ▼
    bucket_index = hash % num_buckets = 3
           │
           ▼
    ┌───────────────────────────────────────────┐
    │ Buckets:                                  │
    │ [0]: empty                                │
    │ [1]: ("grape", 50)                        │
    │ [2]: empty                                │
    │ [3]: ("apple", 100) ← stored here         │
    │ [4]: ("banana", 75)                       │
    │ ...                                       │
    └───────────────────────────────────────────┘
```

When you later call `get("apple")`, the HashMap computes the same hash, finds bucket 3, and returns your value.
The quality of this hash function determines whether lookups stay O(1) or degrade due to collisions.

---

#### Why does the hasher matter?

The choice of hash function affects three critical aspects of HashMap performance: speed, security, and memory access patterns.

**Speed variance is dramatic**. Different hashers can vary by 5-10x in throughput. For a HashMap with millions of
operations, this difference translates directly to application performance. The table below shows approximate relative
speeds (higher is faster) for hashing small keys:

```
╔═══════════════╦════════════════════╦═══════════════════════╗
║    Hasher     ║  Relative Speed    ║       Best For        ║
╠═══════════════╬════════════════════╬═══════════════════════╣
║  NoHash       ║  ~10x baseline     ║  Integer keys only    ║
║  FxHash       ║  ~5-8x baseline    ║  Compilers, trusted   ║
║  aHash        ║  ~4-7x baseline    ║  General purpose      ║
║  Foldhash     ║  ~4-6x baseline    ║  Modern alternative   ║
║  xxHash       ║  ~3-5x baseline    ║  Large data, files    ║
║  SipHash      ║  1x (baseline)     ║  Security-critical    ║
╚═══════════════╩════════════════════╩═══════════════════════╝
```

**Security matters for untrusted input**. A cleverly crafted set of keys can cause hash collisions that turn your O(1)
HashMap into an O(n) linked list. This is called a HashDoS attack, and it's why Rust defaults to SipHash.

**Memory access patterns affect performance**. Some hashers produce better distribution, leading to fewer
collisions and more cache friendly access patterns. Others may be theoretically fast but cause clustering that hurts
practical performance.

---

#### Prerequisites

Before we begin, you'll need:

- Rust installed (version 1.85+, we tested with 1.92)
- Basic knowledge of Rust syntax (variables, functions, basic types)
- Familiarity with `HashMap` from Scenario 1 (or equivalent experience)
- A code editor of your choice
- Terminal/command-line access

If you haven't worked through Scenario 1 (Common Collections in Rust), we recommend doing so first, as this scenario
builds on that foundation.

---

#### Step 1: Setting up our environment

Let's create a new Cargo project for our hashing experiments:

```bash
mkdir -p scenario-02-hashing-algorithms-for-hashmap
cd scenario-02-hashing-algorithms-for-hashmap
cargo init --name hashing_demo
```

Now let's set up our `Cargo.toml` with all the hashers we'll explore:

```toml
[package]
name = "hashing_demo"
version = "0.1.0"
edition = "2024"

[dependencies]
# The default hasher info (for educational comparison)
rustc_version_runtime = "0.3"

# Alternative hashers we'll explore
rustc-hash = "2.1.1"     # FxHash - used in rustc compiler
ahash = "0.8.12"          # aHash - fast with DOS resistance
foldhash = "0.2.0"       # Foldhash - modern, quality-focused
twox-hash = "2.1.2"      # xxHash - established high-speed hasher
xxhash-rust = { version = "0.8.15", features = ["xxh3"] }    # Alternative xxHash implementation
nohash-hasher = "0.2.0"  # NoHash - for integer keys

# For generating random test data
rand = "0.9.2"

[dev-dependencies]
criterion = "0.8.1"

```

Create our initial `src/main.rs`:

```rust
// src/main.rs
use rustc_version_runtime;

fn main() {
    println!("Hashing Algorithms for HashMap - Demo");
    println!("Compiled with: {:?}", rustc_version_runtime::version());
    println!();

    // We'll add our example function calls here
}
```

Verify everything compiles:

```bash
cargo build
cargo run
```

---

#### Step 2: SipHash - the default hasher

Rust's standard library uses SipHash 1-3 as the default hasher for `HashMap` and `HashSet`. Understanding why requires
a brief journey into hash table security.

**The history**: In 2011, security researchers demonstrated that many web frameworks were vulnerable to "hash flooding"
attacks. By sending carefully crafted HTTP parameters, attackers could force worst-case hash table performance, causing
servers to spend minutes processing a single request. This affected Python, Ruby, PHP, Java, and many other languages.

**The solution**: SipHash was designed specifically to resist these attacks. It's a cryptographically inspired hash
function with a key feature: even if an attacker knows the algorithm, they can't predict hash values without knowing
the random seed (which Rust generates fresh for each HashMap).

**The trade-off**: SipHash is slower than simpler hash functions. For most applications this doesn't matter - modern
CPUs hash data faster than you can read it from disk or network. But for hot loops processing millions of lookups,
the difference becomes measurable.

Create `src/siphash_examples.rs`:

```rust
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
```

Update `src/main.rs`:

```rust
// src/main.rs
mod siphash_examples;

use siphash_examples::run_all as siphash_run_all;

//..

fn main() {
  println!("Hashing Algorithms for HashMap - Demo");
  println!("Compiled with: {:?}", rustc_version_runtime::version());
  println!();

  siphash_run_all();
}
```

Run the examples:

```bash
cargo run
```

#### Understanding SipHash internals

SipHash paper: https://cr.yp.to/siphash/siphash-20120918.pdf

SipHash processes data in 64-bit blocks through a series of mixing operations. Here's a simplified view of its structure:

```
                Input: "hello"
                     │
                     ▼
    ┌─────────────────────────────────┐
    │     Pad to 64-bit boundary      │
    │   + append length byte          │
    └─────────────────────────────────┘
                     │
                     ▼
    ┌─────────────────────────────────┐
    │    Initialize state with        │
    │    128-bit random key (k0, k1)  │
    │    v0 = k0 ^ 0x736f6d65...      │
    │    v1 = k1 ^ 0x646f7261...      │
    │    v2 = k0 ^ 0x6c796765...      │
    │    v3 = k1 ^ 0x74656462...      │
    └─────────────────────────────────┘
                     │
        ┌────────────┴────────────┐
        │   For each 64-bit block │
        │   ┌───────────────────┐ │
        │   │ v3 ^= block       │ │
        │   │ SipRound × 1      │ │ ← "c" rounds (1 in SipHash-1-3)
        │   │ v0 ^= block       │ │
        │   └───────────────────┘ │
        └────────────┬────────────┘
                     │
    ┌─────────────────────────────────┐
    │         Finalization            │
    │    v2 ^= 0xff                   │
    │    SipRound × 3                 │ ← "d" rounds (3 in SipHash-1-3)
    │    return v0 ^ v1 ^ v2 ^ v3     │
    └─────────────────────────────────┘
                     │
                     ▼
              64-bit hash value
```

The random key (generated once per HashMap creation) is what makes SipHash resistant to HashDoS attacks. Without knowing
the key, attackers cannot predict which inputs will collide.

#### Key takeaways for SipHash

| Property | Value |
|----------|-------|
| Algorithm | Currently SipHash 1-3 (subject to change) |
| Output size | 64 bits |
| Security | HashDoS resistant |
| Speed | Throughput ranges from hundreds of MB/s to a few GB/s depending on CPU, implementation, and input size; |
| Key requirement | 128-bit random seed |
| Best for | Untrusted input, default use |

**Summary**: SipHash is Rust's safe default. It protects against hash-flooding attacks at the cost of some speed. For
most applications, this trade-off is correct - security by default is the right choice. Only switch to a faster hasher
when you've profiled your code and identified HashMap operations as a bottleneck, or when you know your input is trusted.

---

#### Step 3: FxHash - the compiler's choice

FxHash (from the `rustc-hash` crate) is the hash function used internally by the Rust compiler. It's named after
Firefox, where it was originally developed for SpiderMonkey's JavaScript engine.

**The key insight**: When processing source code, a compiler has complete control over its input. Adversaries can't
inject malicious identifiers designed to cause hash collisions. This means the compiler can use a faster hash function
without security concerns.

**The trade-off**: FxHash is blazingly fast (especially for small keys) but offers no protection against HashDoS attacks.
Never use it for untrusted input.

Create `src/fxhash_examples.rs`:

```rust
//! FxHash Examples - The Compiler's Choice
//!
//! FxHash is a non-cryptographic hash function optimized for speed.
//! Originally developed for Firefox's SpiderMonkey JavaScript engine,
//! it's now used in the Rust compiler (rustc) for symbol tables.
//!
//! Key properties:
//! - Extremely fast for small keys (integers, short strings)
//! - Simple algorithm: multiply-xor-rotate
//! - NO cryptographic security
//! - NOT resistant to HashDoS attacks
//! - Deterministic: same input always produces same hash (no random seed)
//!
//! The "Fx" prefix comes from "Firefox" where it was first used.
//!
//! IMPORTANT: Only use FxHash when you control/trust the input!

use rustc_hash::{FxHashMap, FxHashSet, FxHasher};
use std::collections::HashMap;
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, BuildHasherDefault, DefaultHasher, Hash, Hasher};
use std::rc::Rc;
use std::time::{Duration, Instant};

/// Type aliases for clarity.
/// FxHashMap is just HashMap with FxHasher as the hasher.
pub type FxMap<K, V> = HashMap<K, V, BuildHasherDefault<FxHasher>>;

fn section(name: &str, what: &str, f: impl FnOnce()) {
  println!("\n{:=<80}", "");
  println!("DEMO: {name}");
  println!("  {what}");
  println!("{:=<80}", "");

  f();
}

pub fn run_all() {
  section(
    "basic_fxhashmap_usage",
    "Basic FxHashMap API usage (drop-in replacement for std::HashMap)",
    basic_fxhashmap_usage,
  );

  section(
    "fxhashset_usage",
    "FxHashSet usage and set operations",
    fxhashset_usage,
  );

  section(
    "deterministic_hashing",
    "FxHash is deterministic (same input -> same hash across runs)",
    deterministic_hashing,
  );

  section(
    "examining_fxhash_output",
    "Inspect FxHasher output (hex) for strings and integers",
    examining_fxhash_output,
  );

  section(
    "performance_comparison",
    "Rough timing: FxHash vs SipHash (not a real benchmark)",
    performance_comparison,
  );

  section(
    "compiler_symbol_table",
    "Practical demo: symbol table lookups (compiler-like workload)",
    compiler_symbol_table,
  );

  section(
    "string_interning",
    "Practical demo: string interning with FxHashSet<Rc<str>>",
    string_interning,
  );
}

/// Demonstrates basic FxHashMap usage.
///
/// The API is identical to standard HashMap - only the hasher differs.
/// This makes it easy to swap hashers without changing your code logic.
pub fn basic_fxhashmap_usage() {
  println!("\n  Basic FxHashMap Usage:");

  // Method 1: Using the provided type alias from rustc-hash crate.
  // This is the most common and convenient way.
  let mut scores: FxHashMap<String, i8> = FxHashMap::default();

  scores.insert("Alice".to_string(), 100);
  scores.insert("Bob".to_string(), 85);
  scores.insert("Charlie".to_string(), 92);

  println!("    FxHashMap: {:?}", scores);

  // Method 2: Using explicit type parameters.
  // This shows what's actually happening under the hood.
  // BuildHasherDefault<FxHasher> creates FxHasher instances.
  let mut explicit: HashMap<String, i8, BuildHasherDefault<FxHasher>> = HashMap::default();
  explicit.insert("test".to_string(), 42);
  println!("    Explicit type: {:?}", explicit);

  // Method 3: Converting from standard HashMap.
  // You can collect any iterator into an FxHashMap.
  let std_map: HashMap<&str, i8> = HashMap::from([("a", 1), ("b", 2)]);
  let fx_map: FxHashMap<&str, i8> = std_map.into_iter().collect();
  println!("    Converted from std HashMap: {:?}", fx_map);

  // All standard HashMap methods work identically
  if let Some(score) = scores.get("Alice") {
    println!("    Get 'Alice': {}", score);
  }
}

/// Demonstrates FxHashSet usage.
///
/// FxHashSet provides the same performance benefits as FxHashMap
/// for set operations (membership testing, deduplication).
pub fn fxhashset_usage() {
  println!("\n  FxHashSet Usage:");

  let mut visited: FxHashSet<i8> = FxHashSet::default();

  visited.insert(1);
  visited.insert(2);
  visited.insert(3);
  visited.insert(2); // Duplicate, will be ignored

  println!("    FxHashSet: {:?}", visited);
  println!("    Contains 2? {}", visited.contains(&2));
  println!("    Contains 5? {}", visited.contains(&5));

  // Set operations work as expected
  let mut other: FxHashSet<i8> = FxHashSet::default();
  other.insert(2);
  other.insert(3);
  other.insert(4);

  // Intersection, union, difference all work
  let intersection: FxHashSet<_> = visited.intersection(&other).cloned().collect();
  println!("    Intersection with {{2,3,4}}: {:?}", intersection);
}

/// Demonstrates FxHash's deterministic behavior.
///
/// Unlike SipHash, FxHash produces the same hash for the same input
/// across different HashMap instances and even different program runs.
/// This is both a feature (reproducibility) and a vulnerability (predictable).
pub fn deterministic_hashing() {
  println!("\n  FxHash is Deterministic:");

  // Create two separate BuildHasherDefault instances
  let hasher1: BuildHasherDefault<FxHasher> = BuildHasherDefault::<FxHasher>::default();
  let hasher2: BuildHasherDefault<FxHasher> = BuildHasherDefault::<FxHasher>::default();

  let value: &str = "test_value";

  // Hash with first hasher instance
  let mut h1: FxHasher = hasher1.build_hasher();
  value.hash(&mut h1);
  let hash1: u64 = h1.finish();

  // Hash with second hasher instance
  let mut h2: FxHasher = hasher2.build_hasher();
  value.hash(&mut h2);
  let hash2: u64 = h2.finish();

  println!("    Hash from instance 1: {}", hash1);
  println!("    Hash from instance 2: {}", hash2);
  println!("    Are they equal? {}", hash1 == hash2);
}

/// Examines the actual hash values FxHash produces.
///
/// Looking at the hash output helps understand the algorithm's behavior
/// and verify it has good distribution properties.
pub fn examining_fxhash_output() {
  println!("\n  Examining FxHash Output:");

  let build_hasher: BuildHasherDefault<FxHasher> = BuildHasherDefault::<FxHasher>::default();

  // Hash various strings
  println!("    String hashes:");
  for value in ["a", "b", "ab", "ba", "hello", "world"] {
    let mut hasher: FxHasher = build_hasher.build_hasher();
    value.hash(&mut hasher);
    // Display in hex to see bit patterns more clearly
    println!("      hash({:?}) = {:016x}", value, hasher.finish());
  }

  // Hash various integers
  println!("\n    Integer hashes:");
  for value in [0i32, 1, 42, 100, 1_000, -1] {
    let mut hasher: FxHasher = build_hasher.build_hasher();
    value.hash(&mut hasher);
    println!("      hash({:5}) = {:016x}", value, hasher.finish());
  }
}

/// Demonstrates FxHash performance compared to SipHash.
///
/// This comparison shows why FxHash is preferred for performance-critical
/// applications where security isn't a concern.
pub fn performance_comparison() {
  println!("\n  FxHash vs SipHash Performance:");

  let iterations: i32 = 500_000;

  // Build hashers for both types
  let fx_build: BuildHasherDefault<FxHasher> = BuildHasherDefault::<FxHasher>::default();
  let sip_build: RandomState = RandomState::new();

  // === Test with integer keys (FxHash excels here) ===

  // FxHash timing for integers
  let start: Instant = Instant::now();
  for i in 0..iterations {
    let mut hasher: FxHasher = fx_build.build_hasher();
    i.hash(&mut hasher);
    let _ = std::hint::black_box(hasher.finish());
  }
  let fx_int_time: Duration = start.elapsed();

  // SipHash timing for integers
  let start: Instant = Instant::now();
  for i in 0..iterations {
    let mut hasher: DefaultHasher = sip_build.build_hasher();
    i.hash(&mut hasher);
    let _ = std::hint::black_box(hasher.finish());
  }
  let sip_int_time: Duration = start.elapsed();

  println!("    Integer keys ({} iterations):", iterations);
  println!("      FxHash:  {:?}", fx_int_time);
  println!("      SipHash: {:?}", sip_int_time);
  println!(
    "      Speedup: {:.2}x faster",
    sip_int_time.as_nanos() as f64 / fx_int_time.as_nanos() as f64
  );

  // === Test with string keys ===
  let test_strings: Vec<String> = (0..1_000).map(|i| format!("key_{}", i)).collect();

  let start: Instant = Instant::now();
  for _ in 0..iterations / 1_000 {
    for s in &test_strings {
      let mut hasher: FxHasher = fx_build.build_hasher();
      s.hash(&mut hasher);
      let _ = std::hint::black_box(hasher.finish());
    }
  }
  let fx_str_time: Duration = start.elapsed();

  let start: Instant = Instant::now();
  for _ in 0..iterations / 1_000 {
    for s in &test_strings {
      let mut hasher: DefaultHasher = sip_build.build_hasher();
      s.hash(&mut hasher);
      let _ = std::hint::black_box(hasher.finish());
    }
  }
  let sip_str_time: Duration = start.elapsed();

  println!("\n    String keys ({} iterations):", iterations);
  println!("      FxHash:  {:?}", fx_str_time);
  println!("      SipHash: {:?}", sip_str_time);
  println!(
    "      Speedup: {:.2}x faster",
    sip_str_time.as_nanos() as f64 / fx_str_time.as_nanos() as f64
  );
}

/// Practical example: Symbol table for a compiler/interpreter.
///
/// This is FxHash's ideal use case - a compiler controls its input
/// (source code), so HashDoS resistance isn't needed. Speed matters
/// because compilers do millions of symbol lookups.
pub fn compiler_symbol_table() {
  println!("\n  Practical Example: Compiler Symbol Table");

  // A symbol table maps identifiers to their semantic information.
  // Compilers look up symbols constantly during parsing, type checking,
  // and code generation.

  #[derive(Debug, Clone)]
  struct Symbol {
    name: String,
    kind: SymbolKind,
    scope_level: u32,
  }

  #[derive(Debug, Clone)]
  enum SymbolKind {
    Variable,
    Function,
    Type,
  }

  // Use FxHashMap for fast lookups during compilation.
  // The input (source code) comes from files the developer controls,
  // so there's no risk of HashDoS attacks.
  let mut symbols: FxHashMap<String, Symbol> = FxHashMap::default();

  // Simulate parsing a source file and building the symbol table
  let identifiers: [(&str, SymbolKind); 5] = [
    ("main", SymbolKind::Function),
    ("x", SymbolKind::Variable),
    ("y", SymbolKind::Variable),
    ("Point", SymbolKind::Type),
    ("helper", SymbolKind::Function),
  ];

  for (name, kind) in identifiers {
    symbols.insert(
      name.to_string(),
      Symbol {
        name: name.to_string(),
        kind,
        scope_level: 0,
      },
    );
  }

  println!("    Symbol table contents:");
  for (name, symbol) in &symbols {
    println!("      {} -> {:?}", name, symbol.kind);
  }

  // Fast lookups during semantic analysis
  if let Some(symbol) = symbols.get("main") {
    println!("\n    Found entry point: {:?}", symbol);
  }

  // Simulate multiple lookups (what happens during type checking)
  let lookups: [&str; 5] = ["x", "y", "main", "Point", "unknown"];
  println!("\n    Performing lookups:");
  for name in lookups {
    match symbols.get(name) {
      Some(sym) => println!("      {} -> found ({:?})", name, sym.kind),
      None => println!("      {} -> not found", name),
    }
  }
}

/// Practical example: String interning.
///
/// String interning stores each unique string once and returns
/// references to the stored copy. This saves memory when the same
/// strings appear many times (common in compilers and parsers).
/// FxHash makes lookups fast.
pub fn string_interning() {
  println!("\n  Practical Example: String Interning");

  // An interner stores unique strings and returns references to them.
  // This is useful when you have many duplicate strings (like identifiers
  // in source code) and want to save memory and enable fast comparison.

  struct Interner {
    strings: FxHashSet<Rc<str>>,
  }

  impl Interner {
    fn new() -> Self {
      Interner {
        strings: FxHashSet::default(),
      }
    }

    fn intern(&mut self, s: &str) -> Rc<str> {
      // Check if we already have this string
      if let Some(existing) = self.strings.get(s) {
        return existing.clone();
      }

      // Store new string and return a reference to it
      let rc: Rc<str> = Rc::from(s);
      self.strings.insert(rc.clone());
      rc
    }

    fn stats(&self) -> usize {
      self.strings.len()
    }
  }

  let mut interner: Interner = Interner::new();

  // Intern some strings (with duplicates, simulating repeated identifiers)
  let words: [&str; 6] = ["hello", "world", "hello", "rust", "world", "hello"];

  println!("    Interning strings:");
  for word in words {
    let interned: Rc<str> = interner.intern(word);
    // Show the pointer address - same strings get same pointer
    println!(
      "      Interned {:?} -> ptr {:?}",
      word,
      Rc::as_ptr(&interned)
    );
  }

  println!("\n    Total unique strings stored: {}", interner.stats());
  println!("    Notice: Same strings get the same pointer!");
  println!("    This saves memory and enables O(1) string comparison by pointer.");
}
```

Update `src/main.rs` to include FxHash examples:

```rust
// src/main.rs
//..

mod fxhash_examples;

use fxhash_examples::run_all as fxhash_run_all;

//..

fn main() {
  //..

  fxhash_run_all();
}
```

Run the FxHash examples:

```bash
cargo run
```

The key differences from SipHash:

| Property              | FxHash (`rustc-hash` 2.1.1)                         | SipHash (as used by default `HashMap`)                                   |
| --------------------- |-----------------------------------------------------|--------------------------------------------------------------------------|
| Hasher state          | `usize` (32 or 64 bits)                             | 4×u64 = 256-bit internal state                                           |
| Key / seed            | default seed = 0 (deterministic); seeding supported | 128-bit key; default `HashMap` uses random seeding                       |
| DoS resistance goal   | not designed for HashDoS resistance                 | chosen to resist HashDoS                                                 |
| Typical perf tradeoff | very fast for common/compiler-style keys            | slower for tiny keys, competitive for “medium”, safer for untrusted keys |


#### Key takeaways for FxHash

| Property       | Value (for `rustc-hash` 2.1.1)                                                                                                         |
| -------------- |----------------------------------------------------------------------------------------------------------------------------------------|
| Crate          | `rustc-hash`                                                                                                                           |
| Core structure | fast polynomial hash over integer writes + rotate in `finish()`; `&[u8]`/`&str` are first compressed by a wyhash-inspired `hash_bytes` |
| Output         | `u64` from `finish()` (on 32-bit targets, derived from 32-bit state)                                                                   |
| Security       | non-cryptographic; byte hashing is explicitly non-collision-resistant; not intended for HashDoS defense                                |
| Seeding        | deterministic by default (seed = 0); explicit seeding supported (`with_seed`, `FxSeededState`)                                         |
| Best for       | trusted/internal keys where adversarial collision attacks aren’t a concern                                                             |


**Summary**: FxHash trades security for speed. It's ideal when you control the input (compilers, internal caches), but
dangerous when processing untrusted data. The Rust compiler uses it because source code comes from files the developer
controls - not from attackers over the network.

---

#### Step 4: aHash - speed meets security

aHash attempts to provide the best of both worlds: near FxHash speed with some resistance to HashDoS attacks. It
achieves this through hardware acceleration (AES-NI instructions on x86) and careful algorithm design.

**The innovation**: aHash uses CPU instructions designed for AES encryption to mix hash state. These instructions are
extremely fast (often single-cycle) and provide excellent bit diffusion. On CPUs without AES-NI, aHash falls back to
a quality software implementation.

**The trade-off**: aHash isn't cryptographically secure, but it's designed to make HashDoS attacks impractical. It uses
per-HashMap random seeds like SipHash, preventing attackers from pre-computing collisions.

Create `src/ahash_examples.rs`:

```rust
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
```

Update `src/main.rs` to include aHash examples:

```rust
// src/main.rs
//..

mod ahash_examples;

use ahash_examples::run_all as ahash_run_all;

fn main() {
  //..

  ahash_run_all();
}
```

Run the aHash examples:

```bash
cargo run
```

#### Key takeaways for aHash

| Property    | Validated value (ahash 0.8.12)                                                                           |
| ----------- |----------------------------------------------------------------------------------------------------------|
| Crate       | `ahash`                                                                                                  |
| Algorithm   | Keyed hash; AES-rounds via AES-NI on supported x86/x86_64; folded-multiply-based fallback                |
| Output size | 64-bit (`u64` from `Hasher::finish()`)                                                                   |
| Security    | HashDoS resistant **when keyed** (use `RandomState`); **not** cryptographically secure                   |
| Speed       | Often **~10–20× vs SipHash-1-3** in the crate’s benchmarks (varies by CPU/flags/input)                   |
| Hardware    | AES-NI acceleration on x86/x86_64 (per FAQ); otherwise fallback                                          |
| Best for    | In-memory `HashMap`/`HashSet` where you want speed + HashDoS resistance (not persistence/network/crypto) |

**Summary**: aHash provides an excellent balance of speed and security. It's a compelling choice when you need better
performance than SipHash but can't guarantee trusted input like FxHash requires. The automatic hardware acceleration
means you get optimal performance without platform specific code.

---

#### Step 5: Foldhash - the modern contender

Foldhash is a relatively new hasher designed with modern insights into both hash quality and performance. It emerged
from extensive research into what makes hash functions both fast and well-distributed.

**The philosophy**: Foldhash prioritizes hash quality (uniform distribution, good avalanche) while still being fast.
It's particularly effective for hash table use cases where distribution quality directly impacts performance.

**The approach**: Foldhash uses a "folding" technique that combines multiple values efficiently while maintaining
good statistical properties. It's designed to work well on modern CPUs with their deep pipelines and wide registers.

Create `src/foldhash_examples.rs`:

```rust
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
```

Update `src/main.rs` to include Foldhash examples:

```rust
// src/main.rs
//..

mod foldhash_examples;

use foldhash_examples::run_all as foldhash_run_all;

//..

fn main() {
  //..

  foldhash_run_all();
}
```

Run the Foldhash examples:

```bash
cargo run
```

#### Key takeaways for Foldhash

| Property               | Recommended wording                                                                                                           |
| ---------------------- |-------------------------------------------------------------------------------------------------------------------------------|
| Crate                  | `foldhash`                                                                                                                    |
| Core mixing idea       | **Folded multiply**: multiply to 128-bit then XOR-fold high/low halves (used within a keyed hasher)                           |
| Output size            | 64-bit hash (`u64`)                                                                                                           |
| Variants               | `fast` (hash tables) and `quality` (statistical-quality; post-processed to avalanche)                                         |
| DoS / security posture | **Minimally DoS-resistant**, **not cryptographic**, and not for security-sensitive uses                                       |
| Performance claim      | Benchmarked vs aHash/fxhash/SipHash; often competitive, varies by platform and build flags                                    |
| Best for               | In-memory hashing for hash maps, bloom filters, sketches; use `fast` by default, `quality` when statistical properties matter |

**Summary**: Foldhash represents modern thinking about hash function design. It offers a good balance of speed and
quality, making it suitable for general-purpose use. The availability of both "fast" and "quality" variants lets you
choose the right trade-off for your specific needs.

---

#### Step 6: xxHash - the established performer

xxHash is a family of extremely fast hash functions developed by Yann Collet (also known for the LZ4 and Zstandard
compression algorithms). It's been battle-tested in production systems worldwide for over a decade.

**The history**: xxHash was designed for checksumming large amounts of data - think file integrity verification,
network protocols, and data deduplication. Its focus on raw throughput makes it excellent for hashing large keys.

**The variants**: The xxHash family includes xxHash32, xxHash64, and xxHash3. Each has different characteristics:
- xxHash32: 32-bit output, very fast, widely compatible
- xxHash64: 64-bit output, excellent for 64-bit systems
- xxHash3: The newest variant, even faster, with both 64-bit and 128-bit outputs

Create `src/xxhash_examples.rs`:

```rust
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
```

Update `src/main.rs` to include xxHash examples:

```rust
// src/main.rs
//..

mod xxhash_examples;

use xxhash_examples::run_all as xxhash_run_all;

//..

fn main() {
  //..

  xxhash_run_all();
}
```

Run the xxHash examples:

```bash
cargo run
```

#### Key takeaways for xxHash

| Property    | Recommended value                                                                                        |
| ----------- |----------------------------------------------------------------------------------------------------------|
| Crates      | `twox-hash`, `xxhash-rust`                                                                               |
| Variants    | XXH32, XXH64, XXH3 (64-bit), XXH3/XXH128 (128-bit)                                                       |
| Output size | 32-bit (XXH32), 64-bit (XXH64/XXH3_64), 128-bit (XXH3_128)                                               |
| Security    | Non-cryptographic (not for tamper-proof integrity / adversarial settings)                                |
| Speed       | Extremely fast on large inputs; XXH3/XXH64 are typically **well above** 10 GB/s on modern CPUs (varies)  |
| Best for    | Fast fingerprints: large-data hashing, sharding/partitioning, dedup prefilter, non-adversarial checksums |


**Summary**: xxHash is the speed champion for large data. If you're hashing files, network packets, or large strings,
xxHash (especially xxHash3) will outperform nearly everything else. For typical HashMap usage with small keys, other
hashers like FxHash or aHash may be more appropriate.

---

#### Step 7: NoHash - when hashing is unnecessary

NoHash takes a radically different approach: it doesn't hash at all. Instead, it uses integer keys directly as their
own "hash" values. This seems counterintuitive, but for certain workloads it's optimal.

**The insight**: If your keys are already well-distributed integers (like sequential IDs, random numbers, or pre-hashed
values), running them through another hash function is pure overhead. NoHash eliminates this overhead entirely.

**The catch**: This only works for integer types. Using NoHash with poorly distributed keys (like all even numbers)
will cause severe performance degradation due to clustering.

Create `src/nohash_examples.rs`:

```rust
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
```

Update `src/main.rs` to include xxHash examples:

```rust
// src/main.rs
//..

mod nohash_examples;

use nohash_examples::run_all as nohash_run_all;

//..

fn main() {
  //..

  nohash_run_all();
}
```

Run the NoHash examples:

```bash
cargo run
```

#### Key takeaways for NoHash

| Property          | Corrected value (nohash-hasher 0.2.0)                                                                                                                                       |
| ----------------- |-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Crate             | `nohash-hasher`                                                                                                                                                             |
| Algorithm         | No hashing/mixing: uses the single integer value written via `write_{u8,u16,u32,u64,usize,i8,i16,i32,i64,isize}` as the hash output                                         |
| Output size       | `u64` (because `Hasher::finish()` returns `u64`)                                                                                                                            |
| Security          | Not cryptographic; not suitable for adversarial/untrusted keys (no HashDoS style protection)                                                                                |
| Speed             | Very low hashing overhead for supported keys (no mixing), but not literally “zero overhead”                                                                                 |
| Enabled key types | Out of the box: `u8,u16,u32,u64,usize,i8,i16,i32,i64,isize` (not `u128/i128`). Custom types can opt in via `IsEnabled` if they hash with exactly one allowed `write_*` call |
| Best for          | In-memory maps/sets keyed by integer IDs that are already reasonably distributed (including sequential IDs)                                                                 |

**Summary**: NoHash is the ultimate optimization for integer keys that are already well-distributed. It completely
eliminates hashing overhead, giving the fastest possible HashMap performance.
However, it's easy to misuse - poorly distributed keys will cause severe performance degradation.
Use it only when you understand your key distribution.

---
