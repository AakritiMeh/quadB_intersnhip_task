use std::io::{self, Write};

pub fn main() {
    let mut input = String::new();
    print!("Enter space-separated strings: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let strings: Vec<&str> = input.trim().split_whitespace().collect();

    let prefix = longest_common_prefix(&strings);
    println!("Longest Common Prefix: {}", prefix);
}

fn longest_common_prefix(strings: &Vec<&str>) -> String {
    if strings.is_empty() {
        return String::new();
    }
    let mut prefix = strings[0].to_string();
    for &s in &strings[1..] {
        while !s.starts_with(&prefix) {
            prefix.pop();
        }
    }
    prefix
}
