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
