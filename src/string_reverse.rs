use std::io::{self, Write};

pub fn main() {
    let mut input = String::new();
    print!("Enter a string: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let reversed = reverse_string(&input);
    println!("Reversed string: {}", reversed);
}

fn reverse_string(input: &str) -> String {
    input.chars().rev().collect()
}
