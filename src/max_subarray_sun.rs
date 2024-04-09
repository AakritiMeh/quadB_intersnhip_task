use std::io::{self, Write};

pub fn main() {
    let mut ip = String::new();
    print!("Enter array of integers (space-separated): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut ip).expect("Failed to read line");

    let nums: Vec<i32> = ip
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let max_sum = max_subarray_sum(&nums);
    println!("Maximum Subarray Sum: {}", max_sum);
}

fn max_subarray_sum(nums: &[i32]) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut curr_sum = nums[0];
    let mut max_sum = nums[0];

    for &num in nums.iter().skip(1) {
        curr_sum = curr_sum.max(num);
        max_sum = max_sum.max(curr_sum);
    }

    max_sum
}
