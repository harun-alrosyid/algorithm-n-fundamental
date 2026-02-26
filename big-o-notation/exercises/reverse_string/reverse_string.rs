fn reverse_string(s: &str) -> String {
    // Time Complexity: O(n)
    // Space Complexity: O(n)
    s.chars().rev().collect()
}

fn main() {
    let input = "hello world";
    let reversed = reverse_string(input);
    println!("Original: {}", input);
    println!("Reversed: {}", reversed);
}
