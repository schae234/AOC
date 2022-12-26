use std::cmp::Reverse;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

pub fn mod_main(args: Vec<String>) -> Result<(), Error> {
    if args.len() != 1 {
        panic!("Please provide a single input: input.txt, not {:?}", args)
    } else {
        let calories_file = &args[0];
        println!("Calculating calories from input file: {:?}", calories_file);

        let input = File::open(calories_file)?;
        let buffered = BufReader::new(input);

        let mut current_elf = 1;
        let mut current_calories = 0;

        let mut calories = Vec::new();

        for line_res in buffered.lines() {
            let line = line_res?;
            if line.trim().is_empty() {
                println!("-------------------------");
                println!(
                    "Current max elf: {:?} with {:?} calories",
                    current_elf, current_calories
                );
                println!("Current elf: {:?} has {:?}", current_elf, current_calories);
                // Store and reset
                calories.push((current_elf, current_calories));

                current_calories = 0;
                current_elf += 1;
            } else {
                let number = line.trim().parse::<i64>().unwrap();
                current_calories += number;
            }
        }
        calories.push((current_elf, current_calories));

        // Sort the calories vector
        calories.sort_by_key(|x| Reverse(x.1));
        println!(
            "The elf with the most food is: {:?} who has {:?} calories",
            calories[0].0, calories[0].1
        );

        let mut top3: i64 = 0;
        for (_, cals) in calories[0..3].iter() {
            top3 += cals;
        }

        println!("The sum of the top three calories is: {:?}", top3)
    }
    Ok(())
}
