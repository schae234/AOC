use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

#[derive(Debug)]
struct Row(Vec<char>);
impl Row {
    fn get_col(&self, col: i32) -> char {
        return self.0[1 + ((col - 1) * 4) as usize];
    }
}

#[derive(Debug)]
struct Move(Vec<String>);
impl Move {
    fn from_stack(&self) -> i32 {
        return self.0[3].parse::<i32>().unwrap();
    }
    fn to_stack(&self) -> i32 {
        return self.0[5].parse::<i32>().unwrap();
    }
    fn times(&self) -> i32 {
        return self.0[1].parse::<i32>().unwrap();
    }
}

pub fn mod_main(args: Vec<String>) -> Result<(), Error> {
    if args.len() != 1 {
        panic!("Please provide a single input file: input.txt");
    }
    let input = File::open(&args[0])?;
    let buffered = BufReader::new(input);
    let mut config: Vec<Row> = Vec::new();
    let mut stacks: BTreeMap<i32, Vec<char>> = BTreeMap::new();
    let mut stacks2: BTreeMap<i32, Vec<char>> = BTreeMap::new();
    let mut moves: Vec<Move> = Vec::new();
    let mut in_config = true;
    for line_res in buffered.lines() {
        let line = line_res?;
        if in_config == true {
            if line.starts_with(" 1 ") {
                for col in line.trim().split("   ").map(|x| x.parse::<i32>().unwrap()) {
                    stacks.insert(col, Vec::new());
                    stacks2.insert(col, Vec::new());
                }
                in_config = false;
            } else {
                config.insert(0, Row(line.chars().collect()));
            }
        } else {
            if line.starts_with("move") {
                moves.push(Move(
                    line.trim().split(" ").map(|x| x.to_string()).collect(),
                ));
            }
        }
    }

    for row in config {
        for col in (1..=stacks.len()).map(|x| x as i32) {
            let val = row.get_col(col);
            if !val.is_whitespace() {
                stacks.get_mut(&col).unwrap().push(val);
                stacks2.get_mut(&col).unwrap().push(val);
            }
        }
    }
    println!("Starting Stacks");
    for key in stacks2.keys() {
        println!("{:?} : {:?}", key, stacks2[key])
    }

    for m in moves {
        // part1
        println!("Executing Move: {:?}", m);
        let from = m.from_stack();
        let to = m.to_stack();
        /*
        for _ in 1..=m.times() {
            let item = stacks.get_mut(&from).unwrap().pop().unwrap();
            stacks.get_mut(&to).unwrap().push(item);
        }
        */

        // part2
        let mut crate_stack: Vec<char> = Vec::new();
        for _ in 1..=m.times() {
            let item = stacks2.get_mut(&from).unwrap().pop().unwrap();
            crate_stack.push(item);
        }
        for _ in 1..=m.times() {
            stacks2
                .get_mut(&to)
                .unwrap()
                .push(crate_stack.pop().unwrap());
        }
    }
    println!("Part1: Ending Stacks");
    for key in stacks.keys() {
        println!("{:?} : {:?}", key, stacks[key])
    }

    println!("Part2: Ending Stacks");
    for key in stacks2.keys() {
        println!("{:?} : {:?}", key, stacks2[key])
    }

    Ok(())
}
