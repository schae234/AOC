use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

enum Throw {
    Rock,
    Paper,
    Scissor,
}

impl Throw {
    fn new(val: &str) -> Throw {
        match val {
            "A" => Throw::Rock,
            "B" => Throw::Paper,
            "C" => Throw::Scissor,
            "X" => Throw::Rock,
            "Y" => Throw::Paper,
            "Z" => Throw::Scissor,
            _ => panic!("Invalid Throw: {:?}", val),
        }
    }

    fn value(&self) -> i32 {
        match *self {
            Throw::Rock => 1,
            Throw::Paper => 2,
            Throw::Scissor => 3,
        }
    }

    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Throw::Rock, Throw::Scissor) => Ordering::Greater,
            (Throw::Paper, Throw::Rock) => Ordering::Greater,
            (Throw::Scissor, Throw::Paper) => Ordering::Greater,

            (Throw::Scissor, Throw::Rock) => Ordering::Less,
            (Throw::Rock, Throw::Paper) => Ordering::Less,
            (Throw::Paper, Throw::Scissor) => Ordering::Less,
            _ => Ordering::Equal,
        }
    }
}

struct Hand {
    opponent: Throw,
    you: Throw,
}

impl Hand {
    fn new(opp: &str, you: &str) -> Hand {
        Hand {
            opponent: Throw::new(opp),
            you: Throw::new(you),
        }
    }

    fn score(&self) -> i32 {
        let round_score: i32 = match self.you.cmp(&self.opponent) {
            Ordering::Greater => 6,
            Ordering::Equal => 3,
            Ordering::Less => 0,
        };
        println!("Hand score: {:?}", round_score + self.you.value());
        round_score + self.you.value()
    }
}

pub fn mod_main(args: Vec<String>) -> Result<(), Error> {
    if args.len() != 1 {
        panic!("Please provide a single input file: input.txt")
    } else {
        let input = File::open(&args[0])?;
        let buffered = BufReader::new(input);

        let mut total_score = 0;
        for line_res in buffered.lines() {
            let line = line_res?;
            let fields: Vec<&str> = line.trim().split(" ").collect();
            if fields.len() != 2 {
                panic!("Incorrect number of throws per line: {:?}", fields)
            }
            let hand = Hand::new(fields[0], fields[1]);
            total_score += hand.score();
        }
        println!("Total score: {:?}", total_score)
    }
    Ok(())
}
