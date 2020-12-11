use std::io::{self, BufRead, BufReader};
use std::fs::{canonicalize, File};

pub fn read_from_stdin(prompt: &str) -> Option<String> {
    println!("{}", prompt);
    let stdin = io::stdin();
    let input = stdin.lock().lines().next()?.expect("Found no input");
    Some(input.to_string())
}

pub fn open_file_from_path(path_string: &str) -> Vec<String> {
    let path = canonicalize(path_string).unwrap();
    let file = File::open(path).unwrap();
    let file_vec = BufReader::new(file).lines().map(|x| x.unwrap()).collect();
    return file_vec
}