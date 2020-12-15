use std::collections::HashMap;

pub fn day15() {
    // Part one
    let part_one_wanted_number_count: i64 = 2020;
    let part_one_wanted_number_value: i64 = get_num_at_count(part_one_wanted_number_count);
    println!("Part one wanted num {} is {}", part_one_wanted_number_count, part_one_wanted_number_value);
    
    // Part two
    let part_two_wanted_number_count: i64 = 30000000;
    let part_two_wanted_number_value: i64 = get_num_at_count(part_two_wanted_number_count);
    println!("Part two wanted num {} is {}", part_two_wanted_number_count, part_two_wanted_number_value);
}

fn get_num_at_count(wanted_number: i64) -> i64 {
    let input: [i64;6] = [8,0,17,4,1,12];
    let mut last_counted_map: HashMap<i64, i64> = HashMap::new();
    for i in 0..input.len() {
        last_counted_map.insert(input[i], i as i64+1);
    }
    
    let mut last_num = input[input.len()-1];
    let mut next_num = 0;
    for count in input.len()+1..wanted_number as usize+1 {
        next_num = 0;
        if last_counted_map.contains_key(&last_num) {
            next_num = count as i64-1 - last_counted_map[&last_num];
            last_counted_map.insert(last_num, count as i64-1);
        } else {
            last_counted_map.insert(last_num, count as i64-1);
        }
        last_num = next_num;
    }
    return next_num
}