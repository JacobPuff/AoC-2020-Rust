mod days;

fn main() {
    let day: i32 = 2;
    println!("Running day {}", day);
    match day {
        1 => days::day1::day1(),
        2 => days::day1::day1(),
        _ => println!("We dont have that day.")
    }
}