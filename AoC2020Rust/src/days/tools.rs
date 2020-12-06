use std::io::{self, BufRead};

pub fn read_from_stdin(prompt: &str) -> Option<String> {
    println!("{}", prompt);
    let stdin = io::stdin();
    let line_one = &stdin.lock().lines().next().unwrap().unwrap();
    Some(line_one.to_string())
}