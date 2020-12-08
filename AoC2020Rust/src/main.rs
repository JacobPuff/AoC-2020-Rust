mod days;

fn main() {
    let mut day: String = String::new();
    let input = days::tools::read_from_stdin("What day would you like to run?");
    match input {
        Some(txt) => day = txt,
        _ => ()
    }
    println!("Running day {}", day);
    match day.as_str() {
        "1" => days::day1::day1(),
        "2" => days::day2::day2(),
        "3" => days::day3::day3(),
        "4" => days::day4::day4(),
        "5" => days::day5::day5(),
        "6" => days::day6::day6(),
        "7" => days::day7::day7(),
        _ => println!("We dont have that day.")
    }
}