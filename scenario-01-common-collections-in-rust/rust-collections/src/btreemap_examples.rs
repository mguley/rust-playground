// BTreeMap and BTreeSet are sorted collections based on B-trees. They provide
// the same functionality as HashMap and HashSet, but with one crucial
// difference: elements are always kept in sorted order.
//
// Key trade-offs vs HashMap:
//   HashMap/HashSet         BTreeMap/BTreeSet
//   - O(1) average          - O(log n) all operations
//   - Unordered             - Always sorted
//   - Needs Hash + Eq       - Needs Ord (+ Eq implied)
//   - No range queries      - Supports range queries!

use std::cmp::Reverse;
use std::collections::BTreeMap;

/// Demonstrates all the different ways to create a BTreeMap.
///
/// Unlike HashMap, BTreeMap doesn't need a hasher - it only requires
/// that keys implement the `Ord` trait for comparison-based ordering.
pub fn creating_btreemaps() {
    // Method 1: BTreeMap::new()
    // The most common way - start empty and add items
    let mut map: BTreeMap<&str, i8> = BTreeMap::new();
    map.insert("cherry", 3);
    map.insert("apple", 1);
    map.insert("banana", 2);
    println!("Created with new(): {:?}", map);
    println!("Notice: Keys are automatically sorted alphabetically!");

    // Method 2: BTreeMap::from() - from array of tuples
    // Concise syntax for small, known maps
    let map2: BTreeMap<i8, &str> = BTreeMap::from([(3, "three"), (1, "one"), (2, "two")]);
    println!("\nFrom array: {:?}", map2);
    println!("Notice: Keys 1, 2, 3 are sorted numerically!");

    // Method 3: collect() from iterator of tuples
    // Useful when transforming data
    let map3: BTreeMap<i8, i8> = (1..=5).map(|x| (x, x * x)).collect();
    println!("\nCollected squares: {:?}", map3);

    // Note: Unlike HashMap, BTreeMap has no with_capacity() method.
    // B-trees allocate nodes as needed, so pre-allocation isn't applicable.
}

/// Demonstrates BTreeMap's feature: sorted iteration.
///
/// Every time you iterate over a BTreeMap, keys come out in sorted order.
/// This is guaranteed and deterministic - unlike HashMap's arbitrary order.
pub fn sorted_iteration() {
    let mut scores: BTreeMap<String, i8> = BTreeMap::new();

    // Insert in deliberately random order
    scores.insert("Zoe".to_string(), 85);
    scores.insert("Alice".to_string(), 92);
    scores.insert("Charlie".to_string(), 78);
    scores.insert("Bob".to_string(), 88);

    // Iteration is ALWAYS in sorted order by key!
    println!("Scores (automatically sorted by name):");
    for (name, score) in &scores {
        println!("  {}: {}", name, score);
    }

    // This deterministic ordering is impossible with HashMap!
    // HashMap iteration order can change between runs or even insertions.

    // Keys and values iterators are also sorted
    println!(
        "\nKeys only (sorted): {:?}",
        scores.keys().collect::<Vec<_>>()
    );
    println!(
        "Values in key order: {:?}",
        scores.values().collect::<Vec<_>>()
    );
}

/// Demonstrates range queries - BTreeMap's other feature
///
/// Because keys are sorted, BTreeMap can efficiently answer questions like
/// "give me all entries where the key is between X and Y". HashMap cannot
/// do this at all - you'd have to scan every entry.
pub fn range_queries() {
    println!("Range Queries");

    let mut temperature_log: BTreeMap<u16, f32> = BTreeMap::new();

    // Log temperatures at different timestamps (in seconds)
    temperature_log.insert(0, 20.5);
    temperature_log.insert(100, 21.0);
    temperature_log.insert(200, 22.3);
    temperature_log.insert(300, 23.1);
    temperature_log.insert(400, 21.8);
    temperature_log.insert(500, 20.9);

    println!("Full temperature log: {:?}", temperature_log);

    // range() with inclusive bounds: 100..=400 means 100 <= key <= 400
    println!("\nTemperatures from t=100 to t=400 (inclusive):");
    for (time, temp) in temperature_log.range(100..=400) {
        println!("  t={}: {:.1}°C", time, temp);
    }

    // range() with exclusive end: 100..400 means 100 <= key < 400
    println!("\nTemperatures from t=100 to t=400 (exclusive end):");
    for (time, temp) in temperature_log.range(100..400) {
        println!("  t={}: {:.1}°C", time, temp);
    }

    // Unbounded start: ..250 means key < 250
    println!("\nTemperatures before t=250:");
    for (time, temp) in temperature_log.range(..250) {
        println!("  t={}: {:.1}°C", time, temp);
    }

    // Unbounded end: 300.. means key >= 300
    println!("\nTemperatures from t=300 onwards:");
    for (time, temp) in temperature_log.range(300..) {
        println!("  t={}: {:.1}°C", time, temp);
    }

    // This is O(log n + k) where k is the number of results.
    // HashMap would require O(n) to scan all entries!
}

/// Demonstrates mutable range queries with range_mut().
///
/// You can modify values within a range without affecting the tree structure,
/// as long as you don't change the keys.
pub fn mutable_range_queries() {
    println!("Mutable Range Queries");

    let mut readings: BTreeMap<i8, f32> =
        BTreeMap::from([(1, 10.0), (2, 20.0), (3, 30.0), (4, 40.0), (5, 50.0)]);

    println!("Before modification: {:?}", readings);

    // Double all values where key is between 2 and 4
    for (_key, value) in readings.range_mut(2..=4) {
        *value *= 2.0;
    }

    println!("After doubling values for keys 2-4: {:?}", readings);
}

/// Demonstrates first/last key access - finding min and max keys.
///
/// Because keys are sorted, finding the smallest (first) or largest (last)
/// key is O(log n) - just traverse to the appropriate leaf node.
pub fn min_max_operations() {
    println!("Min/Max (First/Last) Operations");

    let mut prices: BTreeMap<&str, f32> = BTreeMap::new();
    prices.insert("apple", 1.50);
    prices.insert("banana", 0.75);
    prices.insert("cherry", 3.00);
    prices.insert("date", 2.25);

    println!("Price list: {:?}", prices);

    // Peek at first (smallest key) and last (largest key) without removing
    if let Some((item, price)) = prices.first_key_value() {
        println!("\nFirst item (alphabetically): {} at ${:.2}", item, price);
    }
    if let Some((item, price)) = prices.last_key_value() {
        println!("Last item (alphabetically): {} at ${:.2}", item, price);
    }

    // first_entry() and last_entry() give mutable access via OccupiedEntry
    if let Some(mut entry) = prices.first_entry() {
        println!("\nFirst entry key: {}", entry.key());
        // We can modify the value
        *entry.get_mut() = 1.75;
        println!("Updated first entry value to: ${:.2}", entry.get());
    }

    println!("After modifying first entry: {:?}", prices);

    // pop_first() and pop_last() remove and return
    let first: Option<(&str, f32)> = prices.pop_first();
    println!("\npop_first() returned: {:?}", first);

    let last: Option<(&str, f32)> = prices.pop_last();
    println!("pop_last() returned: {:?}", last);

    println!("Remaining after pops: {:?}", prices);
}

/// Demonstrates the Entry API - same patterns as HashMap.
///
/// BTreeMap supports the same entry API as HashMap for efficient
/// conditional insertion and updates.
pub fn entry_api_examples() {
    println!("Entry API with BTreeMap");

    // Pattern 1: Word frequency counter (same as HashMap example)
    let text: &str = "the quick brown fox jumps over the lazy dog the fox";
    let mut word_count: BTreeMap<&str, i8> = BTreeMap::new();

    for word in text.split_whitespace() {
        *word_count.entry(word).or_insert(0) += 1;
    }

    // Unlike HashMap, iteration is alphabetically sorted!
    println!("Word counts (alphabetically sorted):");
    for (word, count) in &word_count {
        println!("  {}: {}", word, count);
    }

    // Pattern 2: or_insert_with for lazy initialization
    let mut cache: BTreeMap<i8, String> = BTreeMap::new();

    let value: &mut String = cache.entry(42).or_insert_with(|| {
        println!("  Computing value for key 42...");
        "computed".to_string()
    });
    println!("\nFirst access: {}", value);

    // Second access - closure doesn't run
    let value2: &mut String = cache.entry(42).or_insert_with(|| {
        println!("  This won't print!");
        "won't happen".to_string()
    });
    println!("Second access (cached): {}", value2);

    // Pattern 3: and_modify + or_insert
    word_count
        .entry("fox")
        .and_modify(|c| *c += 100)
        .or_insert(1);
    println!(
        "\nAfter boosting 'fox': fox count = {:?}",
        word_count.get("fox")
    );
}

/// Demonstrates using BTreeMap for a sorted leaderboard.
///
/// A common challenge: BTreeMap sorts by key in ascending order,
/// but leaderboards typically show highest scores first.
/// Solution: Use std::cmp::Reverse to invert the ordering.
pub fn leaderboard_example() {
    println!("Leaderboard Example (Descending Order)");

    // Using Reverse<i32> as the score component makes higher scores sort first
    // The tuple (Reverse(score), name) ensures:
    //   1. Higher scores come first (due to Reverse)
    //   2. Ties are broken alphabetically by name
    let mut leaderboard: BTreeMap<(Reverse<i32>, String), ()> = BTreeMap::new();

    // Insert players with their scores
    leaderboard.insert((Reverse(1500), "Alice".to_string()), ());
    leaderboard.insert((Reverse(1200), "Bob".to_string()), ());
    leaderboard.insert((Reverse(1800), "Charlie".to_string()), ());
    leaderboard.insert((Reverse(1350), "Diana".to_string()), ());
    leaderboard.insert((Reverse(1500), "Eve".to_string()), ()); // Same score as Alice

    println!("Leaderboard (highest scores first):");
    for (rank, ((Reverse(score), name), _)) in leaderboard.iter().enumerate() {
        println!("  {}. {} - {} points", rank + 1, name, score);
    }

    // Get top 3 players
    println!("\nTop 3 players:");
    for ((Reverse(score), name), _) in leaderboard.iter().take(3) {
        println!("  {} - {} points", name, score);
    }

    // Alternative approach: use negative scores (simpler but less clear)
    println!("\n--- Alternative: Negative Score Trick ---");
    let mut simple_leaderboard: BTreeMap<(i32, String), ()> = BTreeMap::new();

    simple_leaderboard.insert((-100, "Alice".to_string()), ());
    simple_leaderboard.insert((-85, "Bob".to_string()), ());
    simple_leaderboard.insert((-92, "Charlie".to_string()), ());

    println!("Using negated scores:");
    for ((neg_score, name), _) in &simple_leaderboard {
        println!("  {}: {} points", name, -neg_score);
    }
}

/// Practical example: Time-series data storage and querying.
///
/// BTreeMap excels at time-series data because timestamps are naturally
/// ordered, and range queries let you efficiently retrieve time windows.
pub fn time_series_example() {
    println!("Practical Example: Time-Series Data");

    #[derive(Debug)]
    struct Measurement {
        temperature: f64,
        humidity: f64,
    }

    let mut readings: BTreeMap<u64, Measurement> = BTreeMap::new();

    // Add sensor readings (timestamp in milliseconds as key)
    readings.insert(
        1_000,
        Measurement {
            temperature: 22.5,
            humidity: 45.0,
        },
    );
    readings.insert(
        1_100,
        Measurement {
            temperature: 23.0,
            humidity: 43.0,
        },
    );
    readings.insert(
        1_200,
        Measurement {
            temperature: 24.5,
            humidity: 40.0,
        },
    );
    readings.insert(
        1_300,
        Measurement {
            temperature: 26.0,
            humidity: 38.0,
        },
    );
    readings.insert(
        1_400,
        Measurement {
            temperature: 25.5,
            humidity: 42.0,
        },
    );

    println!("All readings (chronologically sorted):");
    for (time, data) in &readings {
        println!(
            "  t={}: {:.1}°C, {:.1}% humidity",
            time, data.temperature, data.humidity
        );
    }

    // Query a specific time window
    println!("\nReadings between t=1100 and t=1300:");
    for (time, data) in readings.range(1_100..=1_300) {
        println!(
            "  t={}: {:.1}°C, {:.1}% humidity",
            time, data.temperature, data.humidity
        );
    }

    // Get the latest reading efficiently
    if let Some((time, data)) = readings.last_key_value() {
        println!("\nLatest reading (t={}): {:.1}°C", time, data.temperature);
    }

    // Get the earliest reading
    if let Some((time, data)) = readings.first_key_value() {
        println!("Earliest reading (t={}): {:.1}°C", time, data.temperature);
    }
}

/// Practical example: Calendar/scheduling with time-based keys.
///
/// Using tuples as keys allows multi-level sorting - perfect for
/// dates and times where you want to sort by hour, then minute.
pub fn calendar_example() {
    println!("Practical Example: Calendar Events");

    #[derive(Debug)]
    struct Event {
        title: String,
        duration_mins: u32,
    }

    // Key is (hour, minute) - sorts chronologically within a day
    let mut calendar: BTreeMap<(u8, u8), Event> = BTreeMap::new();

    calendar.insert(
        (9, 0),
        Event {
            title: "Standup".to_string(),
            duration_mins: 15,
        },
    );
    calendar.insert(
        (10, 30),
        Event {
            title: "Design Review".to_string(),
            duration_mins: 60,
        },
    );
    calendar.insert(
        (14, 0),
        Event {
            title: "Team Sync".to_string(),
            duration_mins: 30,
        },
    );
    calendar.insert(
        (16, 0),
        Event {
            title: "1:1 with Manager".to_string(),
            duration_mins: 30,
        },
    );
    calendar.insert(
        (12, 0),
        Event {
            title: "Lunch".to_string(),
            duration_mins: 60,
        },
    );

    println!("Today's schedule (automatically sorted by time):");
    for ((hour, min), event) in &calendar {
        println!(
            "  {:02}:{:02} - {} ({} min)",
            hour, min, event.title, event.duration_mins
        );
    }

    // Find afternoon events (12:00 and later)
    println!("\nAfternoon events:");
    for ((hour, min), event) in calendar.range((12, 0)..) {
        println!("  {:02}:{:02} - {}", hour, min, event.title);
    }

    // Find morning events (before 12:00)
    println!("\nMorning events:");
    for ((hour, min), event) in calendar.range(..(12, 0)) {
        println!("  {:02}:{:02} - {}", hour, min, event.title);
    }
}

/// Demonstrates using custom types as BTreeMap keys.
///
/// Unlike HashMap (which requires Hash + Eq), BTreeMap requires Ord.
/// The Ord trait defines a total ordering, which the B-tree uses to
/// organize and search for keys.
pub fn custom_key_types() {
    println!("Custom Types as Keys");

    // Derive Ord (and its prerequisites) for automatic ordering
    // The derived ordering compares fields in declaration order:
    // first by major, then minor, then patch
    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
    struct Version {
        major: u8,
        minor: u8,
        patch: u8,
    }

    impl Version {
        fn new(major: u8, minor: u8, patch: u8) -> Self {
            Version {
                major,
                minor,
                patch,
            }
        }
    }

    let mut releases: BTreeMap<Version, &str> = BTreeMap::new();

    // Insert in random order
    releases.insert(Version::new(2, 0, 0), "Major update");
    releases.insert(Version::new(1, 0, 0), "Initial release");
    releases.insert(Version::new(1, 1, 0), "Added features");
    releases.insert(Version::new(1, 0, 1), "Bug fix");
    releases.insert(Version::new(1, 2, 0), "More features");

    println!("Release history (sorted by version):");
    for (version, description) in &releases {
        println!(
            "  v{}.{}.{}: {}",
            version.major, version.minor, version.patch, description
        );
    }

    // Range query: find all 1.x releases
    let start: Version = Version::new(1, 0, 0);
    let end: Version = Version::new(2, 0, 0);
    println!("\nAll 1.x releases:");
    for (version, description) in releases.range(start..end) {
        println!(
            "  v{}.{}.{}: {}",
            version.major, version.minor, version.patch, description
        );
    }
}
