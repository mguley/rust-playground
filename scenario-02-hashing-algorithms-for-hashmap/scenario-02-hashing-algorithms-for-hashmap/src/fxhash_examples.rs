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
