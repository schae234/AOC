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

        // Build the scores HashMap a-z are 1-26 and A-Z are 27-52
        let mut scores: HashMap<char, i32> = HashMap::new();
        for (c, score) in (b'a'..=b'z').map(|c| c as char).zip(1..=26) {
            scores.insert(c, score);
        }
        for (c, score) in (b'A'..=b'Z').map(|c| c as char).zip(27..=52) {
            scores.insert(c, score);
        }
        let mut part1_total = 0;

        let mut badges = Vec::new();
        let mut part2_total = 0;

        for line_res in buffered.lines() {
            let line = line_res?;
            let items: Vec<char> = line.trim().chars().collect();
            if items.len() % 2 != 0 {
                panic!("Rucksack contains uneven number of elements")
            }

            // Part 1

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

            // Part 2
            let mut badge: HashSet<char> = HashSet::new();
            for item in items.iter().cloned() {
                badge.insert(item);
            }

            badges.push(badge);
            if badges.len() == 3 {
                let ab_id_badge: HashSet<char> =
                    badges[0].intersection(&badges[1]).cloned().collect();
                let id_badge = ab_id_badge.intersection(&badges[2]).nth(0).unwrap();
                part2_total += scores[id_badge];
                badges.clear();
            }
        }
        println!("Part 1 total scores: {:?}", part1_total);
        println!("Part 2 total scores: {:?}", part2_total);
    }

    Ok(())
}
