use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::hash::{Hash, Hasher};
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

#[derive(Default, Debug, Eq, PartialEq, Clone)]
struct Coordinate {
    x: Cell<isize>,
    y: Cell<isize>,
}

impl Hash for Coordinate {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.get().hash(state);
        self.y.get().hash(state);
    }
}

#[derive(Default, Debug)]
struct Rope {
    head: Coordinate,
    tail: Coordinate,
    tail_been: HashSet<Coordinate>,
}

enum Dimension {
    X,
    Y,
}

impl Rope {
    fn move_head(&mut self, d: Direction, times: isize) {
        for _ in 0..times {
            match d {
                Direction::Up => self.head.y.set(self.head.y.get() + 1),
                Direction::Right => self.head.x.set(self.head.x.get() + 1),
                Direction::Down => self.head.y.set(self.head.y.get() - 1),
                Direction::Left => self.head.x.set(self.head.x.get() - 1),
            }
            self.move_tail();
            println!("{}", self);
        }
    }
    fn is_stretched(&self) -> Option<Dimension> {
        if (self.head.x.get() - self.tail.x.get()).abs() >= 2 {
            return Some(Dimension::X);
        } else if (self.head.y.get() - self.tail.y.get()).abs() >= 2 {
            return Some(Dimension::Y);
        } else {
            return None;
        }
    }

    fn move_tail(&mut self) -> Option<()> {
        match self.is_stretched() {
            Some(dim) => {
                match self.head.x.get().cmp(&self.tail.x.get()) {
                    // Same row
                    Ordering::Equal => match self.head.y.get().cmp(&self.tail.y.get()) {
                        Ordering::Greater => self.tail.y.set(self.head.y.get() - 1),
                        Ordering::Less => self.tail.y.set(self.head.y.get() + 1),
                        Ordering::Equal => { /*Head and tail are in the same spot*/ }
                    },
                    Ordering::Greater => match self.head.y.get().cmp(&self.tail.y.get()) {
                        Ordering::Greater => match dim {
                            Dimension::X => {
                                self.tail.y.set(self.head.y.get());
                                self.tail.x.set(self.head.x.get() - 1);
                            }
                            Dimension::Y => {
                                self.tail.x.set(self.head.x.get());
                                self.tail.y.set(self.head.y.get() - 1);
                            }
                        },
                        Ordering::Less => match dim {
                            Dimension::X => {
                                self.tail.y.set(self.head.y.get());
                                self.tail.x.set(self.head.x.get() - 1);
                            }
                            Dimension::Y => {
                                self.tail.x.set(self.head.x.get());
                                self.tail.y.set(self.head.y.get() + 1);
                            }
                        },
                        Ordering::Equal => self.tail.x.set(self.head.x.get() - 1),
                    },
                    Ordering::Less => match self.head.y.get().cmp(&self.tail.y.get()) {
                        Ordering::Greater => match dim {
                            Dimension::X => {
                                self.tail.y.set(self.head.y.get());
                                self.tail.x.set(self.head.x.get() + 1);
                            }
                            Dimension::Y => {
                                self.tail.x.set(self.head.x.get());
                                self.tail.y.set(self.head.y.get() - 1);
                            }
                        },
                        Ordering::Less => match dim {
                            Dimension::X => {
                                self.tail.y.set(self.head.y.get());
                                self.tail.x.set(self.head.x.get() + 1);
                            }
                            Dimension::Y => {
                                self.tail.x.set(self.head.x.get());
                                self.tail.y.set(self.head.y.get() + 1);
                            }
                        },
                        Ordering::Equal => self.tail.x.set(self.head.x.get() + 1),
                    },
                }
            }
            None => {}
        }
        self.tail_been.insert(self.tail.clone());
        Some(())
    }
}

impl fmt::Display for Rope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in (0..5).rev() {
            for x in 0..=5 {
                if self.head.x.get() == x && self.head.y.get() == y {
                    write!(f, "H")?;
                } else if self.tail.x.get() == x && self.tail.y.get() == y {
                    write!(f, "T")?;
                } else {
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

    for line in BufReader::new(File::open(&args[0])?)
        .lines()
        .map(|x| x.unwrap())
    {
        println!("== {} ==", line.clone());
        let (ds, ns) = line.split_once(" ").unwrap();
        let d = Direction::from_str(ds).unwrap();
        let n = str::parse::<isize>(ns).unwrap();
        rope.move_head(d, n);
    }
    println!("Tail has been to {} coordinates", rope.tail_been.len());
    Ok(())
}
