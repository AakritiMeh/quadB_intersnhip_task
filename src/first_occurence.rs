use std::io::{self, Write};
fn find_first_occurrence(nums: &Vec<i32>, target: i32) -> Option<usize> {
    for (index, &num) in nums.iter().enumerate() {
        if num == target {
            return Some(index);
        }
    }
    None
}

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

    print!("Enter the number to find: ");
    io::stdout().flush().unwrap();
    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let target: i32 = input.trim().parse().unwrap();

    let result = find_first_occurrence(&nums, target);
    println!("Index of first occurrence of {}: {:?}", target, result);
}
