use crate::days;
use std::collections::HashMap;

#[derive(Debug, Default)]
struct Range {
    first: i64,
    second: i64
}

#[derive(Debug, Default)]
struct RangeSet {
    name: String,
    ranges: Vec<Range>
}

pub fn day16() {
    let (range_sets, your_ticket, mut nearby_tickets) = parse_input();
    // Part one
    nearby_tickets = part_one(&range_sets, nearby_tickets);
    // Part two
    part_two(range_sets, your_ticket, nearby_tickets);
}

fn part_one(range_sets: &Vec<RangeSet>, nearby_tickets: Vec<Vec<i64>>) -> Vec<Vec<i64>>{
    let mut vec_of_valid_nums: Vec<i64> = Vec::new();
    for rs in range_sets {
        for r in &rs.ranges {
            for i in r.first..=r.second {
                if !vec_of_valid_nums.contains(&i) {
                    vec_of_valid_nums.push(i)
                }
            }
        }
    }

    let mut part_one_invalid_ticket_fields: Vec<i64> = Vec::new();
    let mut valid_nearby_tickets: Vec<Vec<i64>> = Vec::new();
    for ticket in nearby_tickets {
        let mut valid = true;
        for field in &ticket {
            if !vec_of_valid_nums.contains(field) {
                part_one_invalid_ticket_fields.push(*field);
                valid = false;
            }
        }
        if valid {
            valid_nearby_tickets.push(ticket);
        }
    }

    let mut part_one_invalid_ticket_field_sum: i64 = 0;
    for field in part_one_invalid_ticket_fields {
        part_one_invalid_ticket_field_sum += field;
    }
    // Part one sum of invalid ticket fields is 20231
    println!("Part one sum of invalid ticket fields is {}", part_one_invalid_ticket_field_sum);
    return valid_nearby_tickets
}

fn part_two(range_sets: Vec<RangeSet>, your_ticket: Vec<i64>, nearby_tickets: Vec<Vec<i64>>) {
    let mut range_map: HashMap<String, HashMap<i64, i64>> = HashMap::new();
    for range_set in &range_sets {
        let new_hashmap = HashMap::new();
        range_map.insert(range_set.name.to_string(), new_hashmap);
    }

    // Get valid count per index for each range.
    for ticket in &nearby_tickets {
        for (f_idx, field) in ticket.iter().enumerate() {
            for rs in &range_sets {
                if !range_map[&rs.name].contains_key(&(f_idx as i64)) {
                    range_map.get_mut(&rs.name).unwrap().insert(f_idx as i64, 0);
                }
                for range in &rs.ranges {
                    if field >= &range.first && field <= &range.second {
                        let new_val = range_map[&rs.name][&(f_idx as i64)] + 1;
                        range_map.get_mut(&rs.name).unwrap().insert(f_idx as i64, new_val);
                    }
                }
            }
        }
    }

    let mut all_indexes: Vec<i64> = Vec::new();
    let mut field_to_index_map: HashMap<String, i64> = HashMap::new();
    for _i in 0..range_sets.len() {
        let mut best_range_set_total = i64::MAX;
        let mut best_range_set: &RangeSet = &RangeSet::default();
        // If there are less total valid numbers in a range, the range set is more accurate.
        for rs in &range_sets {
            // We've already found this range, do not get the total for it again.
            if field_to_index_map.contains_key(&rs.name) {
                continue
            }
            let mut total = 0;
            for count in range_map[&rs.name].values() {
                total += *count;
            }
            if total < best_range_set_total {
                best_range_set_total = total;
                best_range_set = rs;
            }
        }

        let mut best_range_set_count = 0;
        let mut best_index = -1;
        for (idx, count) in range_map[&best_range_set.name].iter() {
            if *count > best_range_set_count && !all_indexes.contains(idx) {
                best_range_set_count = *count;
                best_index = *idx
            }
        }
        field_to_index_map.insert(best_range_set.name.to_string(), best_index);
        all_indexes.push(best_index);
    }
    
    // In case you're curious.
    // println!("{:?}", field_to_index_map);

    let mut part_two_departer_fields_multiplied = 1;
    for (name, idx) in field_to_index_map.iter() {
        if name.contains("departure") {
            part_two_departer_fields_multiplied *= your_ticket[*idx as usize];
        }
    }
    // Part two departure fields on your ticket mupltiplied is 1940065747861
    println!("Part two departure fields on your ticket mupltiplied is {}", part_two_departer_fields_multiplied);
}

fn parse_input() -> (Vec<RangeSet>, Vec<i64>, Vec<Vec<i64>>) {
    let data: Vec<String> = days::tools::open_file_from_path("src/days/day16data.txt");
    let mut current_section = 0;
    let mut range_sets: Vec<RangeSet> = Vec::new();
    let mut your_ticket: Vec<i64> = Vec::new();
    let mut nearby_tickets: Vec<Vec<i64>> = Vec::new();

    for line in &data {
        if line == &"".to_string() {
            current_section += 1;
            continue;
        }
        match current_section {
            0 => {
                let split_for_name: Vec<&str> = line.split(": ").collect();
                let split_for_ranges: Vec<&str> = split_for_name[1].split(" or ").collect();
                let mut new_ranges: Vec<Range> = Vec::new();
                for range_str in split_for_ranges {
                    let range_num_strs: Vec<&str> = range_str.split("-").collect();
                    let mut first_num = range_num_strs[0].parse::<i64>().unwrap();
                    let mut second_num = range_num_strs[1].parse::<i64>().unwrap();
                    if second_num < first_num {
                        let temp = first_num;
                        first_num = second_num;
                        second_num = temp;
                    }
                    let new_range = Range{
                        first: first_num,
                        second: second_num
                    };
                    new_ranges.push(new_range);
                }
                let new_range_set = RangeSet{
                    name: split_for_name[0].to_string(),
                    ranges: new_ranges
                };
                range_sets.push(new_range_set);
            },
            1 => {
                if line == &"your ticket:".to_string() {
                    continue;
                }
                let num_strs: Vec<&str> = line.split(",").collect();
                for num_str in num_strs {
                    let num = num_str.parse::<i64>().unwrap();
                    your_ticket.push(num);
                }
            },
            2 => {
                if line == &"nearby tickets:".to_string() {
                    continue;
                }
                let num_strs: Vec<&str> = line.split(",").collect();
                let mut ticket: Vec<i64> = Vec::new();
                for num_str in num_strs {
                    let num = num_str.parse::<i64>().unwrap();
                    ticket.push(num);
                }
                nearby_tickets.push(ticket);
            },
            _ => ()
        }
    }
    return (range_sets, your_ticket, nearby_tickets)
}