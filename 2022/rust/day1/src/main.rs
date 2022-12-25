use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead, Error};


fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        panic!("Please provide a single input: input.txt, not {:?}", args)
    }
    else {
        let calories_file = &args[1];
        println!("Calculating calories from input file: {:?}", calories_file);

        let input = File::open(calories_file)?;
        let buffered = BufReader::new(input);

        let mut current_elf = 1;
        let mut current_calories = 0;

        let mut max_elf: i64 = 0;
        let mut max_calories: i64 = 0;

        for line in buffered.lines() {
            match line {
                Err(why) => panic!("{:?}", why),
                Ok(val) => {
                    if val.trim().is_empty() {
                        println!("-------------------------");
                        println!("Current max elf: {:?} with {:?} calories",current_elf, current_calories);
                        println!("Current elf: {:?} has {:?}",current_elf, current_calories);
                        // Process the current elf
                        if current_calories > max_calories {
                            println!("New MAX!");
                            max_elf = current_elf;
                            max_calories = current_calories;
                        } else {
                            println!("Not a new max");
                        }
                        current_calories = 0;
                        current_elf += 1;
                    } else {
                        match val.trim().parse::<i64>() {
                            Err(why) => panic!("Calories not a number: {:?}", why),
                            Ok(number) => {
                                current_calories += number;
                            }
                        }
                    }
                }
            }
        }
        if current_calories > max_calories {
            max_elf = current_elf;
            max_calories = current_calories;
        }

        println!("The elf with the most calories is: {:?} who has {:?}", max_elf, max_calories);
    }
    Ok(())
}
