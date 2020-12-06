use std::io::{self, BufRead};

pub fn read_from_stdin(prompt: &str) -> Option<String> {
    println!("{}", prompt);
    let stdin = io::stdin();
    let input = stdin.lock().lines().next()?.expect("Found no input");
    Some(input.to_string())
}