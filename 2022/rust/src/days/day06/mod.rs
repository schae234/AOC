use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

pub fn mod_main(args: Vec<String>) -> Result<(), Error> {
    if args.len() != 1 {
        panic!("Please provide a single input file: input.txt");
    }
    let input = File::open(&args[0])?;
    let buffered = BufReader::new(input);

    for line_res in buffered.lines() {
        let mut position = 0;
        let mut window: VecDeque<char> = VecDeque::new();
        let mut packet_start_found = false;

        let mut message: VecDeque<char> = VecDeque::new();
        let mut message_found = false;
        fr code in line_res?.chars() {
            position += 1;
            window.push_back(code);
            // part1
            if !packet_start_found && window.len() > 4 {
                window.pop_front();
                let mut codes = HashSet::new();
                for c in window.iter() {
                    codes.insert(*c);
                }
                if codes.len() == 4 {
                    println!("The starting position for code is {:?}", position);
                    packet_start_found = true;
                }
            }
            // part2
            message.push_back(code);
            if !message_found && message.len() > 14 {
                message.pop_front();
                let mut codes = HashSet::new();
                for c in message.iter() {
                    codes.insert(*c);
                }
                if codes.len() == 14 {
                    println!("The message for code is {:?}", position);
                    message_found = true;
                } else {
                }
            }
        }
    }

    Ok(())
}
