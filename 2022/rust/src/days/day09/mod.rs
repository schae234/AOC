use std::cell::Cell;
use std::cmp::Ordering;
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
}
enum Dimension {
    X,
    Y,
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
    knots: [Coordinate; 10],
    tail_been: HashSet<Coordinate>,
}

impl Rope {
    fn move_head(&mut self, d: Direction) {
        match d {
            Direction::Up => self.knots[0].y.set(self.knots[0].y.get() + 1),
            Direction::Right => self.knots[0].x.set(self.knots[0].x.get() + 1),
            Direction::Down => self.knots[0].y.set(self.knots[0].y.get() - 1),
            Direction::Left => self.knots[0].x.set(self.knots[0].x.get() - 1),
        }
        for i in 1..self.knots.len() {
            self.move_knot(i);
        }
        self.tail_been.insert(self.knots[self.knots.len() - 1].clone());
    }

    fn is_stretched(&self, i: usize) -> Option<Dimension> {
        if i == 0 {
            return None;
        }
        if (self.knots[i].x.get() - self.knots[i - 1].x.get()).abs() >= 2 {
            return Some(Dimension::X);
        } else if (self.knots[i].y.get() - self.knots[i - 1].y.get()).abs() >= 2 {
            return Some(Dimension::Y);
        } else {
            return None;
        }
    }

    fn move_knot(&mut self, i: usize) -> Option<()> {
        match self.is_stretched(i) {
            Some(dim) => {
                match self.knots[i - 1].x.get().cmp(&self.knots[i].x.get()) {
                    // Same row
                    Ordering::Equal => match self.knots[i - 1].y.get().cmp(&self.knots[i].y.get()) {
                        Ordering::Greater => self.knots[i].y.set(self.knots[i - 1].y.get() - 1),
                        Ordering::Less => self.knots[i].y.set(self.knots[i - 1].y.get() + 1),
                        Ordering::Equal => { /*Head and tail are in the same spot*/ }
                    },
                    Ordering::Greater => match self.knots[i - 1].y.get().cmp(&self.knots[i].y.get()) {
                        Ordering::Greater => match dim {
                            Dimension::X => {
                                self.knots[i].y.set(self.knots[i - 1].y.get());
                                self.knots[i].x.set(self.knots[i - 1].x.get() - 1);
                            }
                            Dimension::Y => {
                                self.knots[i].x.set(self.knots[i - 1].x.get());
                                self.knots[i].y.set(self.knots[i - 1].y.get() - 1);
                            }
                        },
                        Ordering::Less => match dim {
                            Dimension::X => {
                                self.knots[i].y.set(self.knots[i - 1].y.get());
                                self.knots[i].x.set(self.knots[i - 1].x.get() - 1);
                            }
                            Dimension::Y => {
                                self.knots[i].x.set(self.knots[i - 1].x.get());
                                self.knots[i].y.set(self.knots[i - 1].y.get() + 1);
                            }
                        },
                        Ordering::Equal => self.knots[i].x.set(self.knots[i - 1].x.get() - 1),
                    },
                    Ordering::Less => match self.knots[i - 1].y.get().cmp(&self.knots[i].y.get()) {
                        Ordering::Greater => match dim {
                            Dimension::X => {
                                self.knots[i].y.set(self.knots[i - 1].y.get());
                                self.knots[i].x.set(self.knots[i - 1].x.get() + 1);
                            }
                            Dimension::Y => {
                                self.knots[i].x.set(self.knots[i - 1].x.get());
                                self.knots[i].y.set(self.knots[i - 1].y.get() - 1);
                            }
                        },
                        Ordering::Less => match dim {
                            Dimension::X => {
                                self.knots[i].y.set(self.knots[i - 1].y.get());
                                self.knots[i].x.set(self.knots[i - 1].x.get() + 1);
                            }
                            Dimension::Y => {
                                self.knots[i].x.set(self.knots[i - 1].x.get());
                                self.knots[i].y.set(self.knots[i - 1].y.get() + 1);
                            }
                        },
                        Ordering::Equal => self.knots[i].x.set(self.knots[i - 1].x.get() + 1),
                    },
                }
            }
            None => {}
        }
        Some(())
    }
}

impl fmt::Display for Rope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in (-5..15).rev() {
            for x in -11..=14 {
                // iterate through knots and see if x,y
                let mut overlapped: bool = false;
                for (i, knot) in self.knots.iter().enumerate() {
                    if !overlapped && knot.x.get() == x && knot.y.get() == y {
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
