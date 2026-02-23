### Table of Contents

- [Basic Data Structures](#basic-data-structures)
  - [Arrays (Static & Dynamic)](#arrays-static--dynamic)
  - [Hash Tables (Maps/Dictionaries)](#hash-tables-mapsdictionaries)
  - [Stacks (LIFO)](#stacks-lifo)
  - [Queues (FIFO)](#queues-fifo)

# Basic Data Structures

Once you understand [Big O Notation](../big-o-notation/BIG-O-NOTATION.md), the next logical step is to understand how we store and retrieve data optimally. A Data Structure is simply a specialized format for organizing, processing, retrieving, and storing data.

Understanding data structures helps us answer the question: _"What is the most efficient way to access this data for my current use case?"_

## Arrays (Static & Dynamic)

An Array is a collection of items stored at contiguous memory locations. The idea is to store multiple items of the same type together, making it easy to calculate the position of each element.

### Operations and Big-O
- **Access:** `O(1)` (Constant Time - you know the exact index)
- **Search:** `O(n)` (Linear Time - you have to potentially look through everything)
- **Insertion/Deletion (at the end):** `O(1)`
- **Insertion/Deletion (at the middle/start):** `O(n)` (Because all subsequent items must be shifted over)

> Analogy:
>
> A row of mailboxes. If you know you need mailbox #4, you can go straight to it `O(1)`. But if you want to find who has the electric bill, you might have to check every single mailbox `O(n)`. If you force a new mailbox into the middle of the row, you have to shift all the following mailboxes to the right `O(n)`.

Example code (TypeScript):

```typescript
const names: string[] = ["Alice", "Bob", "Charlie", "Diana"]; // Array string

// O(1) - Access by index
console.log(names[2]); // Output: Charlie

// O(n) - Search
function searchArray(arr: string[], target: string): boolean {
  for (let i = 0; i < arr.length; i++) {
    if (arr[i] === target) return true;
  }
  return false;
}
console.log(searchArray(names, "Bob")); // Output: true

// O(1) - Insert at end
names.push("Eve"); 

// O(n) - Insert at beginning
names.unshift("Zach"); 
```

## Hash Tables (Maps/Dictionaries)

A Hash Table (or Dictionary, or Map depending on the language) stores data in key-value pairs. It uses a "hash function" to convert the key into an index, which then points to the location of the value in memory.

### Operations and Big-O
- **Access / Search:** Average `O(1)`, Worst case `O(n)` (If many keys hash to the same index, causing a "collision")
- **Insertion / Deletion:** Average `O(1)`, Worst case `O(n)`

> Analogy:
>
> A library's card catalog system (or an English Dictionary). Instead of scanning every book in the library linearly `O(n)`, you look up the word directly via its spelling (its "key") and instantly find its definition/location `O(1)`.

Example code (TypeScript):

```typescript
// Using Map in TypeScript
const phoneBook = new Map<string, string>();

// O(1) Average - Insertion
phoneBook.set("Alice", "555-0192");
phoneBook.set("Bob", "555-8832");

// O(1) Average - Access
console.log(phoneBook.get("Alice")); // Output: 555-0192

// O(1) Average - Deletion
phoneBook.delete("Bob");
```

## Stacks (LIFO)

A Stack is a linear data structure that follows the **LIFO (Last In, First Out)** principle. The last element added to the stack will be the first one to be removed.

### Operations and Big-O
- **Push (Insert):** `O(1)`
- **Pop (Delete):** `O(1)`
- **Peek/Top:** `O(1)`
- **Access / Search:** `O(n)` (Not meant for searching)

> Analogy:
>
> A stack of plates at a buffet. You can only easily take the top plate `Pop O(1)`. The kitchen staff places new clean plates on the very top `Push O(1)`. If you wanted the 5th plate down, you would have to move 4 plates out of the way first `Access O(n)`. Browser History (Back button) also uses a Stack.

Example code (TypeScript):

```typescript
class Stack<T> {
  private items: T[] = [];

  // O(1)
  push(element: T): void {
    this.items.push(element);
  }

  // O(1)
  pop(): T | undefined {
    return this.items.pop();
  }

  // O(1)
  peek(): T | undefined {
    return this.items[this.items.length - 1];
  }
}

const myStack = new Stack<number>();
myStack.push(10);
myStack.push(20);
console.log(myStack.peek()); // Output: 20
console.log(myStack.pop());  // Output: 20
```

## Queues (FIFO)

A Queue is a linear data structure that follows the **FIFO (First In, First Out)** principle. The first element added to the queue will be the first one removed.

### Operations and Big-O
- **Enqueue (Insert):** `O(1)`
- **Dequeue (Delete):** `O(1)`
- **Peek/Front:** `O(1)`
- **Access / Search:** `O(n)` (Not meant for searching)

> Analogy:
>
> The aforementioned supermarket checkout line. The first person in line is the first person served `Dequeue O(1)`. New people have to join the back of the line `Enqueue O(1)`.

Example code (TypeScript):

```typescript
class Queue<T> {
  private items: T[] = [];

  // O(1)
  enqueue(element: T): void {
    this.items.push(element);
  }

  // O(n) (Because shifting an array is O(n))
  // Note: A truly efficient Queue would be implemented with a Linked List to achieve O(1) Dequeue.
  dequeue(): T | undefined {
    return this.items.shift();
  }

  // O(1)
  peek(): T | undefined {
    return this.items[0];
  }
}

const myQueue = new Queue<string>();
myQueue.enqueue("Alice");
myQueue.enqueue("Bob");
console.log(myQueue.peek()); // Output: Alice
console.log(myQueue.dequeue()); // Output: Alice
```

---

More code examples : [TypeScript](https://github.com/harun-alrosyid/algorithm-n-fundamental/blob/main/basic-data-structures/basic-data-structures.ts) | [Go](https://github.com/harun-alrosyid/algorithm-n-fundamental/blob/main/basic-data-structures/basic-data-structures.go) | [Python](https://github.com/harun-alrosyid/algorithm-n-fundamental/blob/main/basic-data-structures/basic-data-structures.py) | [Rust](https://github.com/harun-alrosyid/algorithm-n-fundamental/blob/main/basic-data-structures/basic-data-structures.rs)
