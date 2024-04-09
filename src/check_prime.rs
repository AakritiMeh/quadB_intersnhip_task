use std::io::{self, Write};

pub fn main() {
    let mut input = String::new();
    print!("Enter a number: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let num: u64 = input.trim().parse().unwrap();
    let result = is_prime(num);
    println!("Is Prime? {}", result);
}

fn is_prime(num: u64) -> bool {
    if num <= 1 {
        return false;
    }
    for i in 2..=num / 2 {
        if num % i == 0 {
            return false;
        }
    }
    true
}
