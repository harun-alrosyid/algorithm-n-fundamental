use std::collections::HashMap;
use std::collections::VecDeque;

fn main() {
    // === Arrays (Vectors in Rust) ===

    let names = vec!["Alice", "Bob", "Charlie", "Diana"];

    // O(1) Access
    println!("Array Access: {}", names[2]);

    // O(n) Search
    fn search_array(arr: &[&str], target: &str) -> bool {
        for &item in arr {
            if item == target {
                return true;
            }
        }
        false
    }
    println!("Array Search: {}", search_array(&names, "Bob"));


    // === Hash Tables (HashMaps in Rust) ===

    let mut phone_book = HashMap::new();

    // O(1) Average - Insertion
    phone_book.insert("Alice", "555-0192");
    phone_book.insert("Bob", "555-8832");

    // O(1) Average - Access
    if let Some(number) = phone_book.get("Alice") {
        println!("Hash Table Access: {}", number);
    }


    // === Stacks (LIFO) ===
    // Vec can be used naturally as a stack

    let mut stack = Vec::new();
    stack.push(10); // Push O(1)
    stack.push(20);

    if let Some(&top) = stack.last() {
        println!("Stack Peek: {}", top); // Peek O(1)
    }
    
    if let Some(popped) = stack.pop() {
        println!("Stack Pop: {}", popped); // Pop O(1)
    }


    // === Queues (FIFO) ===
    // VecDeque provides O(1) operations at both ends

    let mut queue = VecDeque::new();
    queue.push_back("Alice"); // Enqueue O(1)
    queue.push_back("Bob");

    if let Some(&front) = queue.front() {
        println!("Queue Peek: {}", front); // Peek O(1)
    }

    if let Some(dequeued) = queue.pop_front() {
        println!("Queue Dequeue: {}", dequeued); // Dequeue O(1)
    }
}
