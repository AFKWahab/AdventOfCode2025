use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read line");

    let ranges: Vec<&str> = buffer.trim().split(',').collect();
    let mut double_repeat_sequences: Vec<i64> = Vec::new();
    for range in ranges {
        let bounds: Vec<i64> = range
            .split('-')
            .map(|s| s.parse::<i64>().expect("Failed to parse number"))
            .collect();

        let (lower, upper) = (bounds[0], bounds[1]);

        for num in lower..=upper {
            let is_candidate_double_repeat = is_double_repeat(num);
            if is_candidate_double_repeat {
                double_repeat_sequences.push(num);
            }
        }
    }

    // Output the result
    if double_repeat_sequences.is_empty() {
        println!("No invalid IDs found.");
    } else {
        let total: i64 = double_repeat_sequences.iter().sum();
        println!("Total invalid IDs: {}", total);
    }
}

fn is_double_repeat(n: i64) -> bool {
    let number_as_string = n.to_string();
    let len = number_as_string.len();
    // Only check if the length is even (can be split into two equal parts)
    if len % 2 == 0 {
        let half = len / 2;
        let first = &number_as_string[0..half];
        let second = &number_as_string[half..];
        if first == second {
            return true;
        }
    }
    false
}