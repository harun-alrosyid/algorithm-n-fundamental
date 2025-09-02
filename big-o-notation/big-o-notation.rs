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

// Big O(log n)
    let price_list = [20000, 45000, 60000, 90000, 120000, 160000, 200000];
    let threshold_free_delivery = 150000;
    let first_price_gte = |price_list: [i32; 7], threshold_free_delivery: i32| {
        let mut start = 0;
        let mut end = price_list.len() - 1;
        let mut ans: i32 = -1
        while start <= end { 
             let mut mid = ((start + end) / 2).floor();
            if price_list[mid] >= threshold_free_delivery {
                ans = mid;
                end = mid - 1;
            } else {
                start = mid + 1;
            }
            mid = (start + end) / 2;
        }
        return price_list[ans];
    };
    println!("{}", first_price_gte(price_list, threshold_free_delivery));
    // output : 2

// Big O(n^2)
    let tops = ["t-shirt", "shirt", "sweater"];
    let bottoms = ["jeans", "pants", "shorts"];
    let outfit_combo = |tops: [&str], bottoms: [&str]| {
        let mut combos = Vec::new();
        for t in tops {
            for b in bottoms {
                combos.push([t, b]);
            }
        }
        return combos;
    };
    println!("{}", outfit_combo(tops, bottoms));
    // output :
    // [[t-shirt jeans] [t-shirt pants] [t-shirt shorts] [shirt jeans] [shirt pants] [shirt shorts] [sweater jeans] [sweater pants] [sweater shorts]]
}