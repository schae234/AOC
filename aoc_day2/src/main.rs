use std::fs;
use std::env;
use std::error::Error;
use std::{error, fmt};
use std::fmt::Display;


#[derive(Debug)]
struct ArgError {
    v: String,
}

impl ArgError {
    fn new() -> ArgError {
        ArgError{
            v: "Please provide only 1 argument: a file with opCodes".to_string()
        }
    }
}

impl error::Error for ArgError {
    fn description(&self) -> &str { &self.v }
}

impl Display for ArgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MyError: {}", &self.v)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Box::new(ArgError::new())
    }
    else{
        let mass_file = &args[1];
        println!("Reading in OpCodes from {:?}",mass_file);

        // we can use the ? operator here 
        let opcodes = fs::read_to_string(&mass_file)?;
        let opcodes: Vec<u32> = opcodes
            .trim()
            .split(',')
            .map(|x| x.parse().unwrap()).collect();

        println!("Found {:?} OpCodes",opcodes.len());

    }
    Ok(())
}
