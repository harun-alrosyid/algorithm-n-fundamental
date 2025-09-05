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

    
// Big O(n)
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

// Big O(log n)
    let price_list = [20000, 45000, 60000, 90000, 120000, 160000, 200000];
    let threshold_free_delivery = 150000;
    let first_price_gte = |price_list: [i32; 7], threshold: i32| {
        let mut start: i32 = 0;
        let mut end: i32 = price_list.len() as i32 - 1;
        let mut ans: usize = 0; 

        while start <= end {
            let mid = (start + end) / 2;
            let mid_idx = mid as usize;

            if price_list[mid_idx] >= threshold {
                ans = mid_idx;
                if mid == 0 { break; } 
                end = mid - 1;
            } else {
                start = mid + 1;
            }
        }

        price_list[ans]
    };

    println!("{}", first_price_gte(price_list, threshold_free_delivery));
    // output : 160000

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


// Big O(2^n)

    let simulate_pandemic = |generations: i32, reproduction_number: i32| {
        let mut total_infected = 1;
        let mut newly_infected = 1;
        let mut detail = Vec::new();

        for _ in 0..generations {
            newly_infected = newly_infected * reproduction_number;
            total_infected += newly_infected;
            detail.push(newly_infected);
            if total_infected >= 1_000_000 {
                break;
            }
        }
        (total_infected, detail)
    };

    let (total, detail) = simulate_pandemic(10, 2);
    println!("Total infected: {}", total);
    println!("Details: {:?}", detail);

// output :
// Total infected: 2047
// Details: [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024]

}