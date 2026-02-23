use std::collections::HashMap;

/**
 * Problem: Two Sum
 * Complexity:
 * - Time: O(n)
 * - Space: O(n)
 *
 * Memory Model:
 * - 'nums' is a Vec<i32>, which is stored in the Heap.
 * - 'prev_map' is a HashMap, also stored in the Heap.
 * - Rust's ownership system ensures no memory leaks or data races.
 */

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut prev_map = HashMap::new(); // val : index

    for (i, &n) in nums.iter().enumerate() {
        let diff = target - n;
        if let Some(&prev_idx) = prev_map.get(&diff) {
            return vec![prev_idx as i32, i as i32];
        }
        prev_map.insert(n, i);
    }
    vec![]
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = two_sum(nums.clone(), target);

    println!("Input: nums = {:?}, target = {}", nums, target);
    println!("Output: {:?}", result);

    if result == vec![0, 1] {
        println!("Test Passed! ✅");
    } else {
        println!("Test Failed! ❌");
    }
}
