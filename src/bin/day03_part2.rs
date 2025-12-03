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
        let best_value: i64 = best_k_digit_value(&nums, 12);
        all_best_values.push(best_value);
    }
    // Accumulating all values
    let result: i64 = all_best_values.iter().sum();
    println!("The sum of all best two-digit values is: {}", result);
}

fn best_k_digit_value(digits: &[i64], k: usize) -> i64 {
    let n = digits.len();
    let removals_allowed = n - k;

    let mut stack: Vec<i64> = Vec::with_capacity(k);
    let mut removals = removals_allowed;

    for &digit in digits {
        while let Some(&top) = stack.last() {
            if removals > 0 && digit > top {
                stack.pop();
                removals -= 1;
            } else {
                break;
            }
        }
        stack.push(digit);
    }

    stack.truncate(k);

    let mut value: i64 = 0;

    for &digit in &stack {
        value = value * 10 + digit;
    }

    value
}
