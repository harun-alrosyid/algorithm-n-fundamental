fn is_palindrome(s: &str) -> bool {
    // Time Complexity: O(n)
    // Space Complexity: O(n) (for the reversed string)
    let normalized = s.to_lowercase().replace(|c: char| !c.is_alphanumeric(), "");
    let reversed: String = normalized.chars().rev().collect();
    normalized == reversed
}

fn is_palindrome_optimized(s: &str) -> bool {
    // Time Complexity: O(n)
    // Space Complexity: O(1) (ignoring the temporary lowercase string if needed)
    let normalized = s.to_lowercase();
    let chars: Vec<char> = normalized.chars().filter(|c| c.is_alphanumeric()).collect();

    if chars.is_empty() {
        return true;
    }

    let mut left = 0;
    let mut right = chars.len() - 1;

    while left < right {
        if chars[left] != chars[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
}

fn main() {
    let test_cases = vec!["Racecar", "hello", "A man, a plan, a canal: Panama"];
    for tc in test_cases {
        println!(
            "Is \"{}\" a palindrome? {}",
            tc,
            is_palindrome_optimized(tc)
        );
    }
}
