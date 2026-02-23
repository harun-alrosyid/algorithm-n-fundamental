"""
Problem: Two Sum
Complexity:
- Time: O(n)
- Space: O(n)

Memory Model:
- Python handles everything as an object in the Heap.
- 'nums' is a reference to a list.
- 'prevMap' is a reference to a dict.
- Variables like 'diff' or 'num' are references to immutable objects (integers).
"""

def two_sum(nums, target):
    prevMap = {} # val : index

    for i, n in enumerate(nums):
        diff = target - n
        if diff in prevMap:
            return [prevMap[diff], i]
        prevMap[n] = i
    return []

if __name__ == "__main__":
    nums = [2, 7, 11, 15]
    target = 9
    result = two_sum(nums, target)

    print(f"Input: nums = {nums}, target = {target}")
    print(f"Output: {result}")

    if result == [0, 1]:
        print("Test Passed! ✅")
    else:
        print("Test Failed! ❌")
