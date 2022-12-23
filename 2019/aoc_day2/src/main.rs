use std::fs;
use std::env;
use std::io::{Error, ErrorKind};

struct Computer {
    ptr: usize,
    codes: Vec<i64>
}

impl Computer {

    fn new(codes: Vec<i64>) -> Computer {
        return Computer{
            ptr: 0,
            codes: codes
        } 
    }

    fn len(&self) -> usize {
        return self.codes.len()
    }

    fn current_code(&self) -> i64 {
        self.codes[self.ptr]
    }

    fn add(&mut self) -> () {
        let x = self.codes[self.codes[self.ptr+1] as usize];
        let y = self.codes[self.codes[self.ptr+2] as usize];
        let i = self.codes[self.ptr+3] as usize;
        self.codes[i] = x + y;
        ()
    }

    fn mult(&mut self) -> () {
        let x = self.codes[self.codes[self.ptr+1] as usize];
        let y = self.codes[self.codes[self.ptr+2] as usize];
        let i = self.codes[self.ptr+3] as usize;
        self.codes[i] = x * y;
        ()
       
    }

    fn set_input(&mut self, noun: i64, verb: i64) -> () {
        self.codes[1] = noun;
        self.codes[2] = verb;
        ()
    }

    fn run(&mut self) -> i64 {
        loop {
            //println!("Current program state: {:?}",self.codes);
            let instruction = self.current_code();
            match instruction {
                1 => {
                    //println!("Adding! Found a {} at {}", instruction, self.ptr);
                    self.add(); 
                },
                2 => {
                    //println!("Multiplying! Found a {} at {}", instruction, self.ptr);
                    self.mult();

                },
                99 => {
                    //println!("Aborting!, Found a {} at {}", instruction, self.ptr);
                    break;
                },
                _ => {
                    panic!("Bad instruction")
                }
            }
            self.ptr += 4;
        }
        return self.codes[0]
    }

}


fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(Error::new(ErrorKind::InvalidInput,"Invalid number of arguments"))
    }
    else{
        let mass_file = &args[1];
        println!("Reading in OpCodes from {:?}",mass_file);

        // we can use the ? operator here 
        let opcodes = fs::read_to_string(&mass_file)?;
        let opcodes: Vec<i64> = opcodes
            .trim()
            .split(',')
            .map(|x| x.parse().unwrap()).collect();

        let mut c = Computer::new(opcodes.clone());
        c.set_input(12,2);

        println!("Found {:?} OpCodes",c.len());
        
        let output = c.run();
        println!("Program ended with a {} at position 0",output);

        println!("\n\n\n------------------------------------------------");
        println!("Calculating part 2");

        'outer: for x in 0..100 {
            'inner: for y in 0..100 {
                let mut c = Computer::new(opcodes.clone());
                c.set_input(x,y);
                let output = c.run();
                //println!("Calculating output for {} and {}",x,y);
                if output == 19690720 {
                    println!("To get 19690720, used noun={} and verb={}",x,y);
                    println!("100*noun*verb = {}", 100*x+y);
                    break 'outer;
                }
            }
        }


            

    }
    Ok(())
}

#[test]
fn t1() {
    let mut c = Computer::new(vec!(1,0,0,0,99));
    let x = c.run();
    assert_eq!(x,2);
}

#[test]
fn t2() {
   let mut c = Computer::new(vec!(2,3,0,3,99)); 
   let x = c.run();
   assert_eq!(x,2);
}
#[test]
fn t3() {
    let mut c = Computer::new(vec!(1,1,1,4,99,5,6,0,99));
    let x = c.run();
    assert_eq!(x,30);
}
#[test]
fn t4() {
    let mut c = Computer::new(vec!(1,9,10,3,2,3,11,0,99,30,40,50));
    let x = c.run();
    assert_eq!(x,3500);
}
