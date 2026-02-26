// reverse_string.ts

/**
 * Reverses a string using built-in array methods.
 * Time Complexity: O(n)
 * Space Complexity: O(n)
 */
export function reverseString(s: string): string {
    return s.split("").reverse().join("");
}

// Testing
const input = "hello world";
console.log(`Original: ${input}`);
console.log(`Reversed: ${reverseString(input)}`);
