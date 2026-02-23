/**
 * Problem: Two Sum
 * Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
 *
 * Complexity:
 * - Time: O(n) - We only traverse the list once. Each lookup in the hash map is O(1).
 * - Space: O(n) - The extra space required depends on the number of items stored in the hash map.
 *
 * Memory Model:
 * - 'nums' is a reference to an array in the Heap.
 * - 'prevMap' is a reference to a Map object in the Heap.
 */

function twoSum(nums: number[], target: number): number[] {
    // Key: number, Value: index
    const prevMap = new Map<number, number>();

    for (let i = 0; i < nums.length; i++) {
        const diff = target - nums[i];

        if (prevMap.has(diff)) {
            return [prevMap.get(diff)!, i];
        }

        prevMap.set(nums[i], i);
    }

    return [];
}

// Test case
const nums = [2, 7, 11, 15];
const target = 9;
const result = twoSum(nums, target);

console.log(`Input: nums = [${nums}], target = ${target}`);
console.log(`Output: [${result}]`);

if (result[0] === 0 && result[1] === 1) {
    console.log("Test Passed! ✅");
} else {
    console.log("Test Failed! ❌");
}
