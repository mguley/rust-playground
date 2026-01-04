/// Demonstrates basic Vec creation patterns
pub fn basic_vec_operations() {
    // Method 1: Using the vec! macro (most common)
    let numbers: Vec<i8> = vec![1, 2, 3, 4, 5];
    println!("Created with macro: {:?}", numbers);

    // Method 2: Using Vec::new() and push
    let mut fruits: Vec<&str> = Vec::new();
    fruits.push("apple");
    fruits.push("banana");
    fruits.push("cherry");
    println!("Created with new(): {:?}", fruits);

    // Method 3: With pre-allocated capacity (important for performance!)
    // This avoids reallocations when you know the approximate size
    let mut with_capacity: Vec<i8> = Vec::with_capacity(100);
    println!("Capacity before pushes: {}", with_capacity.capacity());

    for i in 0..50 {
        with_capacity.push(i);
    }
    // Still has capacity 100, no reallocation occurred
    println!("Capacity after 50 pushes: {}", with_capacity.capacity());

    // Method 4: From an iterator
    let squares: Vec<i8> = (1..=5).map(|x| x * x).collect();
    println!("Squares from iterator: {:?}", squares);
}

/// Demonstrates accessing elements safely
pub fn accessing_elements() {
    let colors: Vec<&str> = vec!["red", "green", "blue"];

    // Safe access with get() - returns Option<&T>
    // This is the recommended approach when the index might be out of bounds
    match colors.get(1) {
        Some(color) => println!("Color at index 1: {}", color),
        None => println!("No color at that index"),
    }

    // Direct indexing - panics if out of bounds!
    // Only use this when you're certain the index is valid
    let first: &str = colors[0];
    println!("First color (direct access): {}", first);

    // Safe access to first and last elements
    if let Some(first) = colors.first() {
        println!("First: {}", first);
    }
    if let Some(last) = colors.last() {
        println!("Last: {}", last);
    }
}

/// Demonstrates modifying vectors
pub fn modifying_vectors() {
    let mut nums: Vec<i8> = vec![1, 2, 3];

    // Adding elements
    nums.push(4); // Add to end - O(1) amortized
    nums.insert(0, 0); // Insert at index - O(n) because elements shift
    println!("After push and insert: {:?}", nums);

    // Removing elements
    let last: Option<i8> = nums.pop(); // Remove from end - O(1)
    println!("Popped: {:?}", last);

    let removed: i8 = nums.remove(1); // Remove at index - O(n) because elements shift
    println!("Removed at index 1: {}", removed);
    println!("After removals: {:?}", nums);

    // Extending with another collection
    let more_nums: Vec<i8> = vec![10, 20, 30];
    nums.extend(more_nums);
    println!("After extend: {:?}", nums);

    // Retain only elements matching a condition
    nums.retain(|&x| x < 15);
    println!("After retain (x < 15): {:?}", nums);
}

/// Demonstrates slicing - borrowing parts of a vector
pub fn slicing_vectors() {
    let numbers: Vec<i8> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    // Slices borrow a portion of the vector
    let slice: &[i8] = &numbers[2..5]; // Elements at indices 2, 3, 4
    println!("Slice [2..5]: {:?}", slice);

    let from_start: &[i8] = &numbers[..3]; // First 3 elements
    println!("Slice [..3]: {:?}", from_start);

    let to_end: &[i8] = &numbers[7..]; // From index 7 to end
    println!("Slice [7..]: {:?}", to_end);

    // You can iterate over slices
    for (i, num) in numbers[3..6].iter().enumerate() {
        println!("  Index {}: {}", i, num);
    }
}

/// Demonstrates Vec's capacity behavior
pub fn capacity_demonstration() {
    let mut v: Vec<i32> = Vec::new();

    println!("Empty vec - len: {}, capacity: {}", v.len(), v.capacity());

    // Watch how capacity grows as we add elements
    for i in 0..20 {
        v.push(i);
        println!(
            "After push {} - len: {}, capacity: {}",
            i,
            v.len(),
            v.capacity()
        );
    }

    // Capacity grows roughly by doubling (implementation detail, may vary)
    // This amortizes the cost of reallocation over many operations

    // Shrink to fit current contents
    v.shrink_to_fit();
    println!(
        "After shrink_to_fit - len: {}, capacity: {}",
        v.len(),
        v.capacity()
    );
}
