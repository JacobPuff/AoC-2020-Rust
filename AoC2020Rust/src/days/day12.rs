use crate::days;

const DIRS: [(i64, i64); 4] = [
    (1,0),
    (0,1),
    (-1,0),
    (0,-1),
];

pub fn day12() {
    // Part one
    day12a();
    // Part two
    day12b();
}

pub fn day12a() {
    let mut current_dir = 0;
    let mut x_pos = 0;
    let mut y_pos = 0;
    
    let ship_instructions: Vec<String> = days::tools::open_file_from_path("src/days/day12data.txt");
    for instruction in ship_instructions {
        let instrcution_type = instruction.chars().nth(0).unwrap();
        let amount = instruction[1..].parse::<i64>().unwrap();
        match instrcution_type {
            'R' => {
                current_dir = (current_dir + amount/90) % (DIRS.len() as i64);
            },
            'L' => {
                current_dir = ((current_dir - amount/90) + DIRS.len() as i64) % (DIRS.len() as i64);
            },
            'F' => {
                x_pos += DIRS[current_dir as usize].0 * amount;
                y_pos += DIRS[current_dir as usize].1 * amount;
            },
            'N' => y_pos -= amount,
            'S' => y_pos += amount,
            'W' => x_pos -= amount,
            'E' => x_pos += amount,
            _ => println!("This shouldn't happen")
        }
    }
    // Part one ship manhatten distance 1152
    println!("Part one ship manhatten distance {}", x_pos.abs() + y_pos.abs())
}


pub fn day12b() {
    let mut x_pos = 0;
    let mut y_pos = 0;
    let mut waypoint = (10, -1);
    
    let ship_instructions: Vec<String> = days::tools::open_file_from_path("src/days/day12data.txt");
    for instruction in ship_instructions {
        let instrcution_type = instruction.chars().nth(0).unwrap();
        let amount = instruction[1..].parse::<i64>().unwrap();
        match instrcution_type {
            'R' => {
                for _ in 0..amount/90 {
                    waypoint = (waypoint.1, waypoint.0*-1);
                }
            },
            'L' => {
                for _ in 0..amount/90 {
                    waypoint = (waypoint.1*-1, waypoint.0);
                }
            },
            'F' => {
                x_pos += waypoint.0 * amount;
                y_pos += waypoint.1 * amount;
            },
            'N' => waypoint.1 -= amount,
            'S' => waypoint.1 += amount,
            'W' => waypoint.0 -= amount,
            'E' => waypoint.0 += amount,
            _ => println!("This shouldn't happen")
        }
    }
    // Part two ship manhatten distance 58637
    println!("Part two ship manhatten distance {}", x_pos.abs() + y_pos.abs())
}