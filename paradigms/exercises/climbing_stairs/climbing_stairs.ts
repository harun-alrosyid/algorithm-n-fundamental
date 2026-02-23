/**
 * Problem: Climbing Stairs
 * You are climbing a staircase. It takes n steps to reach the top.
 * Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?
 * 
 * Logic:
 * This is effectively a Fibonacci sequence. To reach step N, you could have come from:
 * 1. Step N-1 (by taking 1 step)
 * 2. Step N-2 (by taking 2 steps)
 * So: ways[n] = ways[n-1] + ways[n-2]
 * 
 * Complexity:
 * - Time: O(n)
 * - Space: O(1)
 */

function climbStairs(n: number): number {
    if (n <= 2) return n;

    let oneStepBefore = 2; // ways to reach step 2
    let twoStepsBefore = 1; // ways to reach step 1
    let currentWays = 0;

    for (let i = 3; i <= n; i++) {
        currentWays = oneStepBefore + twoStepsBefore;
        twoStepsBefore = oneStepBefore;
        oneStepBefore = currentWays;
    }

    return currentWays;
}

// Test cases
const testN = 5;
const result = climbStairs(testN);

console.log(`Steps: ${testN}, Distinct ways: ${result}`); // Should be 8 (1,1,1,1,1 / 1,1,1,2 / 1,1,2,1 / 1,2,1,1 / 2,1,1,1 / 1,2,2 / 2,1,2 / 2,2,1)

if (climbStairs(2) === 2 && climbStairs(3) === 3 && climbStairs(testN) === 8) {
    console.log("Test Passed! ✅");
} else {
    console.log("Test Failed! ❌");
}
