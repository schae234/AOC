use std::cmp::Ordering;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

type Height = u32;
type Row = Vec<Height>;

struct Grid {
    rows: Vec<Row>,
}

enum Direction {
    North,
    East,
    South,
    West
}

impl Grid {
    fn width(&self) -> usize {
        self.rows.get(0).unwrap().len()
    }
    fn height(&self) -> usize {
        self.rows.len()
    }
    fn tree_height(&self, r: usize, c: usize) -> Height {
        *self.rows.get(r).unwrap().get(c).unwrap()
    }

    fn tree_line(&self, r: usize, c: usize, d: Direction) -> Vec<Height>{
        match d {
            Direction::North => { self.rows[0..r].iter().rev().map(|x| *x.get(c).unwrap()).collect()},
            Direction::South => { self.rows[r..].iter().skip(1).map(|x| *x.get(c).unwrap()).collect()},

            Direction::East => { self.rows.get(r).unwrap()[c..].iter().skip(1).map(|&x| x).collect()},
            Direction::West => { self.rows.get(r).unwrap()[0..c].iter().rev().map(|&x| x).collect()},
        }
    }

    fn west_viz(&self, r: usize, c: usize) -> bool {
        if self.rows.get(r).unwrap()[0..c]
            .iter()
            .all(|&x| x < self.tree_height(r, c))
        {
            return true;
        }
        return false;
    }
    fn east_viz(&self, r: usize, c: usize) -> bool {
        if self.rows.get(r).unwrap()[c + 1..]
            .iter()
            .all(|&x| x < self.tree_height(r, c))
        {
            return true;
        }
        return false;
    }

    fn north_viz(&self, r: usize, c: usize) -> bool {
        if self.rows[0..r]
            .iter()
            .map(|r| r[c])
            .all(|x| x < self.tree_height(r, c))
        {
            return true;
        }
        false
    }
    fn south_viz(&self, r: usize, c: usize) -> bool {
        if self.rows[r + 1..]
            .iter()
            .map(|r| r[c])
            .all(|x| x < self.tree_height(r, c))
        {
            return true;
        }
        false
    }

    fn is_visible(&self, r: usize, c: usize) -> bool {
        // trees around the perimeter are always visible
        if r == 0 || r == self.height() || c == 0 || c == self.width() {
            return true;
        } else {
            //west visible
            if self.west_viz(r, c) {
                return true;
            }
            false
        }
    }

    fn num_visible(&self) -> usize {
        let mut num_visible = 0;
        for r in 0..self.height() {
            for c in 0..self.width() {
                if self.east_viz(r, c)
                    || self.west_viz(r, c)
                    || self.north_viz(r, c)
                    || self.south_viz(r, c)
                {
                    num_visible += 1;
                }
            }
        }
        return num_visible;
    }

    fn scenic_score(&self, r: usize, c: usize) -> usize {
        println!("Scenic score for {r},{c}");
        let cur_height = self.tree_height(r, c);
        let mut north_score = 0;
        let start = if r == 0 { 0 } else { r - 1 };
        for ri in start..=0 {
            println!("N:{ri},{c}");
            match cur_height.cmp(&self.tree_height(ri, c)) {
                Ordering::Greater => north_score += 1,
                Ordering::Equal => {
                    north_score += 1;
                    break;
                }
                Ordering::Less => break,
            };
        }
        let mut south_score = 0;
        let start = if r == self.height() {
            self.height()
        } else {
            r + 1
        };
        for ri in start..self.height() {
            println!("S:{ri},{c}");
            match cur_height.cmp(&self.tree_height(ri, c)) {
                Ordering::Greater => south_score += 1,
                Ordering::Equal => {
                    south_score += 1;
                    break;
                }
                Ordering::Less => break,
            };
        }
        let mut east_score = 0;
        let start = if c == self.width() {
            self.width()
        } else {
            c + 1
        };
        for ci in c + start..self.width() {
            println!("E:{r},{ci}");
            match cur_height.cmp(&self.tree_height(r, ci)) {
                Ordering::Greater => east_score += 1,
                Ordering::Equal => {
                    east_score += 1;
                    break;
                }
                Ordering::Less => break,
            };
        }
        let mut west_score = 0;
        let start = if c == 0 { 0 } else { c - 1 };

        for ci in start..=0 {
            println!("W:{r},{ci}");
            match cur_height.cmp(&self.tree_height(r, ci)) {
                Ordering::Greater => west_score += 1,
                Ordering::Equal => {
                    west_score += 1;
                    break;
                }
                Ordering::Less => break,
            };
        }
        dbg!(north_score);
        dbg!(east_score);
        dbg!(south_score);
        dbg!(west_score);
        north_score * east_score * south_score * west_score
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} by {} Grid:\n", self.width(), self.height())?;
        for r in self.rows.iter() {
            for c in r.iter() {
                write!(f, "{}", c)?;
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

    let input = File::open(&args[0])?;

    let grid = Grid {
        rows: BufReader::new(input)
            .lines()
            .map(|x| {
                Row::from(
                    x.unwrap()
                        .chars()
                        .map(|x| x.to_digit(10).unwrap())
                        .collect::<Vec<Height>>(),
                )
            })
            .collect(),
    };

    println!("{}", grid);
    // Tests
    if args[0].ends_with("example.txt") {
        assert!(
            grid.tree_height(1, 3) == 1,
            "At r1,c3 the tree wasnt 1 tall"
        );
        assert!(
            grid.west_viz(1, 1) == true,
            "1,1 is West not visible and it should be"
        );
        assert!(
            grid.west_viz(2, 1) == false,
            "2,1 is West visible and it shouldnt be"
        );

        assert!(grid.east_viz(1, 2) == true);
        assert!(grid.east_viz(0, 2) == false);

        assert!(grid.north_viz(3, 4) == true);
        assert!(grid.north_viz(2, 2) == false);

        assert!(grid.south_viz(3, 2) == true);
        assert!(grid.south_viz(3, 3) == false);

        println!("Grid Tree Lines at (4,4):");
        println!("N:{:?}",grid.tree_line(4,4,Direction::North));
        println!("E:{:?}",grid.tree_line(4,4,Direction::East));
        println!("S:{:?}",grid.tree_line(4,4,Direction::South));
        println!("W:{:?}",grid.tree_line(4,4,Direction::West));
    }

    // Part1
    println!("Part 1, num visible: {}", grid.num_visible());

    Ok(())
}
