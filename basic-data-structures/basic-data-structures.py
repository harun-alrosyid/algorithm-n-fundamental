# === Arrays (Lists in Python) ===

names = ["Alice", "Bob", "Charlie", "Diana"]

# O(1) Access
print("Array Access:", names[2])

# O(n) Search
def search_array(arr, target):
    for item in arr:
        if item == target:
            return True
    return False

print("Array Search:", search_array(names, "Bob"))


# === Hash Tables (Dictionaries in Python) ===

phone_book = {}

# O(1) Average - Insertion
phone_book["Alice"] = "555-0192"
phone_book["Bob"] = "555-8832"

# O(1) Average - Access
print("Hash Table Access:", phone_book.get("Alice"))


# === Stacks (LIFO) ===
# Python lists can be used directly as stacks (append/pop are O(1) at the end)

stack = []
stack.append(10) # Push O(1)
stack.append(20)

print("Stack Peek:", stack[-1]) # Peek O(1)
print("Stack Pop:", stack.pop()) # Pop O(1)


# === Queues (FIFO) ===
from collections import deque

# Python lists are slow for queues; deque is optimized for O(1) appends/pops on both ends
queue = deque()
queue.append("Alice") # Enqueue O(1)
queue.append("Bob")

print("Queue Peek:", queue[0]) # Peek O(1)
print("Queue Dequeue:", queue.popleft()) # Dequeue O(1)
