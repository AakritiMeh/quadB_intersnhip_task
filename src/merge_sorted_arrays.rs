use std::io::{self, Write};

pub fn main() {
    let mut arr1 = String::new();
    print!("Enter first sorted array of integers (space-separated): ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut arr1)
        .expect("Failed to read line");

    let nums1: Vec<i32> = arr1
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut arr2 = String::new();
    print!("Enter second sorted array of integers (space-separated): ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut arr2)
        .expect("Failed to read line");

    let nums2: Vec<i32> = arr2
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let merged = merge_sorted_arrays(&nums1, &nums2);
    println!("Merged sorted array: {:?}", merged);
}

fn merge_sorted_arrays(nums1: &[i32], nums2: &[i32]) -> Vec<i32> {
    let mut merged = Vec::with_capacity(nums1.len() + nums2.len());
    let (mut i, mut j) = (0, 0);

    while i < nums1.len() && j < nums2.len() {
        if nums1[i] < nums2[j] {
            merged.push(nums1[i]);
            i += 1;
        } else {
            merged.push(nums2[j]);
            j += 1;
        }
    }

    while i < nums1.len() {
        merged.push(nums1[i]);
        i += 1;
    }

    while j < nums2.len() {
        merged.push(nums2[j]);
        j += 1;
    }

    merged
}
