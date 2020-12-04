mod days;

fn main() {
    let day: i32 = 4;
    println!("Running day {}", day);
    match day {
        1 => days::day1::day1(),
        2 => days::day2::day2(),
        3 => days::day3::day3(),
        4 => days::day4::day4(),
        _ => println!("We dont have that day.")
    }
}