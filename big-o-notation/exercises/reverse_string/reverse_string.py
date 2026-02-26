def reverse_string(s: str) -> str:
    """
    Reverses a string using Python's slicing.
    Time Complexity: O(n)
    Space Complexity: O(n)
    """
    return s[::-1]

if __name__ == "__main__":
    test_str = "hello world"
    print(f"Original: {test_str}")
    print(f"Reversed: {reverse_string(test_str)}")
