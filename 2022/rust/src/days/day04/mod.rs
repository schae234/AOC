use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn calculate_sections(assignment: &str) -> HashSet<i32> {
    let (start, end) = assignment.split_once("-").unwrap();
    let mut sections: HashSet<i32> = HashSet::new();
    for section in start.parse::<i32>().unwrap()..=end.parse::<i32>().unwrap() {
        sections.insert(section);
    }
    sections
}

pub fn mod_main(args: Vec<String>) -> Result<(), Error> {
    if args.len() != 1 {
        panic!("Please provide a single input file: input.txt");
    }
    let input = File::open(&args[0])?;
    let buffered = BufReader::new(input);
    let mut number_exclusive_overlaps = 0;
    let mut number_overlaps = 0;
    for line_res in buffered.lines() {
        let line = line_res?;
        let (elf1_assignment, elf2_assignment) = line.split_once(",").unwrap();

        let elf1_sections = calculate_sections(elf1_assignment);
        let elf2_sections = calculate_sections(elf2_assignment);

        if elf1_sections.is_subset(&elf2_sections) || elf2_sections.is_subset(&elf1_sections) {
            number_exclusive_overlaps += 1;
        }

        if elf1_sections.intersection(&elf2_sections).count() > 0 {
            number_overlaps += 1;
        }
    }
    println!(
        "Part1 - number of exclusive overlaps: {:?}",
        number_exclusive_overlaps
    );
    println!("Part2 - number of overlaps: {:?}", number_overlaps);

    Ok(())
}
