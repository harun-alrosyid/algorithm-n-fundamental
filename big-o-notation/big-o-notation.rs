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

    let data = [1,2,6,4,5,9,0,3];

// Big O(n)
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