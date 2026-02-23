// === Arrays ===

const names: string[] = ["Alice", "Bob", "Charlie", "Diana"];

// O(1) Access
console.log("Array Access:", names[2]);

// O(n) Search
function searchArray(arr: string[], target: string): boolean {
  for (let i = 0; i < arr.length; i++) {
    if (arr[i] === target) return true;
  }
  return false;
}
console.log("Array Search:", searchArray(names, "Bob"));

// === Hash Tables (Maps) ===

const phoneBook = new Map<string, string>();

// O(1) Average - Insertion
phoneBook.set("Alice", "555-0192");
phoneBook.set("Bob", "555-8832");

// O(1) Average - Access
console.log("Hash Table Access:", phoneBook.get("Alice"));

// === Stacks (LIFO) ===

class Stack<T> {
  private items: T[] = [];

  push(element: T): void {
    this.items.push(element); // O(1)
  }

  pop(): T | undefined {
    return this.items.pop(); // O(1)
  }

  peek(): T | undefined {
    return this.items[this.items.length - 1]; // O(1)
  }
}

const myStack = new Stack<number>();
myStack.push(10);
myStack.push(20);
console.log("Stack Peek:", myStack.peek());
console.log("Stack Pop:", myStack.pop());

// === Queues (FIFO) ===

class Queue<T> {
  private items: T[] = [];

  enqueue(element: T): void {
    this.items.push(element); // O(1)
  }

  dequeue(): T | undefined {
    // Note: Array shift is O(n). Real queues use Linked Lists for O(1) Dequeue.
    return this.items.shift();
  }

  peek(): T | undefined {
    return this.items[0]; // O(1)
  }
}

const myQueue = new Queue<string>();
myQueue.enqueue("Alice");
myQueue.enqueue("Bob");
console.log("Queue Peek:", myQueue.peek());
console.log("Queue Dequeue:", myQueue.dequeue());
