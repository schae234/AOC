use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader, Error};

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
    NE,
    SE,
    SW,
    NW,
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

#[derive(Default, Debug, Eq, PartialEq, Clone, Copy, Hash)]
struct Coordinate {
    x: isize,
    y: isize,
}

impl Coordinate {
    fn distance(&self, c: Coordinate) -> (isize, isize) {
        let x = self.x - c.x;
        let y = self.y - c.y;
        return (x, y);
    }
    fn slide(&mut self, d: Direction) -> Option<()> {
        match d {
            Direction::Up => self.y = self.y + 1,
            Direction::Right => self.x = self.x + 1,
            Direction::Down => self.y = self.y - 1,
            Direction::Left => self.x = self.x - 1,
            Direction::NE => {
                self.x = self.x + 1;
                self.y = self.y + 1;
            }
            Direction::SE => {
                self.x = self.x + 1;
                self.y = self.y - 1;
            }
            Direction::SW => {
                self.x = self.x - 1;
                self.y = self.y - 1;
            }
            Direction::NW => {
                self.x = self.x - 1;
                self.y = self.y + 1;
            }
        }
        return None;
    }
}

#[derive(Default, Debug)]
struct Rope {
    knots: [Coordinate; 10],
    tail_been: HashSet<Coordinate>,
}

impl Rope {
    // Moves the head of the rope and update the position of the rest of the knots
    fn move_head(&mut self, d: Direction) {
        self.knots[0].slide(d);
        for i in 0..self.knots.len() - 1 {
            self.move_tail(i, i + 1);
        }
        let tail = self.knots[self.knots.len() - 1].clone();
        self.tail_been.insert(tail);
    }

    // Moves the tail based on the position of the head
    fn move_tail(&mut self, i_head: usize, i_tail: usize) -> Option<()> {
        // Get the distance before we get the mutable reference to head because we can only have 1
        // mut reference at a time
        let distance = self.knots[i_head].distance(self.knots[i_tail]);
        // Get the mutable reference to head and act on the distance b/w head and tail. Self.knots
        // owns the knots so we can only have 1 mut reference to it until this scope is up
        let tail: &mut Coordinate = &mut self.knots[i_tail];
        match distance {
            // Cardinal stretching
            (0, 2) => tail.slide(Direction::Up)?,
            (2, 0) => tail.slide(Direction::Right)?,
            (0, -2) => tail.slide(Direction::Down)?,
            (-2, 0) => tail.slide(Direction::Left)?,
            // Diagonal stretching
            (1, 2) | (2, 1) | (2, 2) => tail.slide(Direction::NE)?,
            (2, -1) | (1, -2) | (2, -2) => tail.slide(Direction::SE)?,
            (-1, -2) | (-2, -1) | (-2, -2) => tail.slide(Direction::SW)?,
            (-2, 1) | (-1, 2) | (-2, 2) => tail.slide(Direction::NW)?,
            // If at any of the coorindates that are 0 or 1 away from head, do nothing
            (0, 0) | (0, 1) | (1, 1) | (1, 0) | (1, -1) | (-1, -1) | (-1, 0) | (-1, 1) | (0, -1) => {}
            // Panic if we observe any unexpected coordinates
            _ => panic!("Unable to slide for distance: {:?}", distance),
        }

        return None;
    }
}

impl fmt::Display for Rope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in (-5..15).rev() {
            write!(f, "{:>5}", y)?;
            for x in -11..=14 {
                // iterate through knots and see if x,y
                let mut overlapped: bool = false;
                for (i, knot) in self.knots.iter().enumerate() {
                    if !overlapped && knot.x == x && knot.y == y {
                        write!(f, "{}", i)?;
                        overlapped = true;
                    }
                }
                if !overlapped {
                    write!(f, ".")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

pub fn mod_main(args: Vec<String>) -> Result<(), Error> {
    if args.len() != 1 {
        panic!("Please provide a single input file: input.txt");
    }

    let mut rope = Rope::default();
    println!("== Initial State ==");
    println!("{}", rope);

    for line in BufReader::new(File::open(&args[0])?).lines().map(|x| x.unwrap()) {
        println!("== {} ==", line.clone());
        let (ds, ns) = line.split_once(" ").unwrap();
        let d = Direction::from_str(ds).unwrap();
        let n = str::parse::<isize>(ns).unwrap();
        for _ in 1..=n {
            rope.move_head(d.clone());
        }
        println!("{}", rope);
    }
    println!("Tail has been to {} coordinates", rope.tail_been.len());
    Ok(())
}
