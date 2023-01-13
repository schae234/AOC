use std::fs::File;
use std::io::{BufRead, BufReader, Error};


pub fn mod_main(args: Vec<String>) -> Result<(), Error> {
    if args.len() != 1 {
        panic!("Please provide a single input file: input.txt");
    }

    let input = File::open(&args[0])?;

    let rows: Vec<Vec<u32>> = BufReader::new(input)
        .lines()
        .map(|x| x.unwrap().chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect();

    println!("{:#?}", rows);
    Ok(())
}
