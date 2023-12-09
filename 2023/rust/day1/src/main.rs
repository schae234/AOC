fn main() {
    // Example
    let example = String::from_utf8_lossy(include_bytes!("example.txt")).into_owned();
    let example_answer = extract_calibrations(example);
    println!("Example answer: {example_answer}");

    // Part 1
    let part1 = String::from_utf8_lossy(include_bytes!("part1.txt")).into_owned();
    let part1_answer = extract_calibrations(part1);
    println!("Part1 Answer: {part1_answer}");

    // Example 2
    let example2 = String::from_utf8_lossy(include_bytes!("example2.txt")).into_owned();
    let example2_answer = extract_calibrations_with_spelled(example2);
    println!("Example2 Answer: {example2_answer}");

}


// Extracts the calibrations from the input string
fn extract_calibrations(text: String) -> usize {

    let mut calibration_value: usize = 0;

    for line in text.lines() {

        let numbers: String = line.chars().filter(|x| x.is_digit(10)).collect();
        if numbers.len() >= 1 {
            
            let x: String = numbers.chars().nth(0).expect("Could not get index 0 of string").to_string();
            let y: String = numbers.chars().last().expect("Could not get last index of string").to_string();
            let z = [x,y].join("").parse::<usize>().unwrap();

            calibration_value += z;
        } 
    }

    return calibration_value
}

fn extract_calibrations_with_spelled(text: String) -> usize {

    fn pop_cal(mut text: &str) -> Option<(&str,usize)> {
        let cal_val = match text {
            s @ "zero" if text.starts_with(s) || s.starts_with('0') => {text = text.strip_prefix(s).unwrap(); 0}
            s @ "one" if text.starts_with(s) || s.starts_with('1') => {text = text.strip_prefix(s).unwrap();1}
            s @ "two" if s.starts_with(s) || s.starts_with('2') => {text = text.strip_prefix(s).unwrap();2}
            s @ "three" if s.starts_with(s) || s.starts_with('3') => {text = text.strip_prefix(s).unwrap();3}
            s @ "four" if s.starts_with(s) || s.starts_with('4') => {text = text.strip_prefix(s).unwrap();4}
            s @ "five" if s.starts_with(s) || s.starts_with('5') => {text = text.strip_prefix(s).unwrap();5}
            s @ "six" if s.starts_with(s) || s.starts_with('6') => {text = text.strip_prefix(s).unwrap();6}
            s @ "seven" if s.starts_with(s) || s.starts_with('7') => {text = text.strip_prefix(s).unwrap();7}
            s @ "eight" if s.starts_with(s) || s.starts_with('8') => {text = text.strip_prefix(s).unwrap();8}
            s @ "nine" if s.starts_with(s) || s.starts_with('9') => {text = text.strip_prefix(s).unwrap();9}
            _ => {return None}
        }; 
        return Some((text,cal_val))
    }

    return 42
}
