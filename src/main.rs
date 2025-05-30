use std::io;
mod utils;
use utils::*;

fn main() {
    // cargo run 2024 15
    // let args: Vec<String> = env::args().collect();
    // dbg!(args);

    loop {
        println!("1) Update Cookie");
        println!("2) Generate Day");
        println!("3) Get Input");
        println!("4) Quit");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("ERROR: unable to read user input");

        // Need to write some better code to handle errors in the case where
        // the data entered is not a number
        let input: i32 = input.trim().parse().unwrap();

        // some things return result or what is the other one?
        match input {
            1 => update_cookie().unwrap(),
            2 => {
                println!("Please enter the year:");
                let mut year = String::new();
                io::stdin()
                    .read_line(&mut year)
                    .expect("ERROR: unable to read user input");

                println!("Please enter the day:");
                let mut day = String::new();
                io::stdin()
                    .read_line(&mut day)
                    .expect("ERROR: unable to read user input");

                generate_day(year.trim().parse().unwrap(), day.trim().parse().unwrap());
            }
            3 => {
                println!("Please enter the year:");
                let mut year = String::new();
                io::stdin()
                    .read_line(&mut year)
                    .expect("ERROR: unable to read user input");

                println!("Please enter the day:");
                let mut day = String::new();
                io::stdin()
                    .read_line(&mut day)
                    .expect("ERROR: unable to read user input");

                println!(
                    "{}",
                    get_input(year.trim().parse().unwrap(), day.trim().parse().unwrap())
                );
            }
            4 => break,
            _ => println!("Please enter a valid number"),
        }
    }
}

// I do like the approach of having all my files in the bink

// for now I think that I will go with the bin strategy, adding things to
