// HashSet and BTreeSet are collections of unique values with no associated data.
// They're essentially maps where you only care about the keys (implemented as
// HashMap<T, ()> and BTreeMap<T, ()> under the hood).
//
// Key trade-offs:
//   HashSet<T>              BTreeSet<T>
//   - O(1) average          - O(log n) all operations
//   - Unordered             - Always sorted
//   - Needs Hash + Eq       - Needs Ord
//   - No range queries      - Supports range queries!

use std::collections::{BTreeSet, HashSet};

/// Demonstrates all the different ways to create a HashSet.
///
/// HashSet requires elements to implement Hash + Eq traits.
/// Like HashMap, it uses SipHash by default for security.
pub fn creating_hashsets() {
    // Method 1: HashSet::new()
    // The most common way - start empty and add items
    let mut set: HashSet<i8> = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    println!("Created with new(): {:?}", set);

    // Method 2: HashSet::from() - from array (Rust 1.56+)
    // Concise syntax for small, known sets
    let set2: HashSet<&str> = HashSet::from(["apple", "banana", "cherry"]);
    println!("Created from array: {:?}", set2);

    // Method 3: collect() from iterator
    // Useful when transforming data
    let set3: HashSet<i8> = (1..=5).collect();
    println!("Created with collect(): {:?}", set3);

    // Method 4: From a Vec (automatically deduplicates!)
    // This is a common pattern for removing duplicates
    let numbers: Vec<i8> = vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4];
    let unique: HashSet<i8> = numbers.into_iter().collect();
    println!("From vec with duplicates: {:?}", unique);
    println!("  (Original had 10 items, set has {})", unique.len());

    // Method 5: with_capacity (performance optimization)
    // Pre-allocate when you know approximate size
    let large_set: HashSet<i8> = HashSet::with_capacity(1_000);
    println!(
        "Created with capacity 1000, current len: {}",
        large_set.len()
    );
}

/// Demonstrates adding and removing elements from a HashSet.
///
/// Key insight: insert() returns a bool indicating whether the value
/// was newly added (true) or already existed (false).
pub fn adding_removing_elements() {
    println!("Adding and removing elements");

    let mut fruits: HashSet<&str> = HashSet::new();

    // insert() returns true if the value was newly added
    let added1: bool = fruits.insert("apple");
    let added2: bool = fruits.insert("banana");
    let added3: bool = fruits.insert("apple"); // Duplicate!

    println!("insert(\"apple\"): {} (was new)", added1);
    println!("insert(\"banana\"): {} (was new)", added2);
    println!("insert(\"apple\"): {} (already existed!)", added3);
    println!("Set after inserts: {:?}", fruits);

    // remove() returns true if the value was present
    let removed1: bool = fruits.remove("apple");
    let removed2: bool = fruits.remove("grape"); // Not in set

    println!("\nremove(\"apple\"): {} (was present)", removed1);
    println!("remove(\"grape\"): {} (wasn't there)", removed2);
    println!("Set after removes: {:?}", fruits);

    // take() - removes and returns the value (if present)
    // Useful when you need ownership of the removed value
    fruits.insert("cherry");
    fruits.insert("date");
    let taken: Option<&str> = fruits.take("cherry");
    println!("\ntake(\"cherry\"): {:?}", taken);
    println!("Set after take: {:?}", fruits);

    // retain() - keep only elements matching a predicate
    // Great for bulk filtering
    let mut numbers: HashSet<i8> = (1..=10).collect();
    println!("\nBefore retain: {:?}", numbers);
    numbers.retain(|&x| x % 2 == 0); // Keep only even numbers
    println!("After retain (even only): {:?}", numbers);

    // clear() - remove all elements
    numbers.clear();
    println!(
        "After clear: {:?}, is_empty: {}",
        numbers,
        numbers.is_empty()
    );
}

/// Demonstrates checking membership in a HashSet.
///
/// Membership testing is O(1) average - this is the primary use case for HashSet.
pub fn checking_membership() {
    println!("Checking Membership");

    let languages: HashSet<&str> = HashSet::from(["Rust", "Python", "JavaScript", "Go", "C++"]);

    println!("Set: {:?}", languages);

    // contains() - check if value exists (O(1) average)
    // This is the most common operation on sets
    println!("\ncontains(\"Rust\"): {}", languages.contains("Rust"));
    println!("contains(\"Java\"): {}", languages.contains("Java"));

    // get() - get a reference to the value in the set
    // Useful when the set owns values and you need to borrow them
    match languages.get("Python") {
        Some(lang) => println!("\nget(\"Python\") found: {}", lang),
        None => println!("\nget(\"Python\") not found"),
    }

    match languages.get("Ruby") {
        Some(lang) => println!("get(\"Ruby\") found: {}", lang),
        None => println!("get(\"Ruby\") not found"),
    }

    // is_empty() and len()
    println!(
        "\nis_empty: {}, len: {}",
        languages.is_empty(),
        languages.len()
    );
}

/// Demonstrates set operations: union, intersection, difference, symmetric_difference.
///
/// These are the mathematical set operations that make sets so powerful.
/// All operations return iterators, so you need to collect() them.
pub fn set_operations() {
    println!("Set Operations");

    let set_a: HashSet<i8> = HashSet::from([1, 2, 3, 4, 5]);
    let set_b: HashSet<i8> = HashSet::from([4, 5, 6, 7, 8]);

    println!("Set A: {:?}", set_a);
    println!("Set B: {:?}", set_b);

    // UNION: All elements from both sets (A ∪ B)
    // Elements that appear in A OR B (or both)
    let union: HashSet<&i8> = set_a.union(&set_b).collect();
    println!("\nUnion (A ∪ B): {:?}", union);
    // {1, 2, 3, 4, 5} ∪ {4, 5, 6, 7, 8} = {1, 2, 3, 4, 5, 6, 7, 8}

    // INTERSECTION: Elements in both sets (A ∩ B)
    // Elements that appear in A AND B
    let intersection: HashSet<&i8> = set_a.intersection(&set_b).collect();
    println!("Intersection (A ∩ B): {:?}", intersection);
    // {1, 2, 3, 4, 5} ∩ {4, 5, 6, 7, 8} = {4, 5}

    // DIFFERENCE: Elements in A but not in B (A - B)
    // Elements unique to the first set
    let difference_a_b: HashSet<&i8> = set_a.difference(&set_b).collect();
    println!("Difference (A - B): {:?}", difference_a_b);
    // {1, 2, 3, 4, 5} - {4, 5, 6, 7, 8} = {1, 2, 3}

    let difference_b_a: HashSet<&i8> = set_b.difference(&set_a).collect();
    println!("Difference (B - A): {:?}", difference_b_a);
    // {4, 5, 6, 7, 8} - {1, 2, 3, 4, 5} = {6, 7, 8}

    // SYMMETRIC DIFFERENCE: Elements in either but not both (A △ B)
    // Elements unique to each set (union minus intersection)
    let sym_diff: HashSet<&i8> = set_a.symmetric_difference(&set_b).collect();
    println!("Symmetric Difference (A △ B): {:?}", sym_diff);
    // Elements in A or B but not both = {1, 2, 3, 6, 7, 8}
}

/// Demonstrates subset, superset, and disjoint checks.
///
/// These predicates answer questions about set relationships.
pub fn set_relationships() {
    println!("Set Relationships");

    let small: HashSet<i16> = HashSet::from([1, 2]);
    let medium: HashSet<i16> = HashSet::from([1, 2, 3, 4, 5]);
    let large: HashSet<i16> = HashSet::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    let disjoint: HashSet<i16> = HashSet::from([100, 200, 300]);

    println!("small: {:?}", small);
    println!("medium: {:?}", medium);
    println!("large: {:?}", large);
    println!("disjoint: {:?}", disjoint);

    // is_subset() - are all elements of self in other?
    println!("\nSubset checks:");
    println!("  small ⊆ medium: {}", small.is_subset(&medium));
    println!("  small ⊆ large: {}", small.is_subset(&large));
    println!("  medium ⊆ small: {}", medium.is_subset(&small));

    // is_superset() - does self contain all elements of other?
    println!("\nSuperset checks:");
    println!("  large ⊇ medium: {}", large.is_superset(&medium));
    println!("  large ⊇ small: {}", large.is_superset(&small));
    println!("  small ⊇ large: {}", small.is_superset(&large));

    // is_disjoint() - do the sets share NO elements?
    println!("\nDisjoint checks:");
    println!("  small ∩ disjoint = ∅: {}", small.is_disjoint(&disjoint));
    println!("  small ∩ medium = ∅: {}", small.is_disjoint(&medium));
}

/// Demonstrates iteration patterns for HashSet.
///
/// Important: HashSet iteration order is NOT guaranteed!
/// If you need ordered iteration, use BTreeSet.
pub fn iterating_hashsets() {
    println!("Iterating over HashSets");

    let colors: HashSet<&str> = HashSet::from(["red", "green", "blue", "yellow"]);

    // Basic iteration (order is NOT guaranteed!)
    println!("Basic iteration (order may vary):");
    for color in &colors {
        println!("  {}", color);
    }

    // iter() explicitly - same as &colors
    print!("\nUsing iter(): ");
    for color in colors.iter() {
        print!("{} ", color);
    }
    println!();

    // Consuming iteration - takes ownership
    let nums: HashSet<i8> = HashSet::from([1, 2, 3]);
    print!("\nConsuming (into_iter): ");
    for num in nums {
        // nums is moved here
        print!("{} ", num);
    }
    println!();
    // nums is no longer valid after into_iter()

    // Functional style with iterator adapters
    let numbers: HashSet<i8> = (1..=10).collect();
    let sum: i8 = numbers.iter().sum();
    let count_even: usize = numbers.iter().filter(|&x| x % 2 == 0).count();
    let squares: Vec<i8> = numbers.iter().map(|&x| x * x).collect();

    println!("\nFunctional operations on {:?}:", numbers);
    println!("  Sum: {}", sum);
    println!("  Even count: {}", count_even);
    println!("  Squares: {:?}", squares);
}

/// Demonstrates using custom types in HashSet.
///
/// To use a custom type, derive Hash, Eq, and PartialEq.
/// The derived Hash must be consistent with Eq:
/// if a == b, then hash(a) == hash(b).
pub fn custom_types_in_hashset() {
    println!("Custom Types in HashSet");

    // Derive the required traits for HashSet usage
    #[derive(Hash, Eq, PartialEq, Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    let mut visited: HashSet<Point> = HashSet::new();

    visited.insert(Point { x: 0, y: 0 });
    visited.insert(Point { x: 1, y: 0 });
    visited.insert(Point { x: 0, y: 1 });
    visited.insert(Point { x: 0, y: 0 }); // Duplicate - ignored!

    println!("Visited points: {:?}", visited);
    println!("Count: {} (duplicate was ignored)", visited.len());

    // Check membership with a new Point instance
    // This works because Hash and Eq are based on field values
    let query: Point = Point { x: 0, y: 0 };
    println!(
        "\nAlready visited {:?}? {}",
        query,
        visited.contains(&query)
    );

    let query2: Point = Point { x: 5, y: 5 };
    println!(
        "Already visited {:?}? {}",
        query2,
        visited.contains(&query2)
    );

    // Custom type with enum
    #[derive(Hash, Eq, PartialEq, Debug, Clone)]
    enum Permission {
        Read,
        Write,
        Delete,
        Admin,
    }

    let user_permissions: HashSet<Permission> =
        HashSet::from([Permission::Read, Permission::Write]);

    let admin_permissions: HashSet<Permission> = HashSet::from([
        Permission::Read,
        Permission::Write,
        Permission::Delete,
        Permission::Admin,
    ]);

    println!("\nUser permissions: {:?}", user_permissions);
    println!("Admin permissions: {:?}", admin_permissions);
    println!(
        "User is subset of admin: {}",
        user_permissions.is_subset(&admin_permissions)
    );
}

// =============================================================================
// BTREESET SECTION
// =============================================================================

/// Demonstrates creating BTreeSet - the sorted set.
///
/// BTreeSet keeps elements in sorted order and supports range queries.
/// Elements must implement Ord (not Hash).
pub fn creating_btreesets() {
    println!("Creating BTreeSets");

    // Method 1: BTreeSet::new()
    let mut set: BTreeSet<i8> = BTreeSet::new();
    set.insert(5);
    set.insert(1);
    set.insert(9);
    set.insert(3);
    println!("Created with new(): {:?}", set);
    println!("Notice: Elements are automatically sorted!");

    // Method 2: BTreeSet::from() - from array
    let set2: BTreeSet<&str> = BTreeSet::from(["cherry", "apple", "banana"]);
    println!("\nFrom array: {:?}", set2);
    println!("Notice: Strings sorted alphabetically!");

    // Method 3: collect() from iterator
    let set3: BTreeSet<i8> = (1..=5).collect();
    println!("\nCollected from range: {:?}", set3);

    // Method 4: From unsorted data - automatically sorts
    let unsorted: Vec<i8> = vec![7, 2, 9, 1, 5, 3, 8, 4, 6];
    let sorted_set: BTreeSet<i8> = unsorted.into_iter().collect();
    println!("\nFrom unsorted vec: {:?}", sorted_set);

    // Note: Unlike HashSet, BTreeSet has no with_capacity() method.
    // B-trees allocate nodes as needed.
}

/// Demonstrates BTreeSet's feature: sorted iteration.
///
/// Unlike HashSet, BTreeSet always iterates in sorted order.
/// This is guaranteed and deterministic.
pub fn btreeset_sorted_iteration() {
    println!("BTreeSet Sorted Iteration");

    let mut numbers: BTreeSet<i8> = BTreeSet::new();

    // Insert in random order
    for n in [5, 2, 8, 1, 9, 3, 7, 4, 6] {
        numbers.insert(n);
    }

    // Iteration is ALWAYS in sorted order
    println!("Inserted in random order, iterating in sorted order:");
    for num in &numbers {
        print!("{} ", num);
    }
    println!();

    // Reverse iteration is also efficient
    println!("\nReverse iteration:");
    for num in numbers.iter().rev() {
        print!("{} ", num);
    }
    println!();

    // Compare with HashSet - order is arbitrary
    let hash_numbers: HashSet<i8> = [5, 2, 8, 1, 9, 3, 7, 4, 6].into_iter().collect();
    println!("\nHashSet iteration (arbitrary order):");
    for num in &hash_numbers {
        print!("{} ", num);
    }
    println!();
}

/// Demonstrates range queries - BTreeSet's other feature.
///
/// Because elements are sorted, BTreeSet can efficiently answer
/// "give me all elements between X and Y".
pub fn btreeset_range_queries() {
    println!("BTreeSet Range Queries");

    let numbers: BTreeSet<i8> = (1..=20).collect();
    println!("Full set: {:?}", numbers);

    // range() with inclusive bounds: 5..=10 means 5 <= x <= 10
    println!("\nrange(5..=10) - elements from 5 to 10 inclusive:");
    let range1: Vec<&i8> = numbers.range(5..=10).collect();
    println!("  {:?}", range1);

    // range() with exclusive end: 5..10 means 5 <= x < 10
    println!("\nrange(5..10) - elements from 5 to 10 exclusive:");
    let range2: Vec<&i8> = numbers.range(5..10).collect();
    println!("  {:?}", range2);

    // Unbounded start: ..8 means x < 8
    println!("\nrange(..8) - elements less than 8:");
    let range3: Vec<&i8> = numbers.range(..8).collect();
    println!("  {:?}", range3);

    // Unbounded end: 15.. means x >= 15
    println!("\nrange(15..) - elements 15 and greater:");
    let range4: Vec<&i8> = numbers.range(15..).collect();
    println!("  {:?}", range4);

    // Practical example: find all values in a score range
    let scores: BTreeSet<i8> = BTreeSet::from([65, 72, 78, 81, 85, 88, 92, 95, 98]);
    println!("\nScores: {:?}", scores);
    println!("B grades (80-89):");
    for score in scores.range(80..90) {
        println!("  {}", score);
    }
}

/// Demonstrates first/last element access in BTreeSet.
///
/// Because elements are sorted, finding min (first) and max (last)
/// is O(log n) - just traverse to the appropriate leaf.
pub fn btreeset_min_max() {
    println!("BTreeSet Min/Max operations");

    let mut numbers: BTreeSet<i8> = BTreeSet::from([5, 2, 8, 1, 9, 3, 7]);

    println!("Set: {:?}", numbers);

    // first() and last() - peek without removing
    println!("\nfirst() (minimum): {:?}", numbers.first());
    println!("last() (maximum): {:?}", numbers.last());

    // pop_first() and pop_last() - remove and return
    let min: Option<i8> = numbers.pop_first();
    println!("\npop_first() returned: {:?}", min);
    println!("Set after pop_first: {:?}", numbers);

    let max: Option<i8> = numbers.pop_last();
    println!("\npop_last() returned: {:?}", max);
    println!("Set after pop_last: {:?}", numbers);
}

/// Demonstrates that BTreeSet supports all the same set operations as HashSet.
///
/// The results are also sorted!
pub fn btreeset_set_operations() {
    println!("BTreeSet set operations");

    let set_a: BTreeSet<i8> = BTreeSet::from([1, 2, 3, 4, 5]);
    let set_b: BTreeSet<i8> = BTreeSet::from([4, 5, 6, 7, 8]);

    println!("Set A: {:?}", set_a);
    println!("Set B: {:?}", set_b);

    // All set operations work the same, but results are sorted!
    let union: BTreeSet<&i8> = set_a.union(&set_b).collect();
    println!("\nUnion (sorted): {:?}", union);

    let intersection: BTreeSet<&i8> = set_a.intersection(&set_b).collect();
    println!("Intersection (sorted): {:?}", intersection);

    let difference: BTreeSet<&i8> = set_a.difference(&set_b).collect();
    println!("Difference A-B (sorted): {:?}", difference);

    let sym_diff: BTreeSet<&i8> = set_a.symmetric_difference(&set_b).collect();
    println!("Symmetric Difference (sorted): {:?}", sym_diff);

    // Relationship checks work the same too
    let subset: BTreeSet<i8> = BTreeSet::from([2, 3, 4]);
    println!("\nsubset {:?} ⊆ A: {}", subset, subset.is_subset(&set_a));
}

/// Practical example: Deduplication with order preservation options.
pub fn practical_deduplication() {
    println!("Practical example: deduplication");

    let emails: Vec<&str> = vec![
        "alice@example.com",
        "bob@example.com",
        "alice@example.com", // Duplicate
        "charlie@example.com",
        "bob@example.com",   // Duplicate
        "alice@example.com", // Another duplicate
    ];

    println!("Original list ({} items):", emails.len());
    for email in &emails {
        println!("  {}", email);
    }

    // Method 1: HashSet - fast dedup, arbitrary order
    let unique_hash: HashSet<&str> = emails.iter().cloned().collect();
    println!(
        "\nHashSet dedup ({} items, arbitrary order):",
        unique_hash.len()
    );
    for email in &unique_hash {
        println!("  {}", email);
    }

    // Method 2: BTreeSet - dedup with sorted order
    let unique_btree: BTreeSet<&str> = emails.iter().cloned().collect();
    println!("\nBTreeSet dedup ({} items, sorted):", unique_btree.len());
    for email in &unique_btree {
        println!("  {}", email);
    }

    // Method 3: Preserve original insertion order (using Vec + HashSet)
    let mut seen: HashSet<&str> = HashSet::new();
    let unique_ordered: Vec<&str> = emails
        .iter()
        .cloned()
        .filter(|email| seen.insert(email))
        .collect();
    println!("\nInsertion-order dedup ({} items):", unique_ordered.len());
    for email in &unique_ordered {
        println!("  {}", email);
    }
}

/// Practical example: Finding duplicates in a collection.
pub fn practical_finding_duplicates() {
    println!("Practical example: finding duplicates");

    let items: Vec<&str> = vec![
        "apple", "banana", "apple", "cherry", "banana", "date", "apple",
    ];

    println!("Items: {:?}", items);

    // Find which items appear more than once
    let mut seen: HashSet<&str> = HashSet::new();
    let mut duplicates: HashSet<&str> = HashSet::new();

    for item in &items {
        // insert() returns false if already present
        if !seen.insert(item) {
            duplicates.insert(item);
        }
    }

    println!("Duplicate items: {:?}", duplicates);

    // Find the first duplicate
    fn find_first_duplicate<'a>(items: &[&'a str]) -> Option<&'a str> {
        let mut seen: HashSet<&str> = HashSet::new();
        for &item in items {
            if !seen.insert(item) {
                return Some(item);
            }
        }
        None
    }

    match find_first_duplicate(&items) {
        Some(dup) => println!("First duplicate: {}", dup),
        None => println!("No duplicates found"),
    }
}

/// Practical example: Comparing two lists to find common/different elements.
pub fn practical_comparing_lists() {
    println!("Practical example: comparing lists");

    let shopping_list_1: HashSet<&str> =
        HashSet::from(["milk", "bread", "eggs", "butter", "cheese"]);
    let shopping_list_2: HashSet<&str> =
        HashSet::from(["bread", "cheese", "apples", "oranges", "milk"]);

    println!("Shopping List 1: {:?}", shopping_list_1);
    println!("Shopping List 2: {:?}", shopping_list_2);

    // Items in both lists
    let common: HashSet<_> = shopping_list_1.intersection(&shopping_list_2).collect();
    println!("\nItems in BOTH lists: {:?}", common);

    // Items only in list 1
    let only_in_1: HashSet<_> = shopping_list_1.difference(&shopping_list_2).collect();
    println!("Only in list 1: {:?}", only_in_1);

    // Items only in list 2
    let only_in_2: HashSet<_> = shopping_list_2.difference(&shopping_list_1).collect();
    println!("Only in list 2: {:?}", only_in_2);

    // Combined unique items (everything from both)
    let all_items: HashSet<_> = shopping_list_1.union(&shopping_list_2).collect();
    println!("All unique items: {:?}", all_items);
}

/// Practical example: Tag system using sets.
pub fn practical_tag_system() {
    println!("Practical example: tag system");

    // Articles with their tags
    struct Article {
        title: String,
        tags: HashSet<String>,
    }

    let articles: Vec<Article> = vec![
        Article {
            title: "Intro to Rust".to_string(),
            tags: HashSet::from([
                "rust".to_string(),
                "programming".to_string(),
                "beginner".to_string(),
            ]),
        },
        Article {
            title: "Advanced Rust Patterns".to_string(),
            tags: HashSet::from([
                "rust".to_string(),
                "programming".to_string(),
                "advanced".to_string(),
            ]),
        },
        Article {
            title: "Web Development with Rust".to_string(),
            tags: HashSet::from([
                "rust".to_string(),
                "web".to_string(),
                "programming".to_string(),
            ]),
        },
        Article {
            title: "Python for Data Science".to_string(),
            tags: HashSet::from([
                "python".to_string(),
                "data-science".to_string(),
                "programming".to_string(),
            ]),
        },
    ];

    // Find articles with specific tags
    let search_tags: HashSet<String> = HashSet::from(["rust".to_string(), "beginner".to_string()]);

    println!("Searching for articles with tags: {:?}\n", search_tags);

    // Articles that have ALL the search tags
    println!("Articles with ALL search tags:");
    for article in &articles {
        if search_tags.is_subset(&article.tags) {
            println!("  - {} (tags: {:?})", article.title, article.tags);
        }
    }

    // Articles that have ANY of the search tags
    println!("\nArticles with ANY search tags:");
    for article in &articles {
        if !search_tags.is_disjoint(&article.tags) {
            println!("  - {} (tags: {:?})", article.title, article.tags);
        }
    }

    // Find all unique tags across all articles
    let all_tags: BTreeSet<&String> = articles.iter().flat_map(|a| a.tags.iter()).collect();
    println!("\nAll tags (sorted): {:?}", all_tags);
}

/// Practical example: Using BTreeSet for a leaderboard with rankings.
pub fn practical_leaderboard() {
    println!("Practical example: leaderboard");

    // For a leaderboard, we want scores sorted descending
    // BTreeSet sorts ascending, so we use Reverse
    use std::cmp::Reverse;

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
    struct Player {
        score: Reverse<i16>, // Reverse for descending order
        name: String,
    }

    let mut leaderboard: BTreeSet<Player> = BTreeSet::new();

    leaderboard.insert(Player {
        score: Reverse(1_500),
        name: "Alice".to_string(),
    });
    leaderboard.insert(Player {
        score: Reverse(1_200),
        name: "Bob".to_string(),
    });
    leaderboard.insert(Player {
        score: Reverse(1_800),
        name: "Charlie".to_string(),
    });
    leaderboard.insert(Player {
        score: Reverse(1_350),
        name: "Diana".to_string(),
    });

    println!("Leaderboard (sorted by score descending):");
    for (rank, player) in leaderboard.iter().enumerate() {
        println!(
            "  {}. {} - {} points",
            rank + 1,
            player.name,
            player.score.0
        );
    }

    // Top 3 players
    println!("\nTop 3:");
    for player in leaderboard.iter().take(3) {
        println!("  {} - {} points", player.name, player.score.0);
    }
}
