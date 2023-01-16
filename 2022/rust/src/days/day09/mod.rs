use std::cell::Cell;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn from_str(s: &str) -> Result<Direction, String> {
        return match s {
            "U" => Ok(Direction::Up),
            "R" => Ok(Direction::Right),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            &_ => Err("Invalid direction input".to_string()),
        };
    }
}

#[derive(Default, Debug)]
struct Coordinate {
    x: Cell<isize>,
    y: Cell<isize>,
}

#[derive(Default, Debug)]
struct Rope {
    head: Coordinate,
    tail: Coordinate,
}

impl Rope {
    fn move_head(&self, d: Direction, times: isize) -> Result<(), ()> {
        match d {
            Direction::Up => self.head.y.set(self.head.y.get() + times),
            Direction::Right => self.head.x.set(self.head.x.get() + times),
            Direction::Down => self.head.y.set(self.head.y.get() - times),
            Direction::Left => self.head.x.set(self.head.x.get() - times),
        }
        Ok(())
    }

    fn move_tail(&self) -> Result<(), ()> {
        Err(())
    }
}

impl fmt::Display for Rope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ROPE")
    }
}

pub fn mod_main(args: Vec<String>) -> Result<(), Error> {
    if args.len() != 1 {
        panic!("Please provide a single input file: input.txt");
    }

    let rope = Rope::default();
    println!("{:?}", rope);

    for line in BufReader::new(File::open(&args[0])?)
        .lines()
        .map(|x| x.unwrap())
    {
        let (ds, ns) = line.split_once(" ").unwrap();
        let d = Direction::from_str(ds).unwrap();
        let n = str::parse::<isize>(ns).unwrap();
        rope.move_head(d, n);
    }

    println!("{:#?}", rope);
    Ok(())
}
