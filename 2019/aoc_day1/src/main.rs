use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn fuel_required(mass: f32) -> f32 {
    let fuel_req = ((mass / 3.0).floor() - 2.0).max(0.0);
    fuel_req
} 

fn total_fuel_required(mass: f32) -> f32 {
    println!("Calculating fuel required for {:?}", mass);
    let mut fuel_req = fuel_required(mass);
    let mut added_fuel = fuel_required(fuel_req);
    loop {
        println!("Need to add {:?} fuel for the fuel", added_fuel);
        fuel_req  += added_fuel;
        added_fuel = fuel_required(added_fuel);
        if added_fuel == 0.0 {
            break;
        }
    }
    println!("Total fuel needed for module: {:?}", fuel_req);
    fuel_req
}

#[test]
fn test_fuel_req(){
    assert_eq!(fuel_required(12.0),2.0);
}

#[test]
fn test_fuel_req2(){
    assert_eq!(fuel_required(14.0),2.0);
}

#[test]
fn test_fuel_req3(){
    assert_eq!(fuel_required(1969.0),654.0);
}

#[test]
fn test_fuel_req4(){
    assert_eq!(fuel_required(100756.0),33583.0);
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Please provide only one argument: a file with module masses")
    }
    else {
        let mass_file = &args[1];
        println!("Calculating the fuel needed for masses in {:?}", mass_file);

        let input = File::open(mass_file)?;
        let buffered = BufReader::new(input);

        let mut total_fuel: f32 = 0.0;

        for line in buffered.lines() {
            match line {
                Err(why)   => panic!("{:?}",why),
                Ok(string) => match string.trim().parse::<f32>() {
                    Err(why)   => panic!("Not a number: {:?}",why),
                    Ok(number) => {
                        total_fuel += total_fuel_required(number)
                    }
                } 
            }
        }
        println!("Total fuel needed for ALL modules: {:?}",total_fuel);
        // Now we need to add fuel for the fuel
    }
    Ok(())
}
