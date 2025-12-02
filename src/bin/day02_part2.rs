use std::io;

/*
Now, an ID is invalid if it is made only of some sequence of digits repeated at least twice. So, 12341234 (1234 two times), 123123123 (123 three times), 1212121212 (12 five times), and 1111111 (1 seven times) are all invalid IDs.

From the same example as before:
What do you get if you add up all of the invalid IDs using these new rules?
 */
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
            let is_candidate_double_repeat = is_multi_repeat(num);
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

fn is_multi_repeat(n: i64) -> bool {
    let number_as_string = n.to_string();
    let len = number_as_string.len();
    for sub_len in 1..=(len / 2) {
        if len % sub_len == 0 {
            let repeat_count = len / sub_len;
            if repeat_count >= 2 {
                let pattern = &number_as_string[0..sub_len];
                let repeated = pattern.repeat(repeat_count);
                if repeated == number_as_string {
                    return true;
                }
            }
        }
    }
    false
}