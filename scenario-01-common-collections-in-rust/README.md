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
- [Step 6: BTreeMap - The Ordered Map](#step-6-btreemap---the-ordered-map)
- [Step 7: HashSet and BTreeSet - The Set Types](#step-7-hashset-and-btreeset---the-set-types)
- [Step 8: BinaryHeap - The Priority Queue](#step-8-binaryheap---the-priority-queue)
- [Step 9: Performance Comparison and Benchmarking](#step-9-performance-comparison-and-benchmarking)

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

#### Step 6: BTreeMap - the ordered map

`BTreeMap<K, V>` is an ordered map based on a `B-Tree`. B-trees are self-balancing tree structures optimized
for systems that read and write large blocks of data. Unlike `HashMap`, it keeps keys sorted and allows range queries.
Also, if you need to iterate over entries in order or query a range of keys, `BTreeMap` is your tool.

The fundamental difference between `HashMap` and `BTreeMap`:

```
  HashMap:                        BTreeMap:
  ┌─────────────────────────┐     ┌─────────────────────────────────┐
  │ Hash table (buckets)    │     │       ┌───[5,10]───┐            │
  │ bucket 0:  (key3, val)  │     │      /      |       \           │
  │ bucket 1:  (key1, val)  │     │   [1,3]   [7,8]   [12,15,20]    │
  │ bucket 2:  empty        │     │                                 │
  │ bucket 3:  (key2, val)  │     │   Keys always in sorted order!  │
  │ ...                     │     │                                 │
  └─────────────────────────┘     └─────────────────────────────────┘
  Iteration order: arbitrary      Iteration order: sorted by key
```

Create `src/btreemap_examples.rs`:

```rust
// src/btreemap_examples.rs
//
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
```

Update `src/main.rs`:
```rust
//..
mod btreemap_examples;

use btreemap_examples::{
    calendar_example, creating_btreemaps, custom_key_types, entry_api_examples as entry_api,
    leaderboard_example, min_max_operations, mutable_range_queries, range_queries,
    sorted_iteration, time_series_example,
};
//..

fn main() {
    //..
    run_btreemap_examples();
}

fn run_btreemap_examples() {
    section("creating_btreemaps", creating_btreemaps);
    section("sorted_iteration", sorted_iteration);
    section("range_queries", range_queries);
    section("mutable_range_queries", mutable_range_queries);
    section("min_max_operations", min_max_operations);
    section("entry_api_examples", entry_api);
    section("leaderboard_example", leaderboard_example);
    section("time_series_example", time_series_example);
    section("calendar_example", calendar_example);
    section("custom_key_types", custom_key_types);
}
```

Run all the examples:
```bash
cargo run
```

A `B-Tree` is a self-balancing tree that keeps data sorted and allows searches, insertions, and deletions in logarithmic time.
Unlike binary trees, each node can have multiple keys and children:

```
B-Tree structure (simplified):

                    ┌─────────────┐
                    │  [10, 20]   │  ← Root node with 2 keys
                    └─────────────┘
                   /      |       \
         ┌────────┐  ┌────────┐  ┌────────┐
         │ [3,7]  │  │[12,15] │  │[25,30] │  ← Internal nodes
         └────────┘  └────────┘  └────────┘
         /   |   \
       ...  ...  ...  ← Leaf nodes contain actual data

Key properties:
1. All keys in left subtree < parent key < all keys in right subtree
2. All leaf nodes are at the same depth (perfectly balanced)
3. Nodes have multiple keys (good for cache locality)
```

This structure gives `BTreeMap` several advantages:
- **Sorted order**: Keys are stored in order, so iteration is always sorted. No need to collect and sort separately.
- **Range queries**: To find all keys between X and Y, traverse to X, then walk forward until Y. This is `O(log n + k)` where `k` is the number of results.
- **Predictable performance**: All operations are `O(log n)` - no best/worst case distinction like HashMap's O(1) average / O(n) worst case.


#### Key takeaways for BTreeMap

| Operation | Time Complexity | Notes                                                                                                         |
|-----------|-----------------|---------------------------------------------------------------------------------------------------------------|
| `insert` | O(log n) | Maintains sorted order                                                                                        |
| `get` | O(log n) | B-tree lookup (tree height is logarithmic; current node search is linear within a node)                       |
| `remove` | O(log n) | Rebalances if needed                                                                                          |
| `contains_key` | O(log n) | Same as `get`                                                                                                 |
| `range` | O(log n + k) | k = items in range; HashMap has no ordered range API; you'd typically have to scan/filter everything |
| `first_key_value` | O(log n) | Traverse to leftmost leaf                                                                                     |
| `last_key_value` | O(log n) | Traverse to rightmost leaf                                                                                    |
| Iteration | O(n) | Always in sorted order                                                                                        |

**When to use `BTreeMap`**:
- You need entries sorted by key
- You need range queries (e.g., "all entries where key is between X and Y")
- You need to find the minimum or maximum key efficiently
- You need a deterministic, reproducible iteration order

**When to prefer `HashMap` instead**:
- You only need lookup/insert/delete (no ordering requirements)
- O(1) average performance matters more than O(log n) guaranteed
- You're building a cache where order is irrelevant

---

#### Step 7: HashSet and BTreeSet - the set types

Sets store unique values with no associated data. They're essentially maps where you only care about the keys - in fact,
`HashSet<T>` is implemented as `HashMap<T, ()>` and `BTreeSet<T>` as `BTreeMap<T, ()>` under the hood.

```
Sets are perfect for:
  • Tracking unique values (deduplication)
  • Fast membership testing ("is X in the set?")
  • Mathematical set operations (union, intersection, difference)

The relationship between maps and sets:

  HashMap<K, V>  →  HashSet<T>     (T is the "key", no value)
  BTreeMap<K, V> →  BTreeSet<T>    (T is the "key", no value)

Key trade-offs:
  HashSet<T>              BTreeSet<T>
  - O(1) average          - O(log n) all operations
  - Unordered             - Always sorted
  - Needs Hash + Eq       - Needs Ord
  - No range queries      - Supports range queries
```

Create `src/set_examples.rs`:

```rust
// src/set_examples.rs
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
```

Update `src/main.rs`:
```rust
//..
mod set_examples;

use set_examples::{
    adding_removing_elements, btreeset_min_max, btreeset_range_queries, btreeset_set_operations,
    btreeset_sorted_iteration, checking_membership, creating_btreesets, creating_hashsets,
    custom_types_in_hashset, iterating_hashsets, practical_comparing_lists,
    practical_deduplication, practical_finding_duplicates, practical_leaderboard,
    practical_tag_system, set_operations, set_relationships,
};
//..

fn main() {
    //..
    
    run_set_examples();
}

fn run_set_examples() {
    section("adding_removing_elements", adding_removing_elements);
    section("btreeset_min_max", btreeset_min_max);
    section("btreeset_range_queries", btreeset_range_queries);
    section("btreeset_set_operations", btreeset_set_operations);
    section("btreeset_sorted_iteration", btreeset_sorted_iteration);
    section("checking_membership", checking_membership);
    section("creating_btreesets", creating_btreesets);
    section("creating_hashsets", creating_hashsets);
    section("custom_types_in_hashset", custom_types_in_hashset);
    section("iterating_hashsets", iterating_hashsets);
    section("practical_comparing_lists", practical_comparing_lists);
    section("practical_deduplication", practical_deduplication);
    section("practical_finding_duplicates", practical_finding_duplicates);
    section("practical_leaderboard", practical_leaderboard);
    section("practical_tag_system", practical_tag_system);
    section("set_operations", set_operations);
    section("set_relationships", set_relationships);
}
```

Run all the examples:
```bash
cargo run
```

`Understanding set operations`

Set operations are fundamental mathematical concepts that sets make easy to express:

```
Given:
  Set A = {1, 2, 3, 4, 5}
  Set B = {4, 5, 6, 7, 8}

UNION (A ∪ B) - "OR" - elements in A or B or both
┌─────────────────────────────────┐
│    A              B             │
│    ┌─────┬───┬─────┐            │
│    │1 2 3│4 5│6 7 8│            │
│    └─────┴───┴─────┘            │
│                                 │
│   Result: {1,2,3,4,5,6,7,8}     │
└─────────────────────────────────┘

INTERSECTION (A ∩ B) - "AND" - elements in both A and B
┌─────────────────────────────────┐
│    A              B             │
│    ┌─────┬───┬─────┐            │
│    │     │4 5│     │            │
│    └─────┴───┴─────┘            │
│                                 │
│    Result: {4, 5}               │
└─────────────────────────────────┘

DIFFERENCE (A - B) - elements in A but NOT in B
┌─────────────────────────────────┐
│    A              B             │
│    ┌─────┬───┬─────┐            │
│    │1 2 3│   │     │            │
│    └─────┴───┴─────┘            │
│                                 │
│    Result: {1, 2, 3}            │
└─────────────────────────────────┘

SYMMETRIC DIFFERENCE (A △ B) - "XOR" - in one but not both
┌─────────────────────────────────┐
│    A              B             │
│    ┌─────┬───┬─────┐            │
│    │1 2 3│   │6 7 8│            │
│    └─────┴───┴─────┘            │
│                                 │
│    Result: {1,2,3,6,7,8}        │
└─────────────────────────────────┘
```

#### Key takeaways for Sets

`HashSet` and `BTreeSet` have the same performance characteristics as their `Map` counterparts, just without associated values.

**When to use `HashSet`**:
- You need to track unique items
- You need fast membership testing (O(1))
- You need set operations (union, intersection, etc.)

**When to use `BTreeSet`**:
- You need unique items in sorted order
- You need range queries on unique items
- You need min/max element efficiently

---

#### Step 8: BinaryHeap - the priority queue

`BinaryHeap<T>` is a priority queue implemented as a binary max-heap. Unlike other collections that store elements
in insertion order or sorted order, a heap maintains a partial ordering that guarantees the maximum element is always
accessible in `O(1)` time.

```
A heap is a tree where each parent is greater than its children:

         ┌───┐
         │ 9 │  ← Maximum is ALWAYS at the root!
         └───┘
        /     \
     ┌───┐   ┌───┐
     │ 7 │   │ 8 │
     └───┘   └───┘
    /   \   /
  ┌───┐ ┌───┐ ┌───┐
  │ 3 │ │ 5 │ │ 6 │
  └───┘ └───┘ └───┘

Key insight: The heap is NOT fully sorted! Only the maximum is guaranteed
at the top. This partial ordering is what makes operations efficient:

  - peek(): O(1)     - maximum is always at index 0
  - push(): O(log n) - add element, "bubble up" to restore heap property
  - pop(): O(log n)  - remove max, "bubble down" to restore heap property

Internally, the heap is stored as a Vec where for node at index i:
  - Left child is at index 2i + 1
  - Right child is at index 2i + 2
  - Parent is at index (i - 1) / 2
```

Create `src/binaryheap_examples.rs`:

```rust
// src/binaryheap_examples.rs
// BinaryHeap is Rust's implementation of a priority queue using a binary
// max-heap. It always keeps the largest element readily accessible at the top.
//
// A heap is a tree structure where each parent is greater than its children:
//
//         ┌───┐
//         │ 9 │  ← Maximum is always at the root!
//         └───┘
//        /     \
//     ┌───┐   ┌───┐
//     │ 7 │   │ 8 │
//     └───┘   └───┘
//    /   \   /
//  ┌───┐ ┌───┐ ┌───┐
//  │ 3 │ │ 5 │ │ 6 │
//  └───┘ └───┘ └───┘
//
// Key insight: BinaryHeap is NOT fully sorted. Only the maximum is guaranteed
// to be at the top. The internal order is "heap order" - not sorted order.
//
// This partial ordering is what makes it efficient:
//   - peek(): O(1) - the maximum is always at index 0
//   - push(): O(log n) - add element, "bubble up" to restore heap property
//   - pop(): O(log n) - remove maximum, "bubble down" to restore heap property

use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// Demonstrates all the different ways to create a BinaryHeap.
///
/// Unlike HashMap/HashSet, BinaryHeap requires elements to implement Ord
/// (not Hash). This is because elements are ordered by comparison, not hashing.
pub fn creating_binaryheaps() {
    // Method 1: BinaryHeap::new()
    // The most common way - start empty and add items
    let mut heap: BinaryHeap<i8> = BinaryHeap::new();
    heap.push(3);
    heap.push(1);
    heap.push(4);
    println!("Created with new(): {:?}", heap);
    println!("Note: Internal order is heap order, NOT sorted order!");
    println!("But peek() always returns the maximum: {:?}", heap.peek());

    // Method 2: BinaryHeap::from() - from array
    // The heap property is established during construction
    let heap2: BinaryHeap<i8> = BinaryHeap::from([5, 2, 8, 1, 9]);
    println!("\nFrom array [5, 2, 8, 1, 9]: {:?}", heap2);
    println!("Maximum is: {:?}", heap2.peek());

    // Method 3: collect() from iterator
    // Building from an iterator is O(n), more efficient than n pushes
    let heap3: BinaryHeap<i8> = (1..=5).collect();
    println!("\nCollected from 1..=5: {:?}", heap3);

    // Method 4: with_capacity (performance optimization)
    // Pre-allocate when you know approximate size
    let heap4: BinaryHeap<i8> = BinaryHeap::with_capacity(1_00);
    println!(
        "\nWith capacity 100, current len: {}, capacity: {}",
        heap4.len(),
        heap4.capacity()
    );
}

/// Demonstrates the fundamental max-heap behavior.
///
/// The key insight: BinaryHeap always gives you the MAXIMUM element.
/// Every push and pop operation maintains this invariant.
pub fn max_heap_behavior() {
    println!("Max-Heap behavior");

    let mut heap: BinaryHeap<i8> = BinaryHeap::new();

    // Watch how the maximum changes as we push elements
    println!("Pushing elements and observing the maximum:");
    for item in [3, 1, 4, 1, 5, 9, 2, 6] {
        heap.push(item);
        println!(
            "  After push({}): peek = {:?}, heap = {:?}",
            item,
            heap.peek(),
            heap
        );
    }

    // peek() - see the maximum without removing (O(1))
    println!("\npeek() returns {:?} - the maximum", heap.peek());

    // pop() - remove and return the maximum (O(log n))
    // Elements come out in descending order!
    println!("\nPopping all elements (they come out in descending order):");
    while let Some(max) = heap.pop() {
        print!("{} ", max);
    }
    println!();
    println!("Heap is now empty: {}", heap.is_empty());
}

/// Demonstrates how to create a min-heap using Reverse.
///
/// BinaryHeap is a max-heap by default. To get min-heap behavior,
/// wrap elements in std::cmp::Reverse, which inverts the ordering.
pub fn min_heap_with_reverse() {
    println!("Min-Heap with Reverse");

    // Reverse<T> inverts the Ord implementation
    // So Reverse(1) > Reverse(5), making the smallest value "largest"
    let mut min_heap: BinaryHeap<Reverse<i8>> = BinaryHeap::new();

    for val in [3, 1, 4, 1, 5, 9, 2, 6] {
        min_heap.push(Reverse(val));
    }

    println!("Min-heap created with values [3, 1, 4, 1, 5, 9, 2, 6]");
    println!("peek() returns {:?}", min_heap.peek()); // Reverse(1)

    // Pop gives smallest first (ascending order)
    println!("\nPopping from min-heap (ascending order):");
    while let Some(Reverse(min)) = min_heap.pop() {
        print!("{} ", min);
    }
    println!();

    // You can also use a type alias for clarity
    // type MinHeap<T> = BinaryHeap<Reverse<T>>;
}

/// Demonstrates push, pop, and peek operations in detail.
///
/// These are the core operations that make BinaryHeap useful as a priority queue.
pub fn push_pop_operations() {
    println!("Push, Pop, and Peek Operations");

    let mut heap: BinaryHeap<i8> = BinaryHeap::from([5, 3, 7]);
    println!("Initial heap: {:?}", heap);

    // push() - add an element, O(log n)
    // The heap reorganizes to maintain the max-heap property
    heap.push(10);
    println!("\nAfter push(10): {:?}", heap);
    println!("New maximum: {:?}", heap.peek());

    heap.push(1);
    println!("After push(1): {:?}", heap);
    println!("Maximum unchanged: {:?}", heap.peek());

    // pop() - remove and return the maximum, O(log n)
    let max: Option<i8> = heap.pop();
    println!("\npop() returned: {:?}", max);
    println!("Heap after pop: {:?}", heap);
    println!("New maximum: {:?}", heap.peek());

    // peek() - view the maximum without removing, O(1)
    // Returns Option<&T> since heap might be empty
    match heap.peek() {
        Some(&max) => println!("\nCurrent maximum is: {}", max),
        None => println!("\nHeap is empty!"),
    }

    // peek_mut() - modify the maximum in place
    // When the PeekMut guard is dropped, the heap reorganizes if needed
    println!("\nUsing peek_mut() to modify the maximum:");
    if let Some(mut max_ref) = heap.peek_mut() {
        println!("  Current max: {}", *max_ref);
        *max_ref = 1; // Change the maximum to a small value
        println!("  Set to 1, heap will reorganize when guard drops");
    }
    println!("After peek_mut(): {:?}", heap);
    println!("The heap automatically reorganized!");
}

/// Demonstrates bulk operations on BinaryHeap.
///
/// These operations are useful for combining heaps or filtering elements.
pub fn bulk_operations() {
    println!("Bulk Operations");

    // append() - move all elements from another heap, O(n + m)
    let mut heap1: BinaryHeap<i8> = BinaryHeap::from([1, 2, 3]);
    let mut heap2: BinaryHeap<i8> = BinaryHeap::from([4, 5, 6]);

    println!("Before append:");
    println!("  heap1: {:?}", heap1);
    println!("  heap2: {:?}", heap2);

    heap1.append(&mut heap2);
    println!("\nAfter heap1.append(&mut heap2):");
    println!("  heap1: {:?}", heap1);
    println!("  heap2: {:?} (now empty)", heap2);

    // extend() - add elements from an iterator
    let mut heap: BinaryHeap<i8> = BinaryHeap::from([10]);
    heap.extend([1, 2, 3, 4, 5]);
    println!("\nAfter extend([1,2,3,4,5]): {:?}", heap);

    // retain() - keep only elements matching a predicate
    let mut heap: BinaryHeap<i8> = (1..=10).collect();
    println!("\nBefore retain: {:?}", heap);
    heap.retain(|&x| x % 2 == 0); // Keep only even numbers
    println!("After retain (even only): {:?}", heap);

    // clear() - remove all elements
    heap.clear();
    println!("\nAfter clear(): {:?}, is_empty: {}", heap, heap.is_empty());

    // drain() - remove all elements as an iterator
    let mut heap: BinaryHeap<i8> = BinaryHeap::from([3, 1, 4, 1, 5]);
    println!("\nDraining heap:");
    let drained: Vec<i8> = heap.drain().collect();
    println!("  Drained elements: {:?}", drained);
    println!("  Heap after drain: {:?}", heap);
}

/// Demonstrates converting a BinaryHeap to other collections.
///
/// Key insight: into_sorted_vec() gives you a sorted Vec efficiently.
pub fn conversion_operations() {
    println!("Conversion Operations");

    let heap: BinaryHeap<i8> = BinaryHeap::from([3, 1, 4, 1, 5, 9, 2, 6]);
    println!("Original heap: {:?}", heap);

    // into_vec() - consume heap into unsorted Vec, O(1)
    // This just unwraps the internal storage
    let heap_copy: BinaryHeap<i8> = BinaryHeap::from([3, 1, 4, 1, 5, 9, 2, 6]);
    let unsorted: Vec<i8> = heap_copy.into_vec();
    println!("\ninto_vec() (unsorted): {:?}", unsorted);

    // into_sorted_vec() - consume heap into sorted Vec, O(n log n)
    // This is essentially heapsort!
    let sorted: Vec<i8> = heap.into_sorted_vec();
    println!("into_sorted_vec() (sorted ascending): {:?}", sorted);

    // For descending order, pop repeatedly or use Reverse
    let heap: BinaryHeap<i8> = BinaryHeap::from([3, 1, 4, 1, 5, 9, 2, 6]);
    let mut descending: Vec<i8> = Vec::with_capacity(heap.len());
    let mut heap_mut = heap;
    while let Some(val) = heap_mut.pop() {
        descending.push(val);
    }
    println!("Via repeated pop (sorted descending): {:?}", descending);
}

/// Demonstrates iteration patterns for BinaryHeap.
///
/// IMPORTANT: iter() does NOT give sorted order! Only pop() does.
pub fn iteration_patterns() {
    println!("Iteration patterns");

    let heap: BinaryHeap<i8> = BinaryHeap::from([3, 1, 4, 1, 5, 9, 2, 6]);

    // iter() - iterate in ARBITRARY heap order (NOT sorted!)
    println!("iter() gives heap order (NOT sorted):");
    print!("  ");
    for val in heap.iter() {
        print!("{} ", val);
    }
    println!();

    // For sorted iteration, you must pop()
    println!("\nFor sorted iteration, use pop():");
    let mut heap_clone: BinaryHeap<i8> = heap.clone();
    print!("  ");
    while let Some(val) = heap_clone.pop() {
        print!("{} ", val);
    }
    println!();

    // Or use into_sorted_vec()
    println!("\nOr use into_sorted_vec():");
    let sorted: Vec<i8> = heap.into_sorted_vec();
    println!("  {:?}", sorted);
}

/// Practical example: Task scheduler with priorities.
///
/// This is the classic use case for a priority queue - process tasks
/// in order of priority, not in order of arrival.
pub fn practical_task_scheduler() {
    println!("Practical example: task scheduler");

    #[derive(Debug, Eq, PartialEq)]
    struct Task {
        priority: u8,
        name: String,
    }

    // Implement Ord so higher priority = greater (processed first)
    impl Ord for Task {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            // Compare by priority only
            // Higher priority number = more important = should come first
            self.priority.cmp(&other.priority)
        }
    }

    impl PartialOrd for Task {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut task_queue: BinaryHeap<Task> = BinaryHeap::new();

    // Add tasks with different priorities (order of insertion doesn't matter!)
    task_queue.push(Task {
        priority: 1,
        name: "Update documentation".to_string(),
    });
    task_queue.push(Task {
        priority: 10,
        name: "Fix production bug".to_string(),
    });
    task_queue.push(Task {
        priority: 5,
        name: "Code review".to_string(),
    });
    task_queue.push(Task {
        priority: 3,
        name: "Write tests".to_string(),
    });
    task_queue.push(Task {
        priority: 10,
        name: "Security patch".to_string(),
    });
    task_queue.push(Task {
        priority: 7,
        name: "Performance optimization".to_string(),
    });

    println!("Processing tasks by priority (highest first):");
    while let Some(task) = task_queue.pop() {
        println!("  [Priority {:2}] {}", task.priority, task.name);
    }
}

/// Practical example: Finding K largest elements efficiently.
///
/// This is more memory-efficient than sorting the entire array
/// when K is much smaller than N.
pub fn practical_k_largest() {
    println!("Practical Example: K Largest Elements");

    // Strategy: Use a MIN-heap of size k
    // - Keep only the k largest elements seen so far
    // - When heap exceeds size k, remove the smallest
    // - At the end, the heap contains exactly the k largest

    fn k_largest(nums: &[i8], k: usize) -> Vec<i8> {
        let mut min_heap: BinaryHeap<Reverse<i8>> = BinaryHeap::with_capacity(k + 1);

        for &num in nums {
            min_heap.push(Reverse(num));
            if min_heap.len() > k {
                min_heap.pop(); // Remove the smallest (which is the "largest" in Reverse order)
            }
        }

        // Extract results
        min_heap.into_iter().map(|Reverse(x)| x).collect()
    }

    // Similarly for k smallest using a max-heap
    fn k_smallest(nums: &[i8], k: usize) -> Vec<i8> {
        let mut max_heap: BinaryHeap<i8> = BinaryHeap::with_capacity(k + 1);

        for &num in nums {
            max_heap.push(num);
            if max_heap.len() > k {
                max_heap.pop(); // Remove the largest
            }
        }

        max_heap.into_vec()
    }

    let data: Vec<i8> = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7];
    println!("Data: {:?}", data);
    println!("5 largest: {:?}", k_largest(&data, 5));
    println!("5 smallest: {:?}", k_smallest(&data, 5));
}

/// Practical example: Merging K sorted lists.
///
/// This is a classic problem that demonstrates the power
/// of priority queues for efficient merging.
pub fn practical_merge_sorted_lists() {
    println!("Practical Example: Merge K Sorted Lists");

    fn merge_k_sorted(lists: Vec<Vec<i8>>) -> Vec<i8> {
        // Entry: (value, list_index, element_index)
        // Use Reverse for min-heap behavior (smallest value first)
        let mut heap: BinaryHeap<Reverse<(i8, usize, usize)>> = BinaryHeap::new();

        // Initialize with the first element of each list
        for (list_idx, list) in lists.iter().enumerate() {
            if !list.is_empty() {
                heap.push(Reverse((list[0], list_idx, 0)));
            }
        }

        let mut result: Vec<i8> = Vec::new();

        while let Some(Reverse((val, list_idx, elem_idx))) = heap.pop() {
            result.push(val);

            // Add next element from the same list (if available)
            let next_idx: usize = elem_idx + 1;
            if next_idx < lists[list_idx].len() {
                heap.push(Reverse((lists[list_idx][next_idx], list_idx, next_idx)));
            }
        }

        result
    }

    let lists: Vec<Vec<i8>> = vec![vec![1, 4, 7, 10], vec![2, 5, 8, 11], vec![3, 6, 9, 12]];

    println!("Lists to merge:");
    for (i, list) in lists.iter().enumerate() {
        println!("  List {}: {:?}", i, list);
    }

    let merged: Vec<i8> = merge_k_sorted(lists);
    println!("\nMerged result: {:?}", merged);
}

/// Practical example: Dijkstra's shortest path algorithm structure.
///
/// BinaryHeap is essential for efficient graph algorithms.
pub fn practical_dijkstra_concept() {
    println!("Practical Example: Dijkstra's Algorithm (Concept)");

    // In Dijkstra's algorithm, we process nodes by minimum distance
    // BinaryHeap (as min-heap) is perfect for this

    #[derive(Debug, Eq, PartialEq)]
    struct State {
        cost: u32,
        node: usize,
    }

    // Reverse ordering: smallest cost = highest priority
    impl Ord for State {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            // Note: reversed! other.cost.cmp(&self.cost)
            other.cost.cmp(&self.cost)
        }
    }

    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    // Simulate a simple graph traversal
    let mut priority_queue: BinaryHeap<State> = BinaryHeap::new();

    // Add nodes with their distances from source
    priority_queue.push(State { cost: 10, node: 2 });
    priority_queue.push(State { cost: 3, node: 1 });
    priority_queue.push(State { cost: 15, node: 3 });
    priority_queue.push(State { cost: 7, node: 4 });

    println!("Processing nodes by minimum cost (Dijkstra's order):");
    while let Some(State { cost, node }) = priority_queue.pop() {
        println!("  Visit node {} with distance {}", node, cost);
    }

    println!("\nIn real Dijkstra's:");
    println!("  1. Start with source node at distance 0");
    println!("  2. Pop minimum distance node from heap");
    println!("  3. For each neighbor, if new path is shorter, update and push");
    println!("  4. Repeat until destination reached or heap empty");
}

/// Practical example: Heapsort implementation.
///
/// Demonstrates how BinaryHeap can be used for sorting.
pub fn practical_heapsort() {
    println!("Practical Example: Heapsort");

    fn heapsort<T: Ord>(data: Vec<T>) -> Vec<T> {
        // Build heap - O(n)
        let heap: BinaryHeap<T> = data.into_iter().collect();
        // Convert to sorted vec - O(n log n)
        heap.into_sorted_vec()
    }

    fn heapsort_descending<T: Ord>(data: Vec<T>) -> Vec<T> {
        let mut heap: BinaryHeap<T> = data.into_iter().collect();
        let mut result: Vec<T> = Vec::with_capacity(heap.len());
        while let Some(val) = heap.pop() {
            result.push(val);
        }
        result
    }

    let data: Vec<i8> = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    println!("Original: {:?}", data);
    println!("Heapsort ascending: {:?}", heapsort(data.clone()));
    println!("Heapsort descending: {:?}", heapsort_descending(data));
}

/// Demonstrates using custom types with BinaryHeap.
///
/// Your type must implement Ord (and therefore PartialOrd, Eq, PartialEq).
pub fn custom_types_in_heap() {
    println!("Custom Types in BinaryHeap");

    // Example 1: Simple struct with derived ordering
    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
    struct Score {
        points: i16,
        player: String,
    }

    let mut leaderboard: BinaryHeap<Score> = BinaryHeap::new();

    leaderboard.push(Score {
        points: 100,
        player: "Alice".to_string(),
    });
    leaderboard.push(Score {
        points: 150,
        player: "Bob".to_string(),
    });
    leaderboard.push(Score {
        points: 120,
        player: "Charlie".to_string(),
    });

    println!("Leaderboard (highest score first):");
    while let Some(score) = leaderboard.pop() {
        println!("  {}: {} points", score.player, score.points);
    }

    // Example 2: Custom ordering (multiple criteria)
    #[derive(Debug, Eq, PartialEq)]
    struct Event {
        priority: u8,
        timestamp: u64, // Earlier timestamp = higher priority for same priority level
        name: String,
    }

    impl Ord for Event {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            // First compare by priority (higher = better)
            // Then by timestamp (earlier = better, so reverse)
            self.priority
                .cmp(&other.priority)
                .then_with(|| other.timestamp.cmp(&self.timestamp))
        }
    }

    impl PartialOrd for Event {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut event_queue: BinaryHeap<Event> = BinaryHeap::new();

    event_queue.push(Event {
        priority: 5,
        timestamp: 100,
        name: "Task A".to_string(),
    });
    event_queue.push(Event {
        priority: 5,
        timestamp: 50,
        name: "Task B".to_string(),
    }); // Same priority, earlier
    event_queue.push(Event {
        priority: 10,
        timestamp: 200,
        name: "Task C".to_string(),
    }); // Higher priority

    println!("\nEvent queue with priority + timestamp ordering:");
    while let Some(event) = event_queue.pop() {
        println!(
            "  [P{}, T{}] {}",
            event.priority, event.timestamp, event.name
        );
    }
}
```

Update `src/main.rs`:
```rust
//..
mod binaryheap_examples;

use binaryheap_examples::{
    bulk_operations, conversion_operations, creating_binaryheaps, custom_types_in_heap,
    iteration_patterns, max_heap_behavior, min_heap_with_reverse, practical_dijkstra_concept,
    practical_heapsort, practical_k_largest, practical_merge_sorted_lists,
    practical_task_scheduler, push_pop_operations,
};
//..

fn main() {
    //..
    run_binary_heap_examples();
}

fn run_binary_heap_examples() {
    section("bulk_operations", bulk_operations);
    section("conversion_operations", conversion_operations);
    section("creating_binaryheaps", creating_binaryheaps);
    section("custom_types_in_heap", custom_types_in_heap);
    section("iteration_patterns", iteration_patterns);
    section("max_heap_behavior", max_heap_behavior);
    section("min_heap_with_reverse", min_heap_with_reverse);
    section("practical_dijkstra_concept", practical_dijkstra_concept);
    section("practical_heapsort", practical_heapsort);
    section("practical_k_largest", practical_k_largest);
    section("practical_merge_sorted_lists", practical_merge_sorted_lists);
    section("practical_task_scheduler", practical_task_scheduler);
    section("push_pop_operations", push_pop_operations);
}
```

Run all the examples:
```bash
cargo run
```

`Understanding the heap property`

The heap maintains a simple but powerful invariant: every parent is greater than or equal to its children.
This gives us `O(1)` access to the maximum while keeping insertions and deletions at `O(log n)`.

```
MAX-HEAP: Parent ≥ Children          MIN-HEAP (using Reverse): Parent ≤ Children

       ┌───┐                                ┌───┐
       │ 9 │ ← Maximum                      │ 1 │ ← Minimum
       └───┘                                └───┘
      /     \                              /     \
   ┌───┐   ┌───┐                        ┌───┐   ┌───┐
   │ 7 │   │ 8 │                        │ 3 │   │ 2 │
   └───┘   └───┘                        └───┘   └───┘

BinaryHeap<i8>                        BinaryHeap<Reverse<i8>>
heap.peek() → 9                       heap.peek() → Reverse(1)
heap.pop() → 9, 8, 7, ...             heap.pop() → 1, 2, 3, ...
```

`Why iteration is NOT sorted`

This is a common source of confusion. The heap property only guarantees that the root is the maximum - it says nothing
about the relationship between siblings or cousins in the tree:

```
This is a valid max-heap:          But the array representation is:

       ┌───┐                        [9, 7, 8, 3, 5, 6, 2]
       │ 9 │                         │  └─┬─┘  └───┬───┘
       └───┘                         │    │        │
      /     \                        │    │        └── Level 2 (leaves)
   ┌───┐   ┌───┐                     │    └── Level 1
   │ 7 │   │ 8 │                     └── Level 0 (root)
   └───┘   └───┘
  /   \   /   \                     Notice: 7 comes before 8, even though 8 > 7!
┌───┐┌───┐┌───┐┌───┐                The array is NOT sorted - only heap-ordered.
│ 3 ││ 5 ││ 6 ││ 2 │
└───┘└───┘└───┘└───┘

iter() returns: [9, 7, 8, 3, 5, 6, 2]  ← Heap order, NOT sorted!
pop() returns:  [9, 8, 7, 6, 5, 3, 2]  ← Sorted descending!
```

#### Key takeaways for BinaryHeap

| Operation | Time Complexity | Notes                                                                                                                                                                                                           |
|-----------|-----------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `push` | O(log n) | Bubble up to restore heap property. Expected (average) cost over random insertion orders: O(1), amortized cost can degrade to O(log n), worst-case single call can be O(n) if the heap has to grow (reallocate) |
| `pop` | O(log n) | Bubble down to restore heap property                                                                                                                                                                            |
| `peek` | O(1) | Maximum is always at index 0                                                                                                                                                                                    |
| `peek_mut` | O(log n) amortized | If you do not modify the top element: O(1). If you do modify it: worst case O(log n) (because the heap must be repaired when the guard drops)                                                                   |
| Build from iter | O(n) | Converting a vector into a heap, building from an iterator by collecting then heapifying                                                                                                                        |
| `into_sorted_vec` | O(n log n) | Converting a heap to a sorted vector                                                                                                                                                                            |
| `iter` | O(n) | NOT sorted! Heap order only                                                                                                                                                                                     |

**When to use `BinaryHeap`**:
- You need a priority queue (process by priority, not arrival order)
- You always want to process the largest (or smallest with `Reverse`) element
- Finding k largest/smallest elements from a stream
- Graph algorithms (Dijkstra's, Prim's)
- Merging k sorted sequences
- Scheduling tasks by priority

**When NOT to use `BinaryHeap`**:
- You need sorted iteration (use `BTreeSet` instead)
- You need to find arbitrary elements (use `HashMap`/`BTreeMap`)
- You need both min AND max efficiently (use `BTreeSet`)

---

#### Step 9: Performance comparison and benchmarking

Understanding collection performance requires more than just knowing Big-O complexity. It depends on factors like cache locality,
memory allocation patterns, and the actual size of your data. In this section we'll use Criterion to measure and compare collections rigorously.

`Setting up Criterion`

First, ensure your `Cargo.toml` includes Criterion as a dev dependency and configures the benchmark harness:

```toml
[package]
name = "collections_demo"
version = "0.1.0"
edition = "2024"

[dependencies]
rustc_version_runtime = "0.3"

[dev-dependencies]
criterion = "0.8.1"

[[bench]]
name = "collections_benchmark"
harness = false
```

Clean up `src/main.rs` file (to avoid unnecessary warnings):

```rust
use rustc_version_runtime;

fn main() {
    println!("Rust Collections Demo");
    println!("Compiled with: {:?}", rustc_version_runtime::version());
}
```

Create the benchmark file at `benches/collections_benchmark.rs`:

```rust
// benches/collections_benchmark.rs
//
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
```

#### Running the benchmarks

To run all benchmarks:

```bash
cargo bench
```

To run a specific benchmark group:

```bash
cargo bench -- Insertions
cargo bench -- Lookups
cargo bench -- Front_Operations
cargo bench -- Iteration
cargo bench -- Range_Queries
cargo bench -- Priority_Operations
cargo bench -- Entry_API
cargo bench -- Removals
cargo bench -- Scaling
```

#### Understanding Criterion output

When you run `cargo bench`, Criterion produces output like this:

```
Insertions/Vec/10000    time:   [45.123 µs 45.456 µs 45.789 µs]
                        thrpt:  [218.39 Melem/s 220.00 Melem/s 221.60 Melem/s]
                 change: [-2.1234% -0.5678% +0.9876%] (p = 0.12 > 0.05)
                        No change in performance detected.
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe
```

Here's what each part means:

The **time** line shows the 95% confidence interval: `[lower_bound  point_estimate  upper_bound]`. Criterion is 95% confident the true mean lies within this range. A tighter interval means more consistent measurements.

The **thrpt** (throughput) line shows elements processed per second when you've set `Throughput::Elements()`. This makes it easier to compare across different batch sizes.

The **change** line compares to the previous run, showing if there's a statistically significant performance regression or improvement. The `p` value indicates statistical significance - if `p < 0.05`, Criterion considers the change significant.

The **outliers** line reports measurements that deviate significantly from the norm, often due to system noise. Criterion automatically handles these in its statistical analysis.

#### Viewing HTML reports

Criterion generates detailed HTML reports in `target/criterion/`. Open `target/criterion/report/index.html` in a browser to see interactive graphs showing performance distributions, comparisons across different sizes, violin plots showing measurement distribution, and historical performance trends.

#### Interpreting the results

After running the benchmarks, you'll observe several important patterns that illuminate the performance characteristics of each collection type.

**Insertion Performance**

The insertion benchmarks reveal fascinating differences between collections. At 10,000 elements, `Vec::with_capacity` completed in approximately 5.6 µs compared to `Vec::new` at 6.8 µs - a 20% improvement just from avoiding reallocations. This demonstrates why pre-allocation matters when you know your approximate data size.

`VecDeque` performed comparably to `Vec` for back insertions (around 7.4 µs for 10,000 elements), confirming that its ring buffer implementation doesn't add significant overhead. Both `push_back` and `push_front` showed nearly identical times (~8.1 µs), validating the O(1) amortized complexity at both ends.

`LinkedList` took approximately 190 µs for 10,000 insertions - roughly 28x slower than `Vec`. This dramatic difference stems from per-element heap allocation overhead. Each `push_back` requires allocating a new node, whereas `Vec` allocates in bulk when it grows.

Hash-based collections (`HashMap`, `HashSet`) showed similar insertion times around 190-196 µs for 10,000 elements, but using `with_capacity` cut this nearly in half to ~86 µs. The lesson is clear: when you know your approximate size, always pre-allocate.

`BTreeMap` and `BTreeSet` insertions took around 372-377 µs for 10,000 elements - roughly twice as slow as hash collections. This is the cost of maintaining sorted order during insertion, as the B-tree must find the correct position and potentially rebalance.

`BinaryHeap` insertions were surprisingly fast at ~59 µs for 10,000 elements, faster than both hash and tree collections. This is because maintaining heap order is less strict than maintaining full sorted order - only the parent-child relationship matters.

**Lookup Performance**

The lookup benchmarks dramatically illustrate how complexity classes translate to real performance differences. At 100,000 elements:

`Vec::contains` (linear search) took approximately 3.9 µs - scanning through 100,000 elements to find the target at the end. At 1,000,000 elements, this would take proportionally longer.

`HashSet::contains` remained constant at approximately 6.2 ns regardless of collection size. Whether searching 100 elements or 100,000, the time stayed essentially the same. This is O(1) in action.

`BTreeSet::contains` showed the logarithmic growth pattern: 7.1 ns at 100 elements, 11.2 ns at 1,000, 12.4 ns at 10,000, and 16.7 ns at 100,000. Each 10x increase in size added only about 2-4 ns - the hallmark of O(log n) behavior.

`Vec::binary_search` on sorted data performed similarly to `BTreeSet`, growing from 3.6 ns to 11.5 ns as size increased from 100 to 100,000. This confirms that if your data is already sorted and you need fast lookups, binary search is competitive with tree-based structures.

The crossover point is notable: at 100 elements, linear search (4.5 ns) actually beats hash lookup (5.9 ns) because the overhead of computing the hash exceeds the cost of scanning a small array. But by 1,000 elements, hash lookup is 7x faster, and by 100,000 elements, it's 630x faster.

**Front Operations**

This benchmark group provides the most dramatic demonstration of algorithmic complexity in the entire suite.

`Vec::insert(0, x)` for 1,000 elements took approximately 14 µs. Each insertion shifts all existing elements right, making this O(n) per operation and O(n²) for n insertions. We intentionally limited this benchmark to smaller sizes because it becomes prohibitively slow.

`VecDeque::push_front` for the same 1,000 elements took only 0.9 µs - about 15x faster. At 100,000 elements, `VecDeque` completed in 79.7 µs, while `Vec::insert(0, x)` for just 1,000 elements was already slower at 14 µs. Extrapolating `Vec` to 100,000 elements would take roughly 1.4 seconds!

This is the textbook case for choosing the right data structure. If your algorithm requires frequent front insertions, `VecDeque` isn't just faster - it's the difference between a usable program and an unusable one.

**Iteration and Cache Locality**

The iteration benchmarks reveal the hidden cost of memory layout. All collections iterated over 100,000 elements:

`Vec` and `VecDeque` both completed in approximately 3.5 µs, processing at roughly 28 billion elements per second. This extraordinary speed comes from sequential memory access - the CPU prefetches the next cache line while processing the current one.

`BinaryHeap::iter` matched this speed at 3.5 µs because internally it's stored as a `Vec`. Remember though: this iteration is NOT in sorted order!

`HashSet` iteration took approximately 88.6 µs - about 25x slower than `Vec`. The hash table's bucket structure means elements are scattered in memory, causing more cache misses.

`BTreeSet` iteration took approximately 105.6 µs. While tree nodes have good internal locality, traversing between nodes still causes more cache misses than contiguous storage.

`LinkedList` was slowest at approximately 113 µs. Each node could be anywhere in memory, causing frequent cache misses. The 32x slowdown compared to `Vec` demonstrates why linked lists are rarely the right choice for iteration-heavy workloads.

**Range Queries**

The range query benchmarks highlight `BTreeMap`'s unique advantage:

`BTreeMap::range` retrieved approximately 5,000 elements (the middle 50% of 10,000) in 4.9 µs.

`HashMap` with filtering took 9.2 µs for the same result - nearly twice as slow. The hash collection must scan all 10,000 entries and check each one against the range predicate.

Similarly, `BTreeSet::range` at 4.8 µs outperformed `HashSet` filtering at 8.6 µs.

This 2x difference would grow with larger datasets and smaller result sets. If you need to query "all entries between X and Y" frequently, `BTreeMap` pays for its higher insertion cost with much faster range operations.

**Entry API**

The Entry API benchmarks quantify a common performance mistake:

`HashMap` with the Entry API processed 10,000 word counting operations in approximately 91.8 µs.

`HashMap` using `contains_key` followed by `get_mut` or `insert` took approximately 185.2 µs - more than twice as slow!

The reason is simple: the approach performs two hash lookups per word (one to check existence, one to access/insert), while the Entry API performs just one. For hot paths that frequently update map entries, this 2x difference adds up quickly.

`BTreeMap` with Entry API took approximately 101 µs, about 10% slower than `HashMap`'s Entry API due to tree traversal versus hashing.

**Removal Operations**

The removal benchmarks show another dimension of collection performance:

`Vec::pop` (from back) was blazingly fast at 7.6 ns for 1,000 elements - just decrementing a length counter and returning the last element.

`VecDeque::pop_front` took 263 ns - still fast O(1) behavior, but with more bookkeeping than `Vec::pop`.

`Vec::remove(0)` (from front) took 13.8 µs - each removal shifts all remaining elements, making this O(n) per operation.

`HashMap::remove` took 8.9 µs for removing all 1,000 elements, while `BTreeMap::remove` took 12.6 µs. The hash-based collection wins on removal speed due to O(1) versus O(log n) lookups.

`BinaryHeap::pop` (all elements) took 16.9 µs. Each `pop` requires O(log n) work to restore the heap property, making this slower than hash removal but still efficient for priority queue operations.

**Scaling Behavior**

The scaling benchmarks provide the clearest visualization of complexity classes:

`Vec` linear search scaled predictably: 4.5 ns at 100 elements, 40.6 ns at 1,000, 392 ns at 10,000, 3.9 µs at 100,000, and 62.2 µs at 1,000,000. Each 10x increase in size caused a 10x increase in lookup time - the signature of O(n).

`HashSet` O(1) lookup remained flat: approximately 5.9-6.2 ns regardless of size. The line is essentially horizontal on a performance graph. At 1,000,000 elements, hash lookup is 10,000x faster than linear search.

`BTreeSet` O(log n) lookup grew slowly: 7.1 ns at 100, 11.2 ns at 1,000, 12.3 ns at 10,000, 16.7 ns at 100,000, and 16.5 ns at 1,000,000. The growth slows as size increases - a 10,000x increase in size (from 100 to 1M) only tripled the lookup time.

These results vividly demonstrate why algorithm complexity matters. The difference between O(1) and O(n) is negligible at small sizes but becomes enormous at scale.

#### Performance summary table

Based on the benchmark results, here are the relative performance characteristics for each collection type. Times are representative measurements from our benchmarks; actual performance will vary based on hardware, data types, and workload patterns.

| Operation | Vec      | VecDeque | LinkedList | HashMap    | BTreeMap | HashSet | BTreeSet | BinaryHeap |
|-----------|----------|----------|------------|------------|----------|--------|----------|------------|
| **Insert (10k elements)** | 6.8 µs   | 7.4 µs | 190 µs | 196 µs     | 376 µs | 192 µs | 373 µs | 59 µs      |
| **Insert w/ capacity** | 5.6 µs   | — | — | 86 µs      | — | — | — | —          |
| **Push front (1k)** | 14 µs*   | 0.9 µs | 18.6 µs | —          | — | — | — | —          |
| **Lookup (100k elements)** | 3.9 µs** | — | — | 6.5 ns     | 16 ns | 6.2 ns | 16.7 ns | —          |
| **Binary search (100k)** | 11.5 ns  | — | — | —          | — | — | — | —          |
| **Iteration (100k)** | 3.6 µs   | 3.5 µs | 113 µs | —          | — | 88.6 µs | 105.6 µs | 3.5 µs***  |
| **Range query (5k of 10k)** | —        | — | — | 9.2 µs**** | 4.9 µs | 8.6 µs**** | 4.8 µs | —          |
| **Remove from back (1k)** | 7.6 ns   | 263 ns | — | —          | — | — | — | —          |
| **Remove from front (1k)** | 13.8 µs  | 263 ns | — | —          | — | — | — | —          |
| **Remove by key (1k)** | —        | — | — | 8.9 µs     | 12.6 µs | — | — | —          |
| **Pop all (1k)** | —        | — | — | —          | — | — | — | 16.9 µs    |
| **Entry API (10k words)** | —        | — | — | 91.8 µs    | 101 µs | — | — | —          |

*Notes:*
- \* Vec insert at front is O(n) per operation - shown time is for 1k elements, not 10k
- \** Vec lookup uses linear search O(n); binary search requires sorted data
- \*** BinaryHeap iteration is NOT in sorted order
- \**** Hash collections use filter, not native range query

**Complexity Reference:**

| Collection | Insert | Lookup             | Remove | Iteration | Special |
|------------|--------|--------------------|--------|-----------|---------|
| **Vec** | O(1)* | O(n) or O(log n)** | O(1) back, O(n) front | O(n), cache-friendly | Random access O(1) |
| **VecDeque** | O(1)* both ends | O(n) or O(log n)** | O(1) both ends | O(n), cache-friendly | Double-ended queue |
| **LinkedList** | O(1) both ends | O(n)               | O(1) at cursor | O(n), cache-unfriendly | O(1) split/append |
| **HashMap** | O(1)* | O(1)*              | O(1)* | O(capacity) | Entry API |
| **BTreeMap** | O(log n) | O(log n)           | O(log n) | O(n), sorted | Range queries O(log n + k) |
| **HashSet** | O(1)* | O(1)*              | O(1)* | O(capacity) | Set operations |
| **BTreeSet** | O(log n) | O(log n)           | O(log n) | O(n), sorted | Range queries, min/max O(log n) |
| **BinaryHeap** | O(log n)* | O(n)               | O(log n) pop max | O(n), not sorted | Peek max O(1) |

*Notes:*
- \* Amortized - occasional O(n) for reallocation or rehashing
- \** O(log n) with binary_search on sorted data

**Key insights from benchmarks:**

1. **Pre-allocation matters**: `Vec::with_capacity` and `HashMap::with_capacity` can improve insertion performance by 20-50% when you know your data size.

2. **Cache locality dominates iteration**: `Vec`, `VecDeque`, and `BinaryHeap` iterate 25-30x faster than `LinkedList` and hash/tree collections for large datasets.

3. **Complexity class trumps constant factors at scale**: At 1M elements, `HashSet` lookup (6.2 ns) is 10,000x faster than `Vec` linear search (62 µs).

4. **Use the Entry API**: It's 2x faster than the `contains_key` + `insert` pattern for maps.

5. **BTree collections excel at range queries**: 2x faster than filtering hash collections, and the advantage grows with larger datasets.

6. **VecDeque is essential for front operations**: 15-1000x faster than `Vec::insert(0, x)` depending on size.

7. **LinkedList is rarely optimal**: It loses on insertion (allocation overhead), iteration (cache misses), and memory (pointer storage). Use it only for O(1) split/append operations.