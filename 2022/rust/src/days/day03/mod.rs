use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

pub fn mod_main(args: Vec<String>) -> Result<(), Error> {
    if args.len() != 1 {
        panic!("Please provide a single input file: input.txt")
    } else {
        let input = File::open(&args[0])?;
        let buffered = BufReader::new(input);

        // Build the scores HashMap
        let mut scores: HashMap<char, i32> = HashMap::new();
        for (c, score) in (b'a'..=b'z').map(|c| c as char).zip(1..=26) {
            println!("Adding score of {:?} for {:?}", score, c);
            scores.insert(c, score);
        }
        for (c, score) in (b'A'..=b'Z').map(|c| c as char).zip(27..=52) {
            println!("Adding score of {:?} for {:?}", score, c);
            scores.insert(c, score);
        }
        let mut part1_total = 0;

        for line_res in buffered.lines() {
            let line = line_res?;
            let items: Vec<char> = line.trim().chars().collect();
            if items.len() % 2 != 0 {
                panic!("Rucksack contains uneven number of elements")
            }

            let mut front: HashSet<char> = HashSet::new();
            for item in (&items[0..items.len() / 2]).iter().cloned() {
                front.insert(item);
            }
            let mut back: HashSet<char> = HashSet::new();
            for item in (&items[(items.len() / 2)..]).iter().cloned() {
                back.insert(item);
            }

            println!("--------------------------");
            println!("Line is: {:?}", line);
            println!("The front compatment contains: {:?}", front);
            println!("The back compatment contains: {:?}", back);
            if front.intersection(&back).count() > 1 {
                panic!("Uhoh, more than one dup item");
            }
            let dup_item = front.intersection(&back).nth(0);

            match dup_item {
                Some(c) => match scores.get(c) {
                    Some(score) => {
                        part1_total += score;
                        println!(
                            "The shared item is: {:?} which has a score of {:?}",
                            c, score
                        );
                    }
                    None => panic!("!!No score for {:?}", c),
                },
                None => {
                    panic!("!!No dup item");
                }
            }
        }
        println!("Part 1 total scores: {:?}", part1_total);
    }

    Ok(())
}
