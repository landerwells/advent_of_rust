mod utils;
mod year2015;
mod year2016;
mod year2017;
mod year2018;
mod year2023;
use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <year> <day>", args[0]);
        std::process::exit(1);
    }

    let year = &args[1];
    let day = format!("day{:02}", args[2].parse::<u32>().expect("Invalid day"));

    match (year.as_str(), day.as_str()) {
        ("2015", "day01") => year2015::day01::run(),
        ("2015", "day07") => year2015::day07::run(),
        ("2015", "day18") => year2015::day18::run(),
        ("2016", "day02") => year2016::day02::run(),
        ("2017", "day02") => year2017::day02::run(),
        ("2017", "day06") => year2017::day06::run(),
        ("2017", "day07") => year2017::day07::run(),
        ("2018", "day01") => year2018::day01::run(),
        ("2023", "day01") => year2023::day01::run(),
        ("2023", "day02") => year2023::day02::run(),
        ("2023", "day03") => year2023::day03::run(),
        ("2023", "day04") => year2023::day04::run(),
        ("2018", "day02") => year2018::day02::run(),
        ("2018", "day03") => year2018::day03::run(),
<<<<<<< HEAD
=======
        ("2018", "day04") => year2018::day04::run(),
        ("2016", "day01") => year2016::day01::run(),
>>>>>>> 251eca4ab5ce56fd1276cd264ea331e411f48466
        // Add new days here
        _ => {
            // Call the create_day.sh script
            let status = Command::new("sh")
                .arg("create_day.sh")
                .arg(year)
                .arg(&args[2]) // Pass the day without leading zero
                .status()
                .expect("Failed to execute script");

            if !status.success() {
                eprintln!("Failed to create solution for year {} day {}", year, day);
                std::process::exit(1);
            }
            println!(
                "Successfully created solution for year {} day {}",
                year, day
            );
        }
    }
}
