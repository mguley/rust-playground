mod hashmap_examples;
mod linked_list_examples;
mod vec_examples;
mod vecdeque_examples;

use hashmap_examples::{
    accessing_values, basic_hashmap_operations, creating_hashmaps, custom_keys, entry_api_examples,
    iterating_hashmaps, ownership_and_borrowing, removing_values,
};
use linked_list_examples::{
    append_and_split, basic_linked_list_operations, compare_linked_list, cursor_example,
    linked_list_iteration,
};
use vec_examples::{
    accessing_elements, basic_vec_operations, capacity_demonstration, modifying_vectors,
    slicing_vectors,
};
use vecdeque_examples::{
    basic_vecdeque_operations, fifo_queue_example, ring_buffer_demonstration,
    sliding_window_example,
};

use rustc_version_runtime;

fn main() {
    println!("Rust Collections Demo");
    println!("Compiled with: {:?}", rustc_version_runtime::version());

    // We'll call our example functions here
    // run_vec_examples();

    // run_vecdeque_examples();

    // run_linked_list_examples();

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

fn run_linked_list_examples() {
    section("basic_linked_list_operations", basic_linked_list_operations);
    section("append_and_split", append_and_split);
    section("linked_list_iteration", linked_list_iteration);
    section("cursor_example", cursor_example);
    section("compare_linked_list", compare_linked_list);
}

fn run_vecdeque_examples() {
    section("basic_vecdeque_operations", basic_vecdeque_operations);
    section("fifo_queue_example", fifo_queue_example);
    section("sliding_window_example", sliding_window_example);
    section("ring_buffer_demonstration", ring_buffer_demonstration);
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
