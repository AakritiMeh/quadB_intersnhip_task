use std::io::{self, Write};

pub fn main() {
    let mut input = String::new();
    print!("Enter sorted array of integers (space-separated): ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let median = find_median(&nums);
    println!("Median: {}", median);
}

fn find_median(nums: &Vec<i32>) -> f64 {
    let n = nums.len();
    if n % 2 == 1 {
        nums[n / 2] as f64
    } else {
        let mid_right = nums[n / 2];
        let mid_left = nums[n / 2 - 1];
        (mid_left + mid_right) as f64 / 2.0
    }
}
