use std::collections::HashSet;
use crate::days;

#[derive(Debug, Default)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y && self.z == other.z
    }
}

#[derive(Debug, Default, Hash, Eq)]
struct Point4d {
    x: i64,
    y: i64,
    z: i64,
    w: i64,
}

impl PartialEq for Point4d {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w
    }
}

pub fn day17() {
    part_one(6);
    part_two(6);
}

fn part_one(get_cycle: i64) {
    let data: Vec<String> = days::tools::open_file_from_path("src/days/day17data.txt");
    let mut point_list: Vec<Point> = Vec::new();
    let mut y = 0;
    for line in &data {
        let mut x = 0;
        for c in line.chars() {
            if c == '#' {
                point_list.push(Point{x, y, z:0});
            }
            x += 1;
        }
        y += 1;
    }

    for cycle in 0..get_cycle {
        let mut new_point_list: Vec<Point> = Vec::new();
        for point in &point_list {
            let (on_count, off_neighbors) = get_neighbors_xyz(&point_list, point);
            if on_count == 2 || on_count == 3 {
                new_point_list.push(Point{x:point.x, y:point.y, z:point.z});
            }
            for npoint in off_neighbors {
                let (on_count, _) = get_neighbors_xyz(&point_list, &npoint);
                if on_count == 3 && !new_point_list.contains(&npoint) {
                    new_point_list.push(npoint);
                }
            }
        }
        point_list = new_point_list;
        println!("cycle {}", cycle);
    }
    // Part one number of cubes active after 6 cycles is 232
    println!("Part one number of cubes active after {} cycles is {}", get_cycle, point_list.len());
}

fn part_two(get_cycle: i64) {
    let data: Vec<String> = days::tools::open_file_from_path("src/days/day17data.txt");
    let mut point_set: HashSet<Point4d> = HashSet::new();
    let mut y = 0;
    for line in &data {
        let mut x = 0;
        for c in line.chars() {
            if c == '#' {
                point_set.insert(Point4d{x, y, z:0, w:0});
            }
            x += 1;
        }
        y += 1;
    }

    for cycle in 0..get_cycle {
        let mut new_point_set: HashSet<Point4d> = HashSet::new();
        for point in &point_set {
            let (on_count, off_neighbors) = get_neighbors_xyzw(&point_set, point);
            if on_count == 2 || on_count == 3 {
                new_point_set.insert(Point4d{x:point.x, y:point.y, z:point.z, w:point.w});
            }
            for npoint in off_neighbors {
                let (on_count, _) = get_neighbors_xyzw(&point_set, &npoint);
                if on_count == 3 {
                    new_point_set.insert(npoint);
                }
            }
        }
        point_set = new_point_set;
        println!("4d cycle {}", cycle);
    }
    // Part two number of cubes active after 6 cycles is 232
    println!("Part two number of 4D cubes active after {} cycles is {}", get_cycle, point_set.len());
}

fn get_neighbors_xyz(point_list: &Vec<Point>, point: &Point) -> (i64, Vec<Point>) {
    // Is this gross? Yes. Is this better than writing a list of 26 directions? Also yes.
    let mut on_neighbor_count: i64 = 0;
    let mut check_off_neighbors: Vec<Point> = Vec::new();
    for x in 0..=2 {
        for y in 0..=2 {
            for z in 0..=2 {
                if x-1 == 0 && y-1 == 0 && z-1 == 0 {
                    continue
                }
                if point_list.contains(&Point{x:point.x + x-1, y:point.y + y-1, z:point.z + z-1}) {
                    on_neighbor_count += 1;
                } else {
                    check_off_neighbors.push(Point{x:point.x + x-1, y:point.y + y-1, z:point.z + z-1});
                }
            }
        }
    }
    return (on_neighbor_count, check_off_neighbors);
}

fn get_neighbors_xyzw(point_set: &HashSet<Point4d>, point: &Point4d) -> (i64, Vec<Point4d>) {
    // Is this gross? Yes. Is this better than writing a list of 26 directions? Also yes.
    let mut on_neighbor_count: i64 = 0;
    let mut check_off_neighbors: Vec<Point4d> = Vec::new();
    for x in 0..=2 {
        for y in 0..=2 {
            for z in 0..=2 {
                for w in 0..=2 {
                    if x-1 == 0 && y-1 == 0 && z-1 == 0 && w-1 == 0 {
                        continue
                    }
                    if point_set.contains(&Point4d{x:point.x + x-1, y:point.y + y-1, z:point.z + z-1, w:point.w + w-1}) {
                        on_neighbor_count += 1;
                    } else {
                        check_off_neighbors.push(Point4d{x:point.x + x-1, y:point.y + y-1, z:point.z + z-1, w:point.w + w-1});
                    }
                }
            }
        }
    }
    return (on_neighbor_count, check_off_neighbors);
}