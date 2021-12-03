use std::fs::File;
use std::io::{BufReader, BufRead};

// file read line by line copied from https://riptutorial.com/rust/example/4275/read-a-file-line-by-line
// string conversion from https://stackoverflow.com/questions/27043268/convert-a-string-to-int
// Deque docs at https://doc.rust-lang.org/std/collections/struct.VecDeque.html#method.pop_front

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut digits = Vec::new();   // count the number of '1' for each digit
    let mut count_lines = 0;

    for line in reader.lines() {

        let line = line.unwrap();
        count_lines = count_lines + 1;
        digits.resize(line.len(), 0);

        let mut i = 0;
        for digit in line.chars() {
            if digit == '1' {
                digits[i] = digits[i] + 1;
            }
            i = i + 1;
        }

    }

    println!("There was a total of {} lines", count_lines);

    let mut gamma = String::from("");
    let mut epsilon = String::from("");

    for digit in digits {
        if digit >= count_lines/2 {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }

    let gamma = isize::from_str_radix(&gamma, 2).unwrap();
    let epsilon = isize::from_str_radix(&epsilon, 2).unwrap();

    // I would love to just invert gamma to get epsilon
    // but since there are a bunch of leading '0' bits it's just easier this way

    println!("gamma is: {} and epsilon is: {} for a power consumption of: {}", gamma, epsilon, gamma * epsilon);
}
