# Common Collections in Rust

## Table of Contents
- [Introduction](#introduction)
- [What are collections?](#what-are-collections)
- [Prerequisites](#prerequisites)
- [Step 1: Setting up our environment](#step-1-setting-up-our-environment)
- [Step 2: Vec - The Dynamic Array](#step-2-vec---the-dynamic-array)
- [Step 3: VecDeque - The Double-Ended Queue](#step-3-vecdeque---the-double-ended-queue)
- [Step 4: LinkedList - The Doubly-Linked List](#step-4-linkedlist---the-doubly-linked-list)
- [Step 5: HashMap - The Hash Table](#step-5-hashmap---the-hash-table)
- TODO

---

#### Introduction

Every programming language provides tools to store and organize data. In Rust, the standard library offers a rich set of
collection types that cover virtually every use case you'll encounter in real-world applications.

For developers coming from Go, you'll notice some familiar concepts (slices become `Vec`, maps become `HashMap`) but also
discover new powerful structures like `BTreeMap` for ordered data and `BinaryHeap` for priority queues.

Understanding which collection to use - and why - can significantly impact your application's performance:

- A web server might handle 10x more requests by choosing the right map implementation
- A search engine could reduce query latency from seconds to milliseconds with proper data structures
- A game engine might achieve consistent frame rates by avoiding unexpected allocations

In this deep dive, we'll explore all eight primary collection types in Rust's standard library, understand their
internal workings, and learn when to use each one.

---

#### What are collections?

Rust's standard collection library provides efficient implementations of the most common general-purpose data structures.
These collections are grouped into four major categories:

**Sequences** store ordered elements:
- `Vec` - A contiguous growable array (like Go's slice)
- `VecDeque` - A double-ended queue with O(1) operations at both ends
- `LinkedList` - A doubly-linked list for efficient splitting and appending

**Maps** associate keys with values:
- `HashMap` - Fast key-value lookups using hashing (like Go's map)
- `BTreeMap` - Ordered key-value storage using a B-Tree

**Sets** store unique elements:
- `HashSet` - Fast membership testing using hashing
- `BTreeSet` - Ordered unique elements using a B-Tree

**Miscellaneous**:
- `BinaryHeap` - A priority queue implemented as a max-heap

The Rust documentation offers this practical advice: *"You should probably just use `Vec` or `HashMap`."*
These two collections cover most use cases. The other collections have specific scenarios where they excel,
but those cases are comparatively niche.

---

#### Prerequisites

Before we begin, you'll need:
- Rust installed (version 1.85+, we tested with 1.92)
- Basic knowledge of Rust syntax (variables, functions, basic types)
- A code editor of your choice
- Terminal/command-line access

If you're new to Rust, we recommend reading the first few chapters of
[The Rust Programming Language](https://doc.rust-lang.org/book/) book first.

---

#### Step 1: Setting up our environment

Let's create a directory structure for our collections experiments:

```bash
mkdir -p rust-collections
cd rust-collections
cargo init --name collections_demo
```

This creates a new Cargo project. Now let's set up our `src/main.rs` with a basic structure:

```rust
// src/main.rs
use rustc_version_runtime;

fn main() {
    println!("Rust Collections Demo");
    println!("Compiled with: {:?}", rustc_version_runtime::version());

    // We'll call our example functions here
}
```

For our examples, we'll also want to enable some useful Cargo features. Update your `Cargo.toml`:

```toml
[package]
name = "collections_demo"
version = "0.1.0"
edition = "2024"

[dependencies]
rustc_version_runtime = "0.3"

[dev-dependencies]
criterion = "0.8.1"
```

To run this:
```bash
cargo run
```
---

#### Step 2: Vec - the dynamic array

`Vec<T>` is Rust's workhorse collection - a contiguous, growable array type. If you're coming from Go,
think of it as a slice with explicit ownership semantics.

#### Creating and using Vectors

Create a file `src/vec_examples.rs`:

```rust
// src/vec_examples.rs

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
```

#### Understanding Vec's memory layout

A `Vec<T>` consists of three components:

1. A pointer to heap-allocated memory; points to the first element on the heap
2. The length (number of elements currently stored)
3. The capacity (total allocated space)

When you push beyond capacity, Vec allocates a new, larger buffer, copies elements over, and frees the old buffer.
This is why push is `O(1) amortized` - usually O(1), occasionally O(n) for reallocation.

```
   Stack                          Heap
   ┌─────────────┐               ┌───┬───┬───┬───┬───┬───┬───┬───┐
   │ ptr ────────┼──────────────►│ 1 │ 2 │ 3 │ 4 │ 5 │   │   │   │
   │ len: 5      │               └───┴───┴───┴───┴───┴───┴───┴───┘
   │ capacity: 8 │                 ▲               ▲               ▲
   └─────────────┘                 │               │               │
                              first element   len boundary   capacity boundary
```

Add to file `src/vec_examples.rs`:

```rust
// src/vec_examples.rs

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
```

Update `src/main.rs`:

```rust
mod vec_examples;
use vec_examples::{
    accessing_elements, basic_vec_operations, capacity_demonstration, modifying_vectors,
    slicing_vectors,
};

use rustc_version_runtime;

fn main() {
    println!("Rust Collections Demo");
    println!("Compiled with: {:?}", rustc_version_runtime::version());

    // We'll call our example functions here
    run_vec_examples();
}

fn run_vec_examples() {
    section("basic_vec_operations()", basic_vec_operations);
    section("accessing_elements()", accessing_elements);
    section("modifying_vectors()", modifying_vectors);
    section("slicing_vectors()", slicing_vectors);
    section("capacity_demonstration()", capacity_demonstration);
}

fn section(title: &str, function: impl FnOnce()) {
    println!("\n{:=^70}", format!(" {} ", title));
    function();
    println!("{:=^70}\n", "");
}
```

Run all the examples:
```bash
cargo run
```

#### Key takeaways for Vec

| Operation | Time Complexity | Notes |
|-----------|-----------------|-------|
| `push` | O(1) amortized | May reallocate |
| `pop` | O(1) | Returns `Option<T>` |
| `insert(i, x)` | O(n-i) | Shifts elements right |
| `remove(i)` | O(n-i) | Shifts elements left |
| `get(i)` | O(1) | Returns `Option<&T>` |
| `[i]` | O(1) | Panics if out of bounds |

**When to use `Vec`**:
- You need a resizable array
- You primarily add/remove from the end
- You need fast random access by index
- You want to use it as a stack (push/pop)

---

#### Step 3: VecDeque - the double-ended queue

`VecDeque<T>` is a double-ended queue implemented with a growable ring buffer. Unlike `Vec`,
it provides O(1) operations at *both* ends.

```
VecDeque uses a ring buffer internally. Imagine elements arranged in a circle:

  Regular Vec:         VecDeque (ring buffer):
  [a][b][c][d][_]      ╭───────────────╮
   ↑           ↑       │    [c][d]     │
  start       end      │ [b]     [_]   │
                       │ [a]     [_]   │
                       │    [_][_]     │
                       ╰───────────────╯
                        head → ... → tail

Key insight: The head and tail can wrap around, so pushing to the front
doesn't require shifting all elements like Vec does.

Vec<T>      push_front: O(n)    push_back: O(1)    index: O(1)
VecDeque<T> push_front: O(1)*   push_back: O(1)*   index: O(1)
            * amortized - occasionally needs to grow the buffer
```

Create `src/vecdeque_examples.rs`:

```rust
// src/vecdeque_examples.rs
use std::collections::VecDeque;

/// Demonstrates basic VecDeque operations
pub fn basic_vecdeque_operations() {
    // Create a new VecDeque
    let mut deque: VecDeque<i8> = VecDeque::new();

    // Add elements to the back (like Vec::push)
    deque.push_back(1);
    deque.push_back(2);
    deque.push_back(3);
    println!("After push_back 1, 2, 3: {:?}", deque);

    // Add elements to the front - this is O(1)!
    // With Vec, this would be O(n) because all elements shift
    deque.push_front(0);
    deque.push_front(-1);
    println!("After push_front 0, -1: {:?}", deque);

    // Remove from front - O(1)
    let front: Option<i8> = deque.pop_front();
    println!("Popped front: {:?}", front);

    // Remove from back - O(1)
    let back: Option<i8> = deque.pop_back();
    println!("Popped back: {:?}", back);

    println!("Final state: {:?}", deque);
}

/// Demonstrates using VecDeque as a queue (FIFO)
pub fn fifo_queue_example() {
    println!("\n--- FIFO Queue Example ---");

    let mut queue: VecDeque<String> = VecDeque::new();

    // Simulate a print queue
    queue.push_back("Document 1".to_string());
    queue.push_back("Document 2".to_string());
    queue.push_back("Document 3".to_string());

    println!("Print queue: {:?}", queue);

    // Process items in order (first in, first out)
    while let Some(doc) = queue.pop_front() {
        println!("Printing: {}", doc);
    }
}

/// Demonstrates using VecDeque for sliding window operations
pub fn sliding_window_example() {
    println!("\n--- Sliding Window Example ---");

    // Calculate moving average of last 3 values
    let data: Vec<i8> = vec![1, 3, 5, 7, 9, 11, 13, 15];
    let window_size: usize = 3;

    let mut window: VecDeque<i8> = VecDeque::with_capacity(window_size);
    let mut averages: Vec<f64> = Vec::new();

    for value in data {
        window.push_back(value);

        // If window is full, calculate average and remove oldest
        if window.len() == window_size {
            let avg: f64 = window.iter().sum::<i8>() as f64 / window_size as f64;
            averages.push(avg);
            window.pop_front();
        }
    }

    println!(
        "Moving averages (window size {}): {:?}",
        window_size, averages
    );
}

/// Demonstrates VecDeque's ring buffer behavior
pub fn ring_buffer_demonstration() {
    println!("\n--- Ring Buffer Demonstration ---");

    // VecDeque uses a circular buffer internally
    // This means push_front doesn't actually move elements

    let mut deque: VecDeque<i8> = VecDeque::with_capacity(5);

    // Fill the deque
    for i in 1..=5 {
        deque.push_back(i);
    }
    println!("Initial: {:?}", deque);

    // Pop from front and push to back - elements "rotate"
    // but no actual memory movement occurs
    for _ in 0..3 {
        if let Some(front) = deque.pop_front() {
            deque.push_back(front + 10);
        }
    }
    println!("After rotation: {:?}", deque);
}
```

Update `src/main.rs`:

```rust
// ...
mod vecdeque_examples;
use vecdeque_examples::{
    basic_vecdeque_operations, fifo_queue_example, ring_buffer_demonstration,
    sliding_window_example,
};
// ...

fn main() {
    // We'll call our example functions here
    // run_vec_examples();

    run_vecdeque_examples();
}

fn run_vecdeque_examples() {
    section("basic_vecdeque_operations", basic_vecdeque_operations);
    section("fifo_queue_example", fifo_queue_example);
    section("sliding_window_example", sliding_window_example);
    section("ring_buffer_demonstration", ring_buffer_demonstration);
}
```

Run all the examples:
```bash
cargo run
```

#### Key takeaways for VecDeque

| Operation | Time Complexity | Notes |
|-----------|-----------------|-------|
| `push_front` | O(1) amortized | Key advantage over Vec |
| `push_back` | O(1) amortized | Same as Vec |
| `pop_front` | O(1) | Returns `Option<T>` |
| `pop_back` | O(1) | Returns `Option<T>` |
| `get(i)` | O(1) | Random access works |

**When to use `VecDeque`**:
- You need efficient insertion/removal at both ends
- You're implementing a queue (FIFO) or deque
- You need a sliding window over data
- You're doing breadth-first search (BFS)

---

#### Step 4: LinkedList - the doubly-linked list

`LinkedList<T>` is a doubly-linked list. Each element is stored separately in memory with pointers
to the previous and next elements. This is the least commonly used sequence type in Rust.

Create `src/linked_list_examples.rs`:

```rust
// src/linked_list_examples
use std::collections::{LinkedList, VecDeque};
use std::time::{Duration, Instant};

/// Demonstrates basic LinkedList operations
pub fn basic_linked_list_operations() {
    let mut list: LinkedList<i8> = LinkedList::new();

    // Add elements to front and back - O(1)
    list.push_back(2);
    list.push_back(3);
    list.push_front(1);
    list.push_front(0);
    println!("List after pushes: {:?}", list);

    // Remove from front and back - O(1)
    let front: Option<i8> = list.pop_front();
    let back: Option<i8> = list.pop_back();
    println!("Popped front: {:?}, back: {:?}", front, back);
    println!("After pops: {:?}", list);

    // Peek without removing
    if let Some(first) = list.front() {
        println!("First element: {}", first);
    }
    if let Some(last) = list.back() {
        println!("Last element: {}", last);
    }

    // Mutable peek
    if let Some(first) = list.front_mut() {
        *first *= 10;
    }
    println!("After modifying front: {:?}", list);
}

/// Demonstrates LinkedList's strength: O(1) append and split
pub fn append_and_split() {
    println!("\n--- Append and Split Example ---");

    let mut list1: LinkedList<i8> = LinkedList::new();
    let mut list2: LinkedList<i8> = LinkedList::new();

    list1.extend([1, 2, 3]);
    list2.extend([4, 5, 6]);

    println!("List 1: {:?}", list1);
    println!("List 2: {:?}", list2);

    // Append is O(1) - just relink pointers!
    // This is where LinkedList shines compared to Vec
    list1.append(&mut list2);

    println!("After append:");
    println!("List 1: {:?}", list1);
    println!("List 2 (now empty): {:?}", list2);

    // Split also just relinks pointers
    let mut original: LinkedList<i8> = (0..10).collect();
    println!("\nOriginal: {:?}", original);

    // Split at index 5
    // Note: Finding position 5 is O(n), but the split itself is O(1)
    let second_half: LinkedList<i8> = original.split_off(5);
    println!("After split_off(5):");
    println!("First half: {:?}", original);
    println!("Second half: {:?}", second_half);
}

/// Demonstrates iteration (works like other collections)
pub fn linked_list_iteration() {
    println!("\n--- Iteration Example ---");
    let list: LinkedList<&str> = ["apple", "banana", "cherry"].into_iter().collect();

    // Immutable iteration
    print!("Forward: ");
    for item in &list {
        print!("{} ", item);
    }
    println!();

    // Reverse iteration (LinkedList supports this efficiently)
    print!("Backward: ");
    for item in list.iter().rev() {
        print!("{} ", item);
    }
    println!();

    // Mutable iteration
    let mut numbers: LinkedList<i8> = (1..=5).collect();
    for item in &mut numbers {
        *item *= 2;
    }
    println!("Doubled: {:?}", numbers);
}

/// Demonstrates cursor-based mutation
pub fn cursor_example() {
    println!("\n--- Understanding LinkedList Limitations ---");

    // LinkedList doesn't support random access
    // You can't do list[3] like with Vec

    let list: LinkedList<i8> = (1..=5).collect();

    // To access the nth element, you must iterate
    if let Some(third) = list.iter().nth(2) {
        println!("Third element (via iteration): {}", third);
    }

    // This is O(n), not O(1)!
    // For most use cases, VecDeque is better
}

/// Demonstrates a comparison of LinkedList with VecDeque
pub fn compare_linked_list() {
    println!("\n--- LinkedList vs VecDeque ---");
    println!("For double-ended operations, VecDeque is usually better:\n");

    // Both support O(1) push/pop at both ends
    let mut deque: VecDeque<i32> = VecDeque::new();
    let mut linked: LinkedList<i32> = LinkedList::new();

    // Timing a simple benchmark
    let iterations: i32 = 10_000;

    let start: Instant = Instant::now();
    for i in 0..iterations {
        deque.push_back(i);
        deque.push_front(i);
    }
    let deque_time: Duration = start.elapsed();

    let start: Instant = Instant::now();
    for i in 0..iterations {
        linked.push_back(i);
        linked.push_front(i);
    }
    let linked_time: Duration = start.elapsed();

    println!("Push {} elements to both ends:", iterations);
    println!("VecDeque: {:?}", deque_time);
    println!("LinkedList: {:?}", linked_time);

    // Clear and test iteration
    deque.clear();
    linked.clear();

    for i in 0..iterations {
        deque.push_back(i);
        linked.push_back(i);
    }

    let start: Instant = Instant::now();
    let _sum: i32 = deque.iter().sum();
    let deque_iter_time: Duration = start.elapsed();

    let start: Instant = Instant::now();
    let _sum: i32 = linked.iter().sum();
    let linked_iter_time: Duration = start.elapsed();

    println!("\nIterate through {} elements:", iterations);
    println!("VecDeque: {:?}", deque_iter_time);
    println!("LinkedList: {:?}", linked_iter_time);

    println!("\nVecDeque should win on iteration due to cache locality!");
}
```

Update `src/main.rs`:
```rust
// ...
mod linked_list_examples;
// ...
use linked_list_examples::{
    append_and_split, basic_linked_list_operations, compare_linked_list, cursor_example,
    linked_list_iteration,
};

fn main() {
    //...
    run_linked_list_examples();
}

fn run_linked_list_examples() {
    section("basic_linked_list_operations", basic_linked_list_operations);
    section("append_and_split", append_and_split);
    section("linked_list_iteration", linked_list_iteration);
    section("cursor_example", cursor_example);
    section("compare_linked_list", compare_linked_list);
}
```

Run all the examples:
```bash
cargo run
```

The truth about LinkedList

```
 Why Vec<T> usually beats LinkedList<T>:

 Memory Layout:

   Vec<T> (contiguous memory - cache friendly):
   ┌───┬───┬───┬───┬───┬───┬───┬───┐
   │ 1 │ 2 │ 3 │ 4 │ 5 │ 6 │ 7 │ 8 │  ← All elements in one block
   └───┴───┴───┴───┴───┴───┴───┴───┘

   LinkedList<T> (scattered memory - cache unfriendly):
   ┌───┐     ┌───┐     ┌───┐     ┌───┐
   │ 1 │────▶│ 2 │────▶│ 3 │────▶│ 4 │  ← Each node somewhere in memory
   └───┘     └───┘     └───┘     └───┘
     │         │         │         │
     └─────────┴─────────┴─────────┘
           Pointers everywhere!

 Modern CPUs are optimized for sequential memory access. When you iterate
 through a Vec, the CPU prefetches the next elements. With LinkedList,
 each element could be anywhere in memory, causing cache misses.

 Performance comparison:

 ┌─────────────────────────────┬─────────────┬──────────────┐
 │ Operation                   │ Vec<T>      │ LinkedList   │
 ├─────────────────────────────┼─────────────┼──────────────┤
 │ Push/pop back               │ O(1)*       │ O(1)         │
 │ Push/pop front              │ O(n)        │ O(1)         │ ← LL wins!
 │ Insert in middle            │ O(n)        │ O(1)**       │ ← LL wins!
 │ Remove from middle          │ O(n)        │ O(1)**       │ ← LL wins!
 │ Random access by index      │ O(1)        │ O(n)         │ ← Vec wins!
 │ Iteration (cache behavior)  │ FAST        │ SLOW         │ ← Vec wins!
 │ Memory per element          │ Just T      │ T + 2 ptrs   │ ← Vec wins!
 │ Allocation per element      │ Amortized   │ Every time   │ ← Vec wins!
 └─────────────────────────────┴─────────────┴──────────────┘

 * Vec push_back is amortized O(1)
 ** Only if you already have a cursor at the position!
```

Here's a critical insight that surprises many developers: **you almost never want `LinkedList`**.

The Rust documentation itself states: *"You are **absolutely** certain you **really**, **truly**, want a doubly linked list."*

Why? Because in practice:
- Modern CPUs have fast caches that favor contiguous memory (Vec, VecDeque)
- The overhead of storing two pointers per node often outweighs benefits
- Random access is O(n), which compounds in many algorithms

**When to actually use `LinkedList`**:
- You need O(1) splitting and appending of lists
- You cannot tolerate *any* amortization (rare real-time scenarios)
- You have a cursor pointing into the list and need O(1) insert/remove at that position

For nearly everything else, use `Vec` or `VecDeque`.

---

#### Step 5: HashMap - the hash table

`HashMap<K, V>` is Rust's hash table implementation. If you're coming from Go, this is the equivalent of `map[K]V`.
It provides expected O(1) lookup, insertion, and removal.

Create `src/hashmap_examples.rs`:

```rust
// src/hashmap_examples.rs
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
```

Update `src/main.rs`:
```rust
//..
mod hashmap_examples;

use hashmap_examples::{
    accessing_values, basic_hashmap_operations, creating_hashmaps, custom_keys, entry_api_examples,
    iterating_hashmaps, ownership_and_borrowing, removing_values,
};
//..

fn main() {
    //..
    run_hashmap_examples();
}

fn run_hashmap_examples() {
    section("accessing_values", accessing_values);
    section("basic_hashmap_operations", basic_hashmap_operations);
    section("creating_hashmaps", creating_hashmaps);
    section("custom_keys", custom_keys);
    section("entry_api_examples", entry_api_examples);
    section("iterating_hashmaps", iterating_hashmaps);
    section("ownership_and_borrowing", ownership_and_borrowing);
    section("removing_values", removing_values);
}
```

Run all the examples:
```bash
cargo run
```

A HashMap works by hashing each key to a number, which determines where the value is stored in an internal array:

```
insert("apple", 1)  →  hash("apple") = 42  →  stored at bucket 42
get("apple")        →  hash("apple") = 42  →  look up bucket 42  →  1
```

Good properties:
- O(1) average lookup, insert, and delete
- Keys can be any hashable type

Trade-offs:
- No ordering (use `BTreeMap` if you need sorted keys)
- O(n) worst case (with pathological hash collisions)
- Keys must implement `Hash + Eq + PartialEq` traits

#### Key takeaways for HashMap

| Operation | Time Complexity | Notes |
|-----------|-----------------|-------|
| `insert` | expected amortized O(1) | Occasionally O(n) when resizing/rehashing occurs |
| `get` | expected O(1) | Plus cost to hash the key; rare bad cases possible |
| `remove` | expected O(1) | Plus hashing cost; does not auto-shrink |
| `contains_key` | expected O(1) | Essentially the same lookup path as `get` |
| Iteration (`iter`, `keys`, `values`) | O(capacity) (current impl) | Arbitrary order; may scan empty buckets |

**When to use `HashMap`**:
- You need fast key-value lookups
- You don't care about the order of elements
- You're implementing a cache
- You're counting occurrences (with the Entry API)

---