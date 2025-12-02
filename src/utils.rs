

/// Replaces all occurrences of `find` with `replace` in `s`. Returns a bool for if replacements were made.
pub fn replace(s: &str, find: &str, replace: &str) -> (bool, String) {
    let mut out = String::new();
    let find_bytes = find.as_bytes();
    let find_len = find_bytes.len();

    let mut changed = false;
    let mut i = 0;

    while i + find_len <= s.len() {
        if &s.as_bytes()[i..i + find_len] == find_bytes {
            out.push_str(replace);
            i += find_len;
            changed = true;
        } else {
            out.push(s.as_bytes()[i] as char);
            i += 1;
        }
    }

    out.push_str(&s[i..]); // append remaining tail
    (changed, out)
}


use std::fs::File;

/// Reads program into memory
pub fn read_program(path: &str) -> Vec<(String, String)> {
    let text = std::fs::read_to_string(path).unwrap();
    let mut program = Vec::new();

    for line in text.lines() {
        let mut parts = line.split(',');

        let a = parts.next().unwrap().to_string();
        let b = parts.next().unwrap_or("").to_string(); // nothing after the comma is a deletion

        program.push((a, b));
    }

    program
}

/// run the program until it halts
pub fn run_program(program: &Vec<(String, String)>, input: String, verbose: bool) -> String {
    let mut previous = input;
    loop {
        let output = run_step(program, previous.clone());
        if output == previous { /*println!("Finished with state: {}",output);*/ return output; }
        if verbose { println!("{}",output); }
        previous = output;
    }
}

/// perform a single step of the program
pub fn run_step(program: &Vec<(String, String)>, input: String) -> String {
    for (a, b) in program {
        let (changed, output) = replace(&input, &a, &b);
        if changed {
            return output;
        }
    }
    input
}


/// replaces all numbers in the input with the unary representation of the number in numeric_char
pub fn convert_numbers_unary(input: &str, numeric_char: char) -> String {
    let mut output = String::new();
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch.is_ascii_digit() {
            // Collect all consecutive digits
            let mut num_str = String::new();
            num_str.push(ch);

            while let Some(&next_ch) = chars.peek() {
                if next_ch.is_ascii_digit() {
                    num_str.push(next_ch);
                    chars.next();
                } else {
                    break;
                }
            }

            // Parse the number and convert to unary
            if let Ok(num) = num_str.parse::<usize>() {
                output.push_str(&numeric_char.to_string().repeat(num));
            }
        } else {
            output.push(ch);
        }
    }

    output
}

use std::fs::read_to_string;

/// read the input file into a string
pub fn read_input(path: &str) -> Vec<String> {
    read_to_string(path)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}


pub fn run_with_bumped_input(input: &Vec<String>, program: Vec<(String,String)>, start: String) -> String {
    let mut current = start;
    let total = input.len();

    for (index, i) in input.iter().enumerate() {
        current += i;
        current = run_program(&program, current.clone(), false);

        let progress = ((index + 1) as f64 / total as f64) * 100.0;
        println!("Progress: {:.1}%", progress);
    }
    current
}