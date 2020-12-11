use std::collections::HashMap;

const WALL: i64 = 0;
const DEVICE_DIFF: i64 = 3;

pub fn day10() {
    let mut data = get_data();
    data.push(WALL);
    data.sort();
    let mut diff_list: Vec<i64> = Vec::new();

    let mut part_one_diff_of_one = 0;
    let mut part_one_diff_of_three = 1;
    // Part one
    for x in 0..data.len()-1 {
        let diff = data[x+1] - data[x];
        diff_list.push(diff);
        match diff {
            1 => part_one_diff_of_one += 1,
            3 => part_one_diff_of_three += 1,
            _ => ()
        }
    }
    // Part two
    diff_list.push(DEVICE_DIFF);
    let mut part_two_combinations: i64 = 0;
    let mut arrangable_ones: u32 = 0;
    for x in 0..diff_list.len() {
        match diff_list[x] {
            1 => arrangable_ones += 1,
            3 => {
                if arrangable_ones > 1 {
                    arrangable_ones -= 1;
                    if part_two_combinations != 0 {
                        part_two_combinations *= get_arrangements(arrangable_ones);
                    } else {
                        part_two_combinations = get_arrangements(arrangable_ones);
                    }
                }
                arrangable_ones = 0;
            },
            _ => ()
        }
    }
    

    // Part one diffs of one(64) and three(30) multiplied is 1920
    println!("Part one diffs of one({}) and three({}) multiplied is {}", part_one_diff_of_one, part_one_diff_of_three, part_one_diff_of_one*part_one_diff_of_three);
    // Part two number of arrangements is 1511207993344
    println!("Part two number of arrangements is {}", part_two_combinations);
}

fn get_arrangements(arrangable_ones: u32) -> i64 {
    let combinations: u64 = 2u64.pow(arrangable_ones);
    let mut count_of_valid = 0;
    for x in 0..combinations {
        let mut num_of_zeros = 0;
        let mut valid = true;
        for b in 0..arrangable_ones {
            if x & (1 << b) == 0 {
                num_of_zeros += 1;
            } else {
                num_of_zeros = 0;
            }
            
            if num_of_zeros == 3 {
                valid = false;
                break;
            }
        }

        if valid {
            count_of_valid += 1
        }
    }
    return count_of_valid
}

fn get_data() -> Vec<i64>{
    return vec![97,
    62,
    23,
    32,
    51,
    19,
    98,
    26,
    90,
    134,
    73,
    151,
    116,
    76,
    6,
    94,
    113,
    127,
    119,
    44,
    115,
    50,
    143,
    150,
    86,
    91,
    36,
    104,
    131,
    101,
    38,
    66,
    46,
    96,
    54,
    70,
    8,
    30,
    1,
    108,
    69,
    139,
    24,
    29,
    77,
    124,
    107,
    14,
    137,
    16,
    140,
    80,
    68,
    25,
    31,
    59,
    45,
    126,
    148,
    67,
    13,
    125,
    53,
    57,
    41,
    47,
    35,
    145,
    120,
    12,
    37,
    5,
    110,
    138,
    130,
    2,
    63,
    83,
    22,
    79,
    52,
    7,
    95,
    58,
    149,
    123,
    89,
    109,
    15,
    144,
    114,
    9,
    78]
}