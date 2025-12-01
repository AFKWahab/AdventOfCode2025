use std::io::{self, BufRead};

fn main() {
    let mut current_safe_dial: i32 = 50;
    let mut zero_count: i32 = 0;

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("Failed to read line");
        match line.chars().next() {
            Some('L') => {
                let num: i32 = line[1..].parse().expect("Failed to parse the number");
                current_safe_dial = subtract_number(current_safe_dial, num);
            }
            Some('R') => {
                let num: i32 = line[1..].parse().expect("Failed to parse the number");
                current_safe_dial = add_number(current_safe_dial, num);
            }
            _ => {}
        }
        if current_safe_dial == 0 {
            zero_count += 1;
        }
    }

    println!("{}", zero_count);
}

pub fn subtract_number(current_number: i32, subtraction_number: i32) -> i32 {
    (current_number - subtraction_number).rem_euclid(100)
}

pub fn add_number(current_number: i32, addition_number: i32) -> i32 {
    (current_number + addition_number).rem_euclid(100)
}
