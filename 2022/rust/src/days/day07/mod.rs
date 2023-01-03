use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

// Working through examples here:
// https://fasterthanli.me/series/advent-of-code-2022/part-7

#[derive(Debug, Default)]
struct Node {
    name: String,
    size: i32,
    children: Vec<Node>,
}

pub fn mod_main(args: Vec<String>) -> Result<(), Error> {
    if args.len() != 1 {
        panic!("Please provide a single input file: input.txt");
    }

    let mut cwd = &mut Node {
        name: "/".to_string(),
        size: 0,
        children: Vec::new(),
    };
    let input = File::open(&args[0])?;
    let buffered = BufReader::new(input);

    for line_res in buffered.lines() {
        let line = line_res?.trim();
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
                    if cwd.children.iter().any(|x| x.name.as_str() == name) {
                        cwd = &mut cwd
                            .children
                            .into_iter()
                            .find(|x| x.name.as_str() == name)
                            .unwrap();
                    } else {
                        let mut child = Node {
                            name: name.to_string(),
                            size: 0,
                            children: Vec::new(),
                        };
                        cwd.children.push(child);
                        cwd = &mut child;
                    }
                }
            }
        } else {
            // we have an ls entry
            let parts: Vec<&str> = line.split(" ").collect();
            // directory entry
            let name = *parts
                .get(0)
                .unwrap_or_else(|| panic!("cannot to get dir name on line: {}", line));
            match parts[0] {
                "dir" => {
                    let mut child = Node {
                        name: name.to_string(),
                        size: 0,
                        children: Vec::new(),
                    };
                    cwd.children.push(child);
                }
                _ => {
                    // file entry
                    let size = parts
                        .get(1)
                        .unwrap_or_else(|| panic!("cannot to get file size on line: {}", line))
                        .parse::<i32>()
                        .unwrap_or_else(|_| {
                            panic!("unable to parse file size from line: {}", line)
                        });
                    let mut child = Node {
                        name: name.to_string(),
                        size,
                        children: Vec::new(),
                    };
                    cwd.children.push(child);
                }
            }
        }
    }
    Ok(())
}
