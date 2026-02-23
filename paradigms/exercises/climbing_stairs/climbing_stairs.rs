/**
 * Problem: Climbing Stairs
 */

fn climb_stairs(n: i32) -> i32 {
    if n <= 2 {
        return n;
    }

    let mut two_steps_before = 1;
    let mut one_step_before = 2;

    for _ in 3..=n {
        let current = one_step_before + two_steps_before;
        two_steps_before = one_step_before;
        one_step_before = current;
    }

    one_step_before
}

fn main() {
    let test_n = 5;
    let result = climb_stairs(test_n);

    println!("Steps: {}, Distinct ways: {}", test_n, result);

    if climb_stairs(2) == 2 && climb_stairs(3) == 3 && result == 8 {
        println!("Test Passed! ✅");
    } else {
        println!("Test Failed! ❌");
    }
}
