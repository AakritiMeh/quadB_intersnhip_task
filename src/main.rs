use std::io::{self, Write};

pub mod check_num_prime;
pub mod check_prime;
pub mod first_occurence;
pub mod kth_smallest_in_array;
pub mod longest_common_prefis;
pub mod max_depth_bt;
pub mod max_subarray_sun;
pub mod median_sortedarr;
pub mod merge_sorted_arrays;
pub mod palindrome;
pub mod shortest_word;
pub mod string_reverse;

fn main() {
    loop {
        println!("Menu:");
        println!("1. Check Palindrome");
        println!("2. Find First Occurrence in Sorted Array");
        println!("3. Find Shortest Word in a String");
        println!("4. Check if a Number is Prime");
        println!("5. Find Median of Sorted Array");
        println!("6. Find Longest Common Prefix of Strings");
        println!("7. Find Kth Smallest Element in an Array");
        println!("8. Find Maximum Depth of a Binary Tree");
        println!("9. Reverse a String");
        println!("10. Check if a Number is Prime");
        println!("11. Merge Two Sorted Arrays");
        println!("12. Find Maximum Subarray Sum");
        println!("0. Exit");

        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        match choice {
            1 => palindrome::main(),
            2 => first_occurence::main(),
            3 => shortest_word::main(),
            4 => check_prime::main(),
            5 => median_sortedarr::main(),
            6 => longest_common_prefis::main(),
            7 => kth_smallest_in_array::main(),
            8 => max_depth_bt::main(),
            9 => string_reverse::main(),
            10 => check_num_prime::main(),
            11 => merge_sorted_arrays::main(),
            12 => max_subarray_sun::main(),
            0 => {
                println!("Exiting program. Goodbye!");
                break;
            }
            _ => {
                println!("Invalid choice. Please enter a valid menu option.");
            }
        }
    }
}
