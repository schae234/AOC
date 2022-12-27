use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

#[derive(Clone, Copy)]
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
    fn beater(self) -> Throw {
        match self {
            Throw::Rock => Throw::Paper,
            Throw::Paper => Throw::Scissor,
            Throw::Scissor => Throw::Rock,
        }
    }
    fn loser(self) -> Throw {
        match self {
            Throw::Rock => Throw::Scissor,
            Throw::Paper => Throw::Rock,
            Throw::Scissor => Throw::Paper,
        }
    }
    fn drawer(self) -> Throw {
        self.clone()
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
        //println!("Hand score: {:?}", round_score + self.you.value());
        round_score + self.you.value()
    }
}

enum Outcome {
    Win,
    Draw,
    Lose,
}
impl Outcome {
    fn new(val: &str) -> Outcome {
        match val {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("Invalid input for new Outcome: {:?}", val),
        }
    }

    fn hand(self, opp: Throw) -> Hand {
        let you: Throw = match self {
            Outcome::Win => opp.beater(),
            Outcome::Draw => opp.drawer(),
            Outcome::Lose => opp.loser(),
        };
        Hand { opponent: opp, you }
    }
}

pub fn mod_main(args: Vec<String>) -> Result<(), Error> {
    if args.len() != 1 {
        panic!("Please provide a single input file: input.txt")
    } else {
        let input = File::open(&args[0])?;
        let buffered = BufReader::new(input);

        let mut total_score = 0;
        let mut round2_score = 0;
        for line_res in buffered.lines() {
            let line = line_res?;
            let fields: Vec<&str> = line.trim().split(" ").collect();
            if fields.len() != 2 {
                panic!("Incorrect number of throws per line: {:?}", fields)
            }
            // Round 1 score
            let hand = Hand::new(fields[0], fields[1]);
            total_score += hand.score();

            // Round 2 score
            let desired_outcome = Outcome::new(fields[1]);
            let desired_hand = desired_outcome.hand(Throw::new(fields[0]));
            round2_score += desired_hand.score();
        }
        println!("Total round 1 score: {:?}", total_score);
        println!("Total round 2 score: {:?}", round2_score);
    }
    Ok(())
}
