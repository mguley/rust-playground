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
