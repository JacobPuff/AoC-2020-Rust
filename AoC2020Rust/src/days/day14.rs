use std::collections::HashMap;
use crate::days;

pub fn day14() {
    // Part one
    part_one();
    // Part two
    part_two();
    
}

pub fn part_one() {
    let data: Vec<String> = days::tools::open_file_from_path("src/days/day14data.txt");
    let mut current_mask: &str = "";
    let mut memory_map: HashMap<u64, u64> = HashMap::new();
    let mut part_one_sum_of_memory: u64 = 0;
    for line in &data {
        let data_chunks: Vec<&str> = line.split(" ").collect();
        if data_chunks[0] == "mask" {
            current_mask = data_chunks[2];
            continue;
        }
        let split_for_mempory_pos: Vec<&str> = data_chunks[0].split(&['[',']'][..]).collect();
        let memory_pos = split_for_mempory_pos[1].parse::<u64>().unwrap();
        let mut memory_num = data_chunks[2].parse::<u64>().unwrap();
        for i in 0..current_mask.len() {
            let mask_bit = current_mask.chars().nth(current_mask.len()-1-i).unwrap();
            match mask_bit {
                'X' => continue,
                '0' => memory_num &= !(1 << i),
                '1' => memory_num |= 1 << i,
                _ => println!("Part one this should never happen.")
            }
        }
        memory_map.insert(memory_pos, memory_num);
    }

    for value in memory_map.values() {
        part_one_sum_of_memory += value;
    }
    // Part one sum of memory nums with mask 6513443633260
    println!("Part one sum of memory nums with mask {}", part_one_sum_of_memory)
}

pub fn part_two() {
    let data: Vec<String> = days::tools::open_file_from_path("src/days/day14data.txt");
    let mut current_mask: &str = "";
    let mut mask_combinations: u64 = 0;
    let mut memory_map: HashMap<u64, u64> = HashMap::new();
    let mut part_two_sum_of_memory: u64 = 0;
    for line in &data {
        let data_chunks: Vec<&str> = line.split(" ").collect();
        if data_chunks[0] == "mask" {
            current_mask = data_chunks[2];
            let mut x_count: u64 = 0;
            for x in current_mask.chars() {
                if x == 'X' {
                    x_count += 1;
                }
            }
            mask_combinations = 2u64.pow(x_count as u32);
            continue;
        }
        let split_for_mempory_pos: Vec<&str> = data_chunks[0].split(&['[',']'][..]).collect();
        let mut memory_pos = split_for_mempory_pos[1].parse::<u64>().unwrap();
        let memory_num = data_chunks[2].parse::<u64>().unwrap();
        
        for x in 0..mask_combinations {
            let mut x_count: u64 = 0;
            for i in 0..current_mask.len() {
                let mut mask_bit = current_mask.chars().nth(current_mask.len()-1-i).unwrap();

                if mask_bit == 'X' {
                    if &x & (1 << x_count) == 0 {
                        mask_bit = '0';
                        memory_pos &= !(1 << i);
                    } else {
                        mask_bit = '1';
                    }
                    x_count += 1;
                }
                
                match mask_bit {
                    '0' => (),
                    '1' => memory_pos |= 1 << i,
                    _ => println!("Part two this should never happen.")
                }
            }
            memory_map.insert(memory_pos, memory_num);
        }
    }

    for value in memory_map.values() {
        part_two_sum_of_memory += value;
    }
    // Part two sum of memory is 3442819875191
    println!("Part two sum of memory is {}", part_two_sum_of_memory)
}