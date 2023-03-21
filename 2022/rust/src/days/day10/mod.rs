use std::{
    collections::VecDeque,
    fmt,
    fs::File,
    io::{BufRead, BufReader, Error},
};

#[derive(Default, Debug)]
pub struct CPUState {
    total_cycles: isize,
    x_register: isize,
}

pub trait Cycle: fmt::Debug {
    fn is_complete(&self) -> bool;
    fn step(&mut self, state: &mut CPUState) -> ();
}

#[derive(Debug)]
struct Addx {
    num_cycles: usize,
    add_value: isize,
}

impl Cycle for Addx {
    fn is_complete(&self) -> bool {
        let mut complete = false;
        if self.num_cycles == 0 {
            complete = true;
        }
        complete
    }

    fn step(&mut self, state: &mut CPUState) -> () {
        self.num_cycles -= 1;
        state.total_cycles += 1;
        if self.num_cycles == 0 {
            state.x_register += self.add_value;
        }
    }
}

#[derive(Debug)]
struct NOOP {
    num_cycles: usize,
}

impl Cycle for NOOP {
    fn is_complete(&self) -> bool {
        let mut complete = false;
        if self.num_cycles == 0 {
            complete = true;
        }
        complete
    }
    fn step(&mut self, state: &mut CPUState) -> () {
        self.num_cycles -= 1;
        state.total_cycles += 1;
    }
}

#[derive(Default)]
struct CPU {
    instruction_pipeline: VecDeque<Box<dyn Cycle>>,
    state: CPUState,
    signal_cycle: isize,
}

impl CPU {
    fn run(&mut self) -> () {
        let mut part_1_total = 0;
        while self.instruction_pipeline.len() > 0 {
            let mut current_instruction = self.instruction_pipeline.pop_front().unwrap();
            //println!("Currently on instruction {:?}", current_instruction);
            while !current_instruction.is_complete() {
                current_instruction.step(&mut self.state);

                if self.state.total_cycles == self.signal_cycle {
                    println!(
                        "On cycle {:?}, CPU state: {:?} - signal strength: {}",
                        self.state.total_cycles,
                        self.state,
                        self.state.x_register * self.state.total_cycles
                    );
                    self.signal_cycle += 40;
                    part_1_total += self.state.x_register * self.state.total_cycles;
                } else {
                    //println!("On cycle {:?}", self.state.total_cycles);
                }
            }
        }
        println!("Part 1 total: {}", part_1_total);
    }
}

pub fn mod_main(args: Vec<String>) -> Result<(), Error> {
    if args.len() != 1 {
        panic!("Please provide a single input file: input.txt");
    }

    let mut cpu = CPU::default();
    cpu.state.x_register = 1;
    cpu.signal_cycle = 20;
    cpu.state.total_cycles = 1;

    for line in BufReader::new(File::open(&args[0])?).lines().map(|x| x.unwrap()) {
        if line.starts_with("noop") {
            cpu.instruction_pipeline.push_back(Box::new(NOOP { num_cycles: 1 }));
        } else if line.starts_with("addx") {
            let (_, arg) = line.split_once(" ").unwrap();
            cpu.instruction_pipeline.push_back(Box::new(Addx {
                num_cycles: 2,
                add_value: arg.clone().parse::<isize>().unwrap(),
            }));
        } else {
            panic!("Unable to create instruction from line: {:?}", line)
        }
    }
    cpu.run();

    Ok(())
}
