# Big O Notation
Big o notation is how to measure algothm, how  perfomance or time excute some algorithm can as it increaes input. 

## O(1) Constant Time
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

## O(n) Linear Time
is a operation runs with propotional to the size of the input.

Example : find maximum value in array



```typescript
// TypeScript

const data : number[] = [1,2,6,4,5,9,0,3]

function getMaxValue(data:number[]):number{	
	let max :number =data[1]
    
    for (let i=0; i < data.length ;i++ ){
    
        if(data[i] > max){
            max =data[i]
        }
    }


    return max
}

console.log(getMaxValue(data))
// output : 9

```
```go
// Golang
package main

import "fmt"

var data1 = []int{1,2,6,4,5,9,0,3}

func getMaxValue(data []int) int {
	max := data[0]
	for _, val := range data {
		if val > max {
			max = val
		}
	}
	return max
}

func main() {

	fmt.Println(getMaxValue(data1))
	// output : 5

}

``` 
```python
# Python
def getMaxValue(data):
    max = data[0]
    for val in data:
        if val > max:
            max = val
    return max

print(getMaxValue(data1))
# output : 9
```
```rust
// Rust
fn main() {

    let data = [1,2,6,4,5,9,0,3];

    let get_max_value = |data: [i32; 8]| {
        let mut max = data[0];
        for val in data {
            if val > max {
                max = val;
            }
        }
        return max;
       
    };
    println!("{}",  get_max_value(data));
    // output : 9
    
}
```

