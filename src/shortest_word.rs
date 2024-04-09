use std::io::{self, Write};

pub fn main() {
    let mut input = String::new();
    print!("Enter a string of words: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let shortest = shortest_word(&input);
    println!("Shortest word: {}", shortest);
}

fn shortest_word(input: &str) -> &str {
    input
        .split_whitespace()
        .min_by_key(|word| word.len())
        .unwrap_or("")
}
