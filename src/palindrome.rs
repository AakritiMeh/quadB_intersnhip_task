use std::io::{self, Write};

fn is_palindrome(s: &str) -> bool {
    let s = s.to_lowercase();
    let chars = s.chars();
    let reversed_chars: Vec<char> = chars.rev().collect();
    let reversed_string: String = reversed_chars.iter().collect();
    s == reversed_string
}

pub fn main() {
    let mut input = String::new();
    print!("Enter a string: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim(); // Remove trailing newline

    if is_palindrome(input) {
        println!("'{}' is a palindrome!", input);
    } else {
        println!("'{}' is not a palindrome.", input);
    }
}
