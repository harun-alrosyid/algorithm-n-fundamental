/**
 * Problem: Valid Anagram
 * Given two strings s and t, return true if t is an anagram of s, and false otherwise.
 *
 * Complexity:
 * - Time: O(n) - We iterate through both strings once.
 * - Space: O(1) - Although we use a frequency map, the space is limited by the alphabet size (e.g., 26 letters), which is constant.
 *
 * Memory Model:
 * - 's' and 't' are references to strings in the Heap.
 * - 'count' is a reference to a Map in the Heap.
 */

function isAnagram(s: string, t: string): boolean {
    if (s.length !== t.length) return false;

    const count = new Map<string, number>();

    for (let i = 0; i < s.length; i++) {
        count.set(s[i], (count.get(s[i]) || 0) + 1);
        count.set(t[i], (count.get(t[i]) || 0) - 1);
    }

    for (let val of count.values()) {
        if (val !== 0) return false;
    }

    return true;
}

// Test cases
const s1 = "anagram", t1 = "nagaram";
const s2 = "rat", t2 = "car";

console.log(`s1: "${s1}", t1: "${t1}" -> Result: ${isAnagram(s1, t1)}`); // true
console.log(`s2: "${s2}", t2: "${t2}" -> Result: ${isAnagram(s2, t2)}`); // false

if (isAnagram(s1, t1) === true && isAnagram(s2, t2) === false) {
    console.log("Test Passed! ✅");
} else {
    console.log("Test Failed! ❌");
}
