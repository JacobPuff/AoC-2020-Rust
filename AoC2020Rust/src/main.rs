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
        "8" => days::day8::day8(),
        "9" => days::day9::day9(),
        "10" => days::day10::day10(),
        "11" => days::day11::day11(),
        "12" => days::day12::day12(),
        "13" => days::day13::day13(),
        "14" => days::day14::day14(),
        "15" => days::day15::day15(),
        "16" => days::day16::day16(),
        "17" => days::day17::day17(),
        _ => println!("We dont have that day.")
    }
}