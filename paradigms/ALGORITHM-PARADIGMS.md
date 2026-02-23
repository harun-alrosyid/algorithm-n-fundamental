### Table of Contents

- [Recursion vs Iteration](#recursion-vs-iteration)
  - [Stack vs Loop](#stack-vs-loop)
- [Divide & Conquer](#divide--conquer)
- [Greedy Algorithms](#greedy-algorithms)
- [Dynamic Programming (DP)](#dynamic-programming-dp)
  - [Memoization (Top-Down)](#memoization-top-down)
  - [Tabulation (Bottom-Up)](#tabulation-bottom-up)

# Algorithm Paradigms

Algorithm paradigms are high-level strategies or blueprints used to solve problems. Choosing the right paradigm can drastically change your algorithm's efficiency.

---

## Recursion vs Iteration

Most problems can be solved either way, but they use memory and the CPU differently.

### Recursion
A function that calls itself to solve a smaller sub-problem.
- **Mental Model**: "To solve for N, I first need the solution for N-1."
- **Memory**: Uses the **Call Stack**. Each call adds a "frame" to the stack. Too many calls lead to a `Stack Overflow`.

### Iteration
Using loops (`for`, `while`) to repeat an operation.
- **Mental Model**: "Start at 0 and keep going until I reach N."
- **Memory**: Uses a fixed amount of memory on the Stack (just the loop counter and variables).

> [!IMPORTANT]
> **Base Case**: Every recursive function *must* have a condition to stop, or it will run forever (until it crashes).

---

## Divide & Conquer

Break a problem into independent sub-problems, solve them, and combine the results.

1. **Divide**: Split the problem into smaller pieces.
2. **Conquer**: Solve the pieces (often using recursion).
3. **Combine**: Join the pieces back together to get the final answer.

**Examples**: Merge Sort, Quick Sort, Binary Search.

---

## Greedy Algorithms

Make the **best local choice** at each step, hoping it leads to the best global solution.

- **Greedy Choice**: "What is the best thing I can do *right now*?"
- **Pros**: Usually very fast (often O(n)).
- **Cons**: Doesn't always find the perfect solution (e.g., finding the shortest path in some graphs).

---

## Dynamic Programming (DP)

DP is an optimization technique for problems with **overlapping sub-problems**. Instead of re-calculating the same thing 100 times, you store the result.

### Memoization (Top-Down)
Usually implemented with **Recursion**. You check a cache (Map/Array) before doing any work.
> "If I've seen this input before, return the saved answer."

### Tabulation (Bottom-Up)
Usually implemented with **Iteration**. You build a table (usually an array) from the smallest sub-problem up to the largest.
> "Fill in the easy answers first, then use them to find the harder ones."

---

## Exercises & Examples
- [Factorial & Fibonacci](file:///Users/harunalrosyid/Documents/Opensource/algorithm-n-fundamental/paradigms/) - Recursion vs Iteration compared.
- [Climbing Stairs](file:///Users/harunalrosyid/Documents/Opensource/algorithm-n-fundamental/big-o-notation/paradigms/exercises/climbing_stairs/) - A classic introduction to DP.

More foundational topics: [Big-O Notation](file:///Users/harunalrosyid/Documents/Opensource/algorithm-n-fundamental/big-o-notation/BIG-O-NOTATION.md)
