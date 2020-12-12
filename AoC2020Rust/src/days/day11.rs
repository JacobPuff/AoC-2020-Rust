use crate::days;

pub fn day11() {

    let part_one_occupied_count = count_seats_in_result(get_stable_with_part(1));
    // Part one seats occupied (using immediate neighbors) 2183
    println!("Part one seats occupied (using immediate neighbors) {}", part_one_occupied_count);

    let part_two_occupied_count = count_seats_in_result(get_stable_with_part(2));
    // Part two seats occupied (using vector) {}
    println!("Part two seats occupied (using vector) {}", part_two_occupied_count);
}

fn get_stable_with_part(part: i64) -> Vec<String> {
    let mut last_iteration: Vec<String> = days::tools::open_file_from_path("src/days/day11data.txt");
    let mut next_iteration: Vec<String> = Vec::new();
    let mut stable: bool = false;
    while !stable {
        for y in 0..last_iteration.len() {
            let mut next_line: Vec<String> = Vec::new();
            for x in 0..last_iteration[y].len() {
                let current_char = last_iteration[y].chars().nth(x).unwrap();
                let mut next_char = current_char;
                if current_char != '.' {
                    
                    if part == 1 {
                        // Part one
                        let neighbors: Vec<char> = get_neighbors(&last_iteration, x, y);
                        if neighbors.len() == 0 {
                            next_char = '#';
                        }
                        if neighbors.len() >= 4 {
                            next_char = 'L';
                        }
                    } else {
                        // Part two
                        let neighbors: i64 = get_neighbors_with_vector(&last_iteration, x, y);
                        if neighbors == 0 {
                            next_char = '#';
                        }
                        if neighbors >= 5 {
                            next_char = 'L';
                        }
                    }
                }
                next_line.push(next_char.to_string());
            }
            let next_string = next_line.join("");
            next_iteration.push(next_string);
        }
        stable = next_iteration == last_iteration;
        last_iteration = next_iteration.to_vec();
        next_iteration = Vec::new();
    }
    return last_iteration
}

fn count_seats_in_result(iteration: Vec<String>) -> i64 {
    let mut count = 0;
    for y in 0..iteration.len() {
        for x in 0..iteration[y].len() {
            let current_char = iteration[y].chars().nth(x).unwrap();
            if current_char == '#' {
                count += 1;
            }
        }
    }
    return count
}

fn get_neighbors(iteration: &Vec<String>, x: usize, y: usize) -> Vec<char> {
    let mut neighbors: Vec<char> = Vec::new();
    let up_good: bool = y > 0;
    let down_good: bool = y < iteration.len()-1;
    let left_good: bool = x > 0;
    let right_good: bool = x < iteration[y].len()-1;

    if up_good {
        let up: char = iteration[y-1].chars().nth(x).unwrap();
        neighbors.push(up);
    }
    
    if down_good {
        let down: char = iteration[y+1].chars().nth(x).unwrap();
        neighbors.push(down);
    }

    if left_good {
        let left: char = iteration[y].chars().nth(x-1).unwrap();
        neighbors.push(left);
    }

    if right_good {
        let right: char = iteration[y].chars().nth(x+1).unwrap();
        neighbors.push(right);
    }

    if up_good && left_good {
        let up_left: char = iteration[y-1].chars().nth(x-1).unwrap();
        neighbors.push(up_left);
    }
    
    if up_good && right_good {
        let up_right: char = iteration[y-1].chars().nth(x+1).unwrap();
        neighbors.push(up_right);
    }

    if down_good && left_good {
        let down_left: char = iteration[y+1].chars().nth(x-1).unwrap();
        neighbors.push(down_left);
    }
    
    if down_good && right_good {
        let down_right: char = iteration[y+1].chars().nth(x+1).unwrap();
        neighbors.push(down_right);
    }

    let mut neighbors_occupied: Vec<char> = Vec::new();
    for neighbor in 0..neighbors.len() {
        if neighbors[neighbor] == '#' {
            neighbors_occupied.push(neighbors[neighbor]);
        }
    }
    return neighbors_occupied
}

fn get_neighbors_with_vector(iteration: &Vec<String>, initial_x: usize, initial_y: usize) -> i64 {
    let dirs: [(i64, i64); 8] = [
        (0, -1),
        (0, 1),
        (-1, 0),
        (1, 0),
        (-1, -1),
        (1, -1),
        (-1, 1),
        (1, 1),
    ];
    let mut neighbor_count = 0;
    for dir in dirs.iter() {
        let mut x = initial_x as i64;
        let mut y = initial_y as i64;
        let (x_diff, y_diff) = dir;
        loop {
            x += x_diff;
            y += y_diff;
            if x < 0 || x > iteration[0].len() as i64 -1 ||
            y < 0 || y > iteration.len() as i64 -1 {
                break;
            }
            let current_char = iteration[y as usize].chars().nth(x as usize).unwrap();
            if current_char == '#' {
                neighbor_count += 1;
                break;
            }
            if current_char == 'L' {
                break;
            }
        }
    }
    return neighbor_count
}