use std::collections::HashMap;
use std::hash::{Hash, Hasher};

/// Demonstrates all the different ways to create a HashMap
pub fn creating_hashmaps() {
    // Method 1: HashMap::new()
    // The most common way - start empty and add items
    let mut scores: HashMap<String, i8> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);
    println!("Created with new(): {:?}", scores);

    // Method 2: collect() from an iterator of tuples
    // Useful when you already have data in tuple form
    let teams: Vec<(String, i8)> = vec![(String::from("Green"), 25), (String::from("Yellow"), 30)];
    let scores2: HashMap<String, i8> = teams.into_iter().collect();
    println!("Created with collect(): {:?}", scores2);

    // Method 3: From arrays (Rust 1.56+)
    // Concise syntax for small, known maps
    let scores3: HashMap<&str, i16> = HashMap::from([("Alpha", 100), ("Beta", 200)]);
    println!("Created with from(): {:?}", scores3);

    // Method 4: with_capacity (performance optimization)
    // When you know approximately how many items you'll store,
    // pre-allocating avoids repeated reallocations
    let large_map: HashMap<i32, i32> = HashMap::with_capacity(1_000);
    println!(
        "Created with capacity 1_000, current len: {}",
        large_map.len()
    );

    // Method 5: Using zip to combine two iterables
    // Perfect when keys and values come from separate sources
    let keys: Vec<&str> = vec!["a", "b", "c"];
    let values: Vec<i8> = vec![1, 2, 3];
    let zipped: HashMap<&str, i8> = keys.into_iter().zip(values).collect();
    println!("Created with zip: {:?}", zipped);
}

/// Demonstrates HashMap operations: insert, get, contains_key, update, and remove
pub fn basic_hashmap_operations() {
    println!("Basic Hashmap Operations");

    // Create a new HashMap to track player scores
    let mut scores: HashMap<String, i8> = HashMap::new();

    // Insert key-value pairs
    // insert() returns Option<V> - Some(old_value) if key existed, None otherwise
    scores.insert("Alice".to_string(), 100);
    scores.insert("Bob".to_string(), 85);
    scores.insert("Charlie".to_string(), 92);
    println!("Initial scores: {:?}", scores);

    // Access values with get() - returns Option<&V>
    // This is the safe way to access values (won't panic)
    match scores.get("Alice") {
        Some(score) => println!("Alice's score: {}", score),
        None => println!("Alice not found"),
    }

    // You can also use unwrap_or for a default value
    let alice_score: &i8 = scores.get("Alice").unwrap_or(&0);
    let unknown_score: &i8 = scores.get("Unknown").unwrap_or(&0);
    println!(
        "With unwrap_or: Alice={}, Unknown={}",
        alice_score, unknown_score
    );

    // Check if key exists without retrieving the value
    if scores.contains_key("Bob") {
        println!("Bob is in the scoreboard");
    }

    // Update a value by inserting with the same key
    // The old value is returned
    let old_score: Option<i8> = scores.insert("Alice".to_string(), 105);
    println!(
        "Alice's score updated from {:?} to {:?}",
        old_score,
        scores.get("Alice")
    );

    // Remove a key-value pair
    // Returns Option<V> with the removed value
    let removed: Option<i8> = scores.remove("Charlie");
    println!("Removed Charlie with score: {:?}", removed);
    println!("Final scores: {:?}", scores);
}

/// Demonstrates the Entry API - Rust's solution for conditional insertion and updates.
///
/// The Entry API handles the common pattern of "check if key exists, then insert
/// or update" in a single, efficient operation. This avoids the inefficiency of
/// doing two separate lookups (contains_key + insert) and is more idiomatic Rust.
pub fn entry_api_examples() {
    println!("The Entry API Examples");

    // Use case: word frequency counter
    let text: &str = "hello world hello rust world rust rust";
    let mut word_count: HashMap<String, i8> = HashMap::new();

    // The inefficient way (two lookups per word - don't do this!):
    // if !word_count.contains_key(word) {
    //     word_count.insert(word.to_string(), 0);
    // }
    // *word_count.get_mut(word).unwrap() += 1;

    // The Entry API way (one lookup, idiomatic):
    for word in text.split_whitespace() {
        // entry() returns an Entry enum (Occupied or Vacant)
        // or_insert() inserts default if vacant, returns &mut to value
        let count: &mut i8 = word_count.entry(word.to_string()).or_insert(0);
        *count += 1;
    }
    println!("Word counts: {:?}", word_count);

    // or_insert_with() - lazy initialization with a closure
    // The closure only executes if the key doesn't exist
    let mut expensive_cache: HashMap<i8, String> = HashMap::new();

    fn expensive_computation(key: i8) -> String {
        println!("Computing expensive value for key {}...", key);
        format!("computed-{}", key)
    }

    let key: i8 = 42;
    let value: &mut String = expensive_cache
        .entry(key)
        .or_insert_with(|| expensive_computation(key));
    println!("First access: {}", value);

    // Second access - the closure doesn't run because key exists
    let value2: &mut String = expensive_cache
        .entry(key)
        .or_insert_with(|| expensive_computation(key));
    println!("Second access (cached): {}", value2);

    // or_default() - uses the type's Default trait value
    // For i32, default is 0; for String, default is ""
    let mut counts: HashMap<&str, i8> = HashMap::new();
    *counts.entry("visits").or_default() += 1;
    *counts.entry("visits").or_default() += 1;
    println!("Using or_default(): {:?}", counts);

    // and_modify() - modify existing value, can chain with or_insert
    // Pattern: "add to existing, or start at initial value"
    let mut inventory: HashMap<&str, i8> = HashMap::new();
    inventory.insert("apple", 5);

    // Add 3 to existing count, or start at 3 if new
    inventory
        .entry("apple")
        .and_modify(|count| *count += 3)
        .or_insert(3);
    inventory
        .entry("banana")
        .and_modify(|count| *count += 3)
        .or_insert(3);
    println!("Inventory after and_modify: {:?}", inventory);
}

/// Demonstrates the ways to read values from a HashMap.
pub fn accessing_values() {
    println!("Accessing values");

    let mut scores: HashMap<&str, i8> = HashMap::new();
    scores.insert("Alice", 100);
    scores.insert("Bob", 85);

    // get() - returns Option<&V>, the safest approach
    match scores.get("Alice") {
        Some(score) => println!("Alice's score: {}", score),
        None => println!("Alice not found"),
    }
    match scores.get("Charlie") {
        Some(score) => println!("Charlie's score: {}", score),
        None => println!("Charlie not found"),
    }

    // contains_key() - check existence without getting value
    println!("\ncontains_key(\"Bob\"): {}", scores.contains_key("Bob"));
    println!("contains_key(\"Eve\"): {}", scores.contains_key("Eve"));

    // get_mut() - get a mutable reference to modify in place
    if let Some(score) = scores.get_mut("Alice") {
        *score += 10;
        println!("Alice's score: {}", score);
    }

    // get_key_value() - get both key and value as a tuple
    if let Some((key, value)) = scores.get_key_value("Bob") {
        println!("get_key_value result: {} -> {}", key, value);
    }
}

/// Demonstrates methods for removing entries from a HashMap.
pub fn removing_values() {
    println!("Removing values");

    let mut map: HashMap<&str, i8> = HashMap::from([("a", 1), ("b", 2), ("c", 3), ("d", 4)]);
    println!("Starting map: {:?}", map);

    // remove() - remove by key, returns Option<V> with the value
    let removed: Option<i8> = map.remove("b");
    println!("remove(\"b\") returned: {:?}", removed);
    println!("Map after remove: {:?}", map);

    // remove_entry() - get both the key and value back
    if let Some((key, value)) = map.remove_entry("c") {
        println!("remove_entry returned: {} -> {}", key, value);
    }

    // retain() - keep only entries matching a predicate
    // Great for bulk filtering
    let square: fn(i8) -> i8 = |n: i8| n * n;
    let mut numbers: HashMap<i8, i8> = (0..10).map(|i| (i, square(i))).collect();
    println!("Before retain: {:?}", numbers);

    let keep_even: fn(&i8, &mut i8) -> bool = |key, _value| key % 2 == 0;
    numbers.retain(|key, _value| keep_even(key, _value)); // Keep only even keys
    println!("After retain (even keys only): {:?}", numbers);

    // clear() - remove all entries
    let mut to_clear: HashMap<&str, i8> = HashMap::from([("x", 1), ("y", 2)]);
    println!("Before clear: {:?}", to_clear);
    to_clear.clear();
    println!(
        "After clear: {:?}, is_empty: {}",
        to_clear,
        to_clear.is_empty()
    );
}

/// Demonstrates all iteration patterns for HashMaps.
pub fn iterating_hashmaps() {
    println!("Iterating over HashMaps");

    let mut capitals: HashMap<&str, &str> = HashMap::new();
    capitals.insert("France", "Paris");
    capitals.insert("Japan", "Tokyo");
    capitals.insert("Brazil", "Brasília");
    capitals.insert("Australia", "Canberra");

    // Iterate over key-value pairs (order is NOT guaranteed!)
    println!("All capitals (immutable iteration):");
    for (country, capital) in &capitals {
        println!("{} -> {}", country, capital);
    }

    // Iterate over keys only
    println!("\nKeys only: {:?}", capitals.keys().collect::<Vec<_>>());

    // Iterate over values only
    println!("Values only: {:?}", capitals.values().collect::<Vec<_>>());

    // Mutable iteration - modify values in place
    let mut scores: HashMap<&str, i8> =
        HashMap::from([("Alice", 95), ("Bob", 87), ("Charlie", 91)]);
    println!("Before curve: {:?}", scores);
    for (_name, score) in &mut scores {
        *score = (*score + 5).min(100); // Add 5 points, cap at 100
    }
    println!("After curve: {:?}", scores);

    // Consuming iteration - takes ownership of the HashMap
    let temp_map: HashMap<&str, i8> = HashMap::from([("x", 1), ("y", 2)]);
    println!("Consuming iteration:");
    for (key, value) in temp_map {
        // temp_map is moved here
        println!("Consumed: {} -> {}", key, value);
    }
    // println!("{:?}", temp_map);  // ERROR: temp_map was moved!
}

/// Demonstrates how HashMap interacts with Rust's ownership system.
pub fn ownership_and_borrowing() {
    println!("Ownership and borrowing");

    // For Copy types (like i32), values are copied into the HashMap
    let mut map: HashMap<i8, i8> = HashMap::new();
    let key: i8 = 1;
    let value: i8 = 100;
    map.insert(key, value);
    // key and value are still valid because i32 implements Copy!
    println!(
        "Copy types: key and value still valid after insert: {}, {}",
        key, value
    );

    // For owned types (like String), ownership transfers to the HashMap
    let mut map: HashMap<String, String> = HashMap::new();
    let key: String = String::from("name");
    let value: String = String::from("Rust");
    map.insert(key, value);
    // println!("{}", key);     // ERROR: key was moved into the HashMap
    // println!("{}", value);   // ERROR: value was moved into the HashMap
    println!("Owned types moved into HashMap: {:?}", map);

    // Using references as keys - the referenced data must outlive the HashMap
    let names: Vec<String> = vec![String::from("Alice"), String::from("Bob")];
    let mut ages: HashMap<&str, i8> = HashMap::new();
    ages.insert(&names[0], 30); // Borrowing from names vector
    ages.insert(&names[1], 25);
    println!("Borrowed keys from vec: {:?}", ages);
    // ages must not outlive names, or we'd have dangling references

    // Common pattern: clone to avoid ownership issues
    let key: String = String::from("language");
    let value: String = String::from("Rust");
    let mut map: HashMap<String, String> = HashMap::new();
    map.insert(key.clone(), value.clone()); // Clone to keep originals
    println!("After cloning: originals still valid: {} = {}", key, value);
    println!("HashMap contains: {:?}", map);
}

/// Demonstrates using custom types as HashMap keys.
///
/// To use a custom type as a key, it must implement:
/// - `Hash`: to compute the hash value for bucket placement
/// - `Eq`: for equality comparison (handles hash collisions)
/// - `PartialEq`: required by Eq
///
/// This function shows both manual implementation and the derive macro approach.
pub fn custom_keys() {
    println!("Custom Types as Keys");

    // First, let's see the manual implementation approach
    // Normally you'd just derive these traits

    #[derive(Debug)]
    struct PointManual {
        x: i32,
        y: i32,
    }

    // Implement PartialEq for equality comparison
    impl PartialEq for PointManual {
        fn eq(&self, other: &Self) -> bool {
            self.x == other.x && self.y == other.y
        }
    }

    // Eq is a marker trait - just declares that equality is reflexive,
    // symmetric, and transitive (which PartialEq doesn't guarantee for floats)
    impl Eq for PointManual {}

    // Implement Hash to compute the hash value
    impl Hash for PointManual {
        fn hash<H: Hasher>(&self, state: &mut H) {
            // Hash each field - order matters for consistency
            self.x.hash(state);
            self.y.hash(state);
        }
    }

    let mut point_values: HashMap<PointManual, String> = HashMap::new();
    point_values.insert(PointManual { x: 0, y: 0 }, "origin".to_string());
    point_values.insert(PointManual { x: 1, y: 1 }, "diagonal".to_string());
    println!("Manual implementation - Point map: {:?}", point_values);

    // The easy way: derive the traits automatically
    // This is what you should normally do for simple structs

    #[derive(Hash, Eq, PartialEq, Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    let mut locations: HashMap<Point, &str> = HashMap::new();
    locations.insert(Point { x: 0, y: 0 }, "Origin");
    locations.insert(Point { x: 10, y: 20 }, "Point A");
    locations.insert(Point { x: -5, y: 3 }, "Point B");

    // Look up by creating an equivalent Point
    let query: Point = Point { x: 0, y: 0 };
    println!(
        "Derived implementation - Location at {:?}: {:?}",
        query,
        locations.get(&query)
    );

    // Note on performance: Rust uses SipHash by default for security against
    // HashDoS attacks. For performance-critical code with trusted input,
    // consider using a faster hasher like FxHash or AHash.
}
