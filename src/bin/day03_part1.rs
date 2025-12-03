use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut all_best_values: Vec<i64> = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.expect("Failed to read line");
        let nums: Vec<i64> = line
            .trim()
            .chars()
            .map(|c| c.to_digit(10).expect("Failed to convert char to digit") as i64)
            .collect();
        let best_value: i64 = best_two_digit_value(&nums);
        all_best_values.push(best_value);
    }
    // Accumulating all values
    let result: i64 = all_best_values.iter().sum();
    println!("The sum of all best two-digit values is: {}", result);
}

fn best_two_digit_value(digits: &[i64]) -> i64 {
    let mut best_tens_so_far = digits[0];
    let mut best_value = -1;
    let len = digits.len();
    for i in 1..len {
        let value = digits[i];
        let candidate: i64 = 10 * best_tens_so_far + value;
        if candidate > best_value {
            best_value = candidate;
        }
        if value > best_tens_so_far {
            best_tens_so_far = value
        }
    }
    best_value
}
