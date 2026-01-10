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
