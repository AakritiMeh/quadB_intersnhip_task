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

    print!("Enter kth value: ");
    io::stdout().flush().unwrap();
    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let k: usize = input.trim().parse().unwrap();

    let result = kth_smallest(&nums, k);
    println!("{}th Smallest Element: {:?}", k, result);
}

fn kth_smallest(nums: &Vec<i32>, k: usize) -> Option<i32> {
    if k > 0 && k <= nums.len() {
        Some(nums[k - 1])
    } else {
        None
    }
}
