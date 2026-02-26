def is_palindrome(s: str) -> bool:
    """
    Checks if a string is a palindrome using Python's slicing.
    Time Complexity: O(n)
    Space Complexity: O(n) (due to string creation during slicing)
    """
    # Normalize
    s = s.lower().replace(" ", "")
    return s == s[::-1]

def is_palindrome_optimized(s: str) -> bool:
    """
    Checks if a string is a palindrome using two pointers.
    Time Complexity: O(n)
    Space Complexity: O(1)
    """
    s = s.lower().replace(" ", "")
    left, right = 0, len(s) - 1
    while left < right:
        if s[left] != s[right]:
            return false
        left += 1
        right -= 1
    return True

if __name__ == "__main__":
    test_cases = ["Racecar", "hello", "A man a plan a canal Panama"]
    for tc in test_cases:
        print(f"Is '{tc}' a palindrome? {is_palindrome(tc)}")
