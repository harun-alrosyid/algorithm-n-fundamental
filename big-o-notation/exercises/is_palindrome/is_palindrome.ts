// is_palindrome.ts

/**
 * Checks if a string is a palindrome.
 * Time Complexity: O(n)
 * Space Complexity: O(n)
 */
export function isPalindrome(s: string): boolean {
    const normalized = s.toLowerCase().replace(/[^a-z0-9]/g, "");
    const reversed = normalized.split("").reverse().join("");
    return normalized === reversed;
}

/**
 * Optimized Two-Pointer approach.
 * Time Complexity: O(n)
 * Space Complexity: O(1)
 */
export function isPalindromeOptimized(s: string): boolean {
    const normalized = s.toLowerCase().replace(/[^a-z0-9]/g, "");
    let left = 0;
    let right = normalized.length - 1;

    while (left < right) {
        if (normalized[left] !== normalized[right]) {
            return false;
        }
        left++;
        right--;
    }
    return true;
}

// Testing
const testCases = ["Racecar", "hello", "A man, a plan, a canal: Panama"];
testCases.forEach((tc) => {
    console.log(`Is "${tc}" a palindrome? ${isPalindromeOptimized(tc)}`);
});
