use std::env;
use std::io::Error;

mod days;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    // Create a new args vector for the module main function
    let day_args: Vec<String> = args[2..].iter().cloned().collect();

    match args[1].as_str() as &str {
        "day01" => days::day01::mod_main(day_args)?,
        "day02" => days::day02::mod_main(day_args)?,
        "day03" => days::day03::mod_main(day_args)?,
        "day04" => days::day04::mod_main(day_args)?,
        "day05" => days::day05::mod_main(day_args)?,
        "day06" => days::day06::mod_main(day_args)?,
        "day07" => days::day07::mod_main(day_args)?,
        "day08" => days::day08::mod_main(day_args)?,
        "day09" => days::day09::mod_main(day_args)?,
        "day10" => days::day10::mod_main(day_args)?,
        _ => {
            panic!("{:?} not a valid AOC day", args[1])
        }
    }

    Ok(())
}
