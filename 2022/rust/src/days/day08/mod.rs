use std::cmp::Ordering;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

type Height = u32;
type Row = Vec<Height>;

enum Direction {
    North,
    East,
    South,
    West,
}

struct Grid {
    rows: Vec<Row>,
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

    fn tree_line(&self, r: usize, c: usize, d: Direction) -> Vec<Height> {
        match d {
            Direction::North => self.rows[0..r]
                .iter()
                .rev()
                .map(|x| *x.get(c).unwrap())
                .collect(),
            Direction::South => self.rows[r..]
                .iter()
                .skip(1)
                .map(|x| *x.get(c).unwrap())
                .collect(),

            Direction::East => self.rows.get(r).unwrap()[c..]
                .iter()
                .skip(1)
                .map(|&x| x)
                .collect(),
            Direction::West => self.rows.get(r).unwrap()[0..c]
                .iter()
                .rev()
                .map(|&x| x)
                .collect(),
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

    #[allow(dead_code)]
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

    fn tree_line_scenic_score(&self, r: usize, c: usize, d: Direction) -> usize {
        let cur_height = self.tree_height(r, c);
        let mut score = 0;
        for t in self.tree_line(r, c, d) {
            match cur_height.cmp(&t) {
                Ordering::Greater => score += 1,
                Ordering::Equal | Ordering::Less => {
                    score += 1;
                    break;
                }
            }
        }
        return score;
    }

    fn scenic_score(&self, r: usize, c: usize) -> usize {
        let north_score = self.tree_line_scenic_score(r, c, Direction::North);
        let south_score = self.tree_line_scenic_score(r, c, Direction::South);
        let east_score = self.tree_line_scenic_score(r, c, Direction::East);
        let west_score = self.tree_line_scenic_score(r, c, Direction::West);
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

        println!("Grid Tree Lines at (0,0):");
        println!("N:{:?}", grid.tree_line(0, 0, Direction::North));
        println!("E:{:?}", grid.tree_line(0, 0, Direction::East));
        println!("S:{:?}", grid.tree_line(0, 0, Direction::South));
        println!("W:{:?}", grid.tree_line(0, 0, Direction::West));
    }

    // Part1
    println!("Part 1, num visible: {}", grid.num_visible());

    // Part2
    let mut max_score = 0;
    for (r, rv) in grid.rows.iter().enumerate() {
        for (c, _) in rv.iter().enumerate() {
            let score = grid.scenic_score(r, c);
            if score > max_score {
                max_score = score;
            }
        }
    }
    println!("Part 2 max score: {max_score}");

    Ok(())
}
