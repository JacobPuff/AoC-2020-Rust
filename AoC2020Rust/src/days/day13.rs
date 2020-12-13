use crate::days;

pub fn day13() {
    let bus_data: Vec<String> = days::tools::open_file_from_path("src/days/day13data.txt");
    let earliest_time: i64 = bus_data[0].parse::<i64>().unwrap();
    let bus_list: Vec<&str> = bus_data[1].split(",").collect();
    let mut earliest_bus_time: i64 = i64::MAX;
    let mut earliest_bus: i64 = 0;

    // Part one
    for bus_str in &bus_list {
        if bus_str == &"x" {
            continue;
        }
        let bus: i64 = bus_str.parse::<i64>().unwrap();
        let next_time: i64 = ((earliest_time as f64/bus as f64).ceil() as i64 * bus) - earliest_time;
        if next_time < earliest_bus_time {
            earliest_bus_time = next_time;
            earliest_bus = bus;
        }
        
    }

    // Part two
    // adapted from https://github.com/colinodell/advent-2020/blob/main/day13/day13.go
    // It makes much sense.
    let mut int_bus_list: Vec<i64> = Vec::new();
    for i in 0..bus_list.len() {
        let bus_str = bus_list[i];
        if bus_str == "x" {
            int_bus_list.push(1);
            continue;
        }
        let bus: i64 = bus_str.parse::<i64>().unwrap();
        int_bus_list.push(bus);
    }
    let mut earliest_ordered_bus_time: i64 = 0;
    loop {
        let mut time_to_skip_if_no_match: i64 = 1;
        let mut found_time = true;
        for i in 0..int_bus_list.len() {
            let bus = int_bus_list[i];
            if bus == 1 {
                continue;
            }
            if (earliest_ordered_bus_time + i as i64) % bus != 0 {
                found_time = false;
                break;
            }
            
            // This particular bus schedule matches, but we don't know if subsequent ones will. However, we
			// do know this when this particular schedule will match again, so let's keep track of that.
			// For example, if the first bus is Bus 7, we know it won't depart again for another 7 minutes,
			// so we'll skip ahead by 7 minutes and ignore the timestamps we know won't match.
			//
			// If we find a partial match where, say, 2 or 3 schedules match but not the whole thing, we
			// can calculate the next time those 2-3 schedules align by multiplying their values together;
			// worst case, we still have no match there, or best case we find yet another matching bus!
			//
			// For example, let's say we find a timestamp where the first two buses (7 and 11) align but none
			// of the others do; in that case, we know that buses 7 and 9 won't align again for another
			// 77 (7*11) minutes, so we'll skip ahead 77 minutes. Eventually we might find that now buses
			// 7, 11, and 13 align, but none others do. Well, that means the next time that these three buses
			// align will be in 7*11*13 minutes, so skip ahead by that much and try again there.
			//
			// This approach significantly speeds up the search and the speed improves the bigger your
			// partial match is!
			//
			// (Note that technically we'd need to find the LCM of those bus IDs, but luckily our inputs are
			// always prime numbers so the LCM will always equal the product of those IDs.)
            time_to_skip_if_no_match *= bus
        }
        if found_time {
            break;
        }
        earliest_ordered_bus_time += time_to_skip_if_no_match;
    }

    // Part one earliest bus 601 multiplied by minutes to wait 7 is 4207
    println!("Part one earliest bus {} multiplied by minutes to wait {} is {}", earliest_bus, earliest_bus_time, earliest_bus*earliest_bus_time);
    // Part two earliest time buses leave in order is 725850285300475
    println!("Part two earliest time buses leave in order is {}", earliest_ordered_bus_time);
}