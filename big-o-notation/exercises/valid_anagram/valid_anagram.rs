use std::collections::HashMap;

/**
 * Problem: Valid Anagram
 * Complexity:
 * - Time: O(n)
 * - Space: O(1) - fixed alphabet size
 */

fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut count = HashMap::new();

    for c in s.chars() {
        *count.entry(c).or_insert(0) += 1;
    }

    for c in t.chars() {
        let entry = count.entry(c).or_insert(0);
        *entry -= 1;
        if *entry < 0 {
            return false;
        }
    }

    true
}

fn main() {
    let s1 = "anagram".to_string();
    let t1 = "nagaram".to_string();
    let s2 = "rat".to_string();
    let t2 = "car".to_string();

    println!("s1: \"{}\", t1: \"{}\" -> Result: {}", s1, t1, is_anagram(s1.clone(), t1.clone()));
    println!("s2: \"{}\", t2: \"{}\" -> Result: {}", s2, t2, is_anagram(s2.clone(), t2.clone()));

    if is_anagram(s1, t1) == true && is_anagram(s2, t2) == false {
        println!("Test Passed! ✅");
    } else {
        println!("Test Failed! ❌");
    }
}
