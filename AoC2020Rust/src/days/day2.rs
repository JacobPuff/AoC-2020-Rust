use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader};
use std::path::Path;

pub fn day2() {
    let path = Path::new("src/days/day2data.txt").as_os_str();
    let data_file = File::open(path).unwrap();
    let data_file = BufReader::new(data_file);
    let mut part_one_valid_passwords = 0;
    let mut part_two_valid_passwords = 0;

    for wrapped_line in data_file.lines() {
        let line = wrapped_line.unwrap();
        let split_on_space: Vec<&str> = line.split(" ").collect();
        
        let mut count_of_letter: i32 = 0;
        let look_for_letter: char = split_on_space[1].chars().nth(0).unwrap();
        let password = split_on_space[2];

        let min_and_max: Vec<&str> = split_on_space[0].split("-").collect();
        let min: i32 =  min_and_max[0].parse::<i32>().unwrap();
        let max: i32 =  min_and_max[1].parse::<i32>().unwrap();

        // Part one
        for c in password.chars() {
            if c == look_for_letter {
                count_of_letter += 1;
            }
        }
        if count_of_letter >= min && count_of_letter <= max {
            part_one_valid_passwords += 1;
        }

        // Part two
        let pos_one: usize = min as usize - 1;
        let pos_two: usize = max as usize - 1;
        let has_at_one: bool = password.chars().nth(pos_one).unwrap() == look_for_letter;
        let has_at_two: bool = password.chars().nth(pos_two).unwrap() == look_for_letter;
        if  has_at_one && !has_at_two || !has_at_one && has_at_two {
            part_two_valid_passwords += 1;
        }
    }

    // There are 416 valid passwords in part one.
    // There are 688 valid passwords in part two.
    println!("There are {} valid passwords in part one.", part_one_valid_passwords);
    println!("There are {} valid passwords in part two.", part_two_valid_passwords);
}