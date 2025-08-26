fn main() {
// Big O(1)
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