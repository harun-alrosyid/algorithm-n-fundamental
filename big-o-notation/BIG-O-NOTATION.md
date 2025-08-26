# Big O Notation
Big o notation is how to measure algothm, how  perfomance or time excute some algorithm can as it increaes input. 

## O(1) waktu konstan
is an operation that runs regardless of the input, regardless of the size of the input.

Example access value from array using index

```typescript
// TypeScript

const data : number[] = [1,2,3,4,5]

function getValueItem(n:number):number{
    return data[n]
}

console.log(getValueItem(2))
// output : 3
console.log(getValueItem(4))
// output : 5

```
```go
// Golang
package main

import "fmt"

var data = []int{1, 2, 3, 4, 5}

func getValueItem(n int) int {
	return data[n]
}

func main() {

	fmt.Println(getValueItem(2))
	// output : 3
	fmt.Println(getValueItem(4))
	// output : 5
}

``` 
```python
# Python
data = [1, 2, 3, 4, 5]

def getValueItem(n):
    return data[n]

print(getValueItem(2))
# output : 3
print(getValueItem(4))
# output : 5
```
```rust
// Rust
fn main() {

  let numbers = [1, 2, 3, 4, 5];
 

    let get_value_item = |n: usize| {
        match numbers.get(n) {
            Some(val) => println!("{}", val),
        }
    };

    get_value_item(2);
    // output : 3
    get_value_item(4);
    // output : 5
    
}
```
