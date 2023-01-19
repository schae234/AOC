use std::rc::Rc;
use std::cell::RefCell;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::fmt;

// Working through examples here:
// https://fasterthanli.me/series/advent-of-code-2022/part-7

type NodeHandle = Rc<RefCell<Node>>;

#[allow(dead_code)]
#[derive(Default)]
struct Node {
    name: String,
    size: i32,
    children: Vec<NodeHandle>,
    parent: Option<NodeHandle>,
}
impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node")
            .field("name", &self.name)
            .field("size", &self.size)
            .field("children", &self.children)
            .finish()
    }
}

impl Node {
    fn print_tree(&self, ilevel: usize) {
        if self.size == 0 {
            println!("{} - {} (dir) /"," ".repeat(ilevel), self.name);
        } else {
            println!("{} - {} (file, size={})"," ".repeat(ilevel), self.name,  self.size);
        }
        for child in &self.children {
            child.borrow().print_tree(ilevel+1);
        }
    }

    fn is_dir(&self) -> bool {
        match self.size {
            0 => true,
            _ => false
        }
    }

    fn total_size(&self) -> u64 {
        self.children
            .iter()
            .map(|child| child.borrow().total_size())
            .sum::<u64>()
            + self.size as u64
    }
}

fn all_dirs(n: NodeHandle) -> Box<dyn Iterator<Item = NodeHandle>> {
    
    // This is needed to make clippy happy. Collect the children into a vector
    #[allow(clippy::needless_collect)]
    let children = n.borrow().children.iter().cloned().collect::<Vec<_>>();

    // Return 
    Box::new(
        std::iter::once(n).chain(
            children
                .into_iter()
                .filter_map(|c| {
                    if c.borrow().is_dir() {
                        Some(all_dirs(c))
                    } else {
                        None
                    }
                })
                .flatten(),
        ),
    )
}


pub fn mod_main(args: Vec<String>) -> Result<(), Error> {
    if args.len() != 1 {
        panic!("Please provide a single input file: input.txt");
    }

    let root = Rc::new(RefCell::new(Node{name:String::from("/"), size: 0, children: Vec::new(), parent: None}));
    let mut cwd = root.clone();
    let input = File::open(&args[0])?;
    let buffered = BufReader::new(input);

    for line_res in buffered.lines() {
        let full_line = line_res?;
        let line = full_line.trim();
        if line.starts_with("$") {
            // we have a command
            let cmd: Vec<&str> = line.split(" ").collect();
            match cmd[1] {
                "ls" => {
                    // ls command
                    // ignore this, the listed dirs will be added to cwd in the ls entry branch below
                }
                "cd" => {
                    // cd command, add a new node if name not in children
                    let name = *cmd
                        .get(2)
                        .unwrap_or_else(|| panic!("unable to cd into {}", line));
                    if name == "/" && cwd.borrow().name == "/" {
                        continue;
                    } else {
                        if name == ".." {
                            match &cwd.clone().borrow().parent {
                                Some(new_cwd) => {cwd = new_cwd.clone()}
                                None => {cwd = cwd;}
                            }
                        } else {
                            if cwd.borrow().children.iter().any(|x| x.borrow().name.as_str() == name) {
                                for child in cwd.clone().borrow().children.iter() {
                                    if child.borrow().name == name {
                                        cwd = child.clone();
                                    }
                                }
                            } else {
                                let child = Rc::new(RefCell::new(Node {
                                    name: name.to_string(),
                                    size: 0,
                                    children: Vec::new(),
                                    parent: Some(cwd.clone())
                                }));
                                cwd.borrow_mut().children.push(child.clone());
                                cwd = child;
                            }
                        }
                    }
                }
                _ => panic!("Command not recognized!")
            }
        } else {
            // we have an ls entry
            let parts: Vec<&str> = line.split(" ").collect();
            // directory entry
            let name = *parts
                .get(1)
                .unwrap_or_else(|| panic!("cannot to get dir name on line: {}", line));
            match parts[0] {
                "dir" => {
                    let child = Node{
                        name: parts.get(1).unwrap().to_string(),
                        size: 0,
                        children: Vec::new(),
                        parent:Some(cwd.clone())
                    };
                    cwd.borrow_mut().children.push(Rc::new(RefCell::new(child)));
                }
                _ => {
                    // file v
                    let size = parts
                        .get(0)
                        .unwrap_or_else(
                            || panic!("cannot to get file size on line: {}", line)
                        )
                        .parse::<i32>()
                        .unwrap_or_else(|_| {
                            panic!("unable to parse file size from line: {}", line)
                        });
                    let child = Node{
                        name: name.to_string(),
                        size,
                        children: Vec::new(),
                        parent: Some(cwd.clone()),
                    }; 
                    cwd.borrow_mut().children.push(Rc::new(RefCell::new(child)));
                }
            }
        }
    } 
    root.borrow().print_tree(0);
   
    // Part 1
    let part1 = all_dirs(root.clone())
    .map(|d| d.borrow().total_size())
    .filter(|&s| s <= 100_000)
    .inspect(|s| {
        dbg!(s);
    })
    .sum::<u64>();
    println!("Part 1 total: {part1}");

    // Part2
    let current_unused_space = 70000000 - root.borrow().total_size();
    println!("Total root size: {}",root.borrow().total_size());
    println!("Total unused size: {}", current_unused_space);
    let target_size: i64 = 30000000 - current_unused_space as i64;
    println!("Target size: {}", target_size);
    let part2: (i64,i64) = all_dirs(root)
    .map(
        |d| (d.borrow().total_size() as i64 - target_size, d.borrow().total_size() as i64)
    )
    .filter(|&a| a.0 > 0)
    .min_by(|a,b| a.0.cmp(&b.0))
    .unwrap();

    println!("Part2 - target diff: {} dir size:{}", part2.0, part2.1);

    Ok(())
}
