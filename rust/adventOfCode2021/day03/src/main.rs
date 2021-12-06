use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::BinaryHeap;

/// find the place where vector splits into values heigher than 2^pivot and lower
/// returns the position where vector[position] >= 2_u32.pow(pivot);
fn get_pivot_position(pivot: u32, previous_pivots: &[u32], vector: &[u32]) -> usize {
    let mut pivot = 2_u32.pow(pivot);
    for i in previous_pivots {
        // the * is from compiler recommendations. but it makes sense
        pivot = pivot + 2_u32.pow(*i);
    }
    let mut position = vector.binary_search(&pivot)
    // https://stackoverflow.com/questions/36249693/whats-the-most-efficient-way-to-insert-an-element-into-a-sorted-vector
        .unwrap_or_else(|e| e);

    position = if position >= vector.len() { vector.len() - 1 } else { position };
    position = if vector[position] < pivot { position + 1 } else { position };
    // repeating this line is only necessary if the whole slice is below 2^pivot
    position = if position >= vector.len() { vector.len() - 1 } else { position };

    position
}

fn get_oxygen(mut pivot: u32, pivots: &mut Vec<u32>, entries: &[u32]) {
    if pivot == 0 { return } else { pivot = pivot - 1 };
    let position = get_pivot_position(pivot, pivots, entries);
    if position <= entries.len() / 2 {
        pivots.push(pivot);
        get_oxygen(pivot, pivots, &entries[position..]);
    } else {
        get_oxygen(pivot, pivots, &entries[..position]);
    }
}

// file read line by line copied from https://riptutorial.com/rust/example/4275/read-a-file-line-by-line
// string conversion from https://stackoverflow.com/questions/27043268/convert-a-string-to-int

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut digits = Vec::new();   // count the number of '1' for each digit
    let mut count_lines = 0;
    let mut entries: BinaryHeap<u32> = BinaryHeap::new();

    for line in reader.lines() {

        let line = line.unwrap();
        count_lines = count_lines + 1;
        digits.resize(line.len(), 0);
        // https://stackoverflow.com/questions/27606616/is-there-anything-in-rust-to-convert-a-binary-string-to-an-integer
        entries.push(u32::from_str_radix(&line, 2).unwrap());

        for (i, digit) in line.chars().enumerate() {
            if digit == '1' {
                digits[i] = digits[i] + 1;
            }
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

    let gamma = u32::from_str_radix(&gamma, 2).unwrap();
    let epsilon = u32::from_str_radix(&epsilon, 2).unwrap();

    // I would love to just invert gamma to get epsilon
    // but since there are a bunch of leading '0' bits it's just easier this way

    println!("gamma is: {} and epsilon is: {} for a power consumption of: {}", gamma, epsilon, gamma * epsilon);

    // part two

    let entries = entries.into_sorted_vec();

    let mut pivots_oxygen: Vec<u32> = Vec::from([]);
    let pivot: u32 = 12;  // the input has 12 bits
    get_oxygen(pivot, &mut pivots_oxygen, entries.as_slice());
    //let position = get_pivot_position(pivot, entries[..].to_vec());    // I don't think this is so great

    let mut pivot = 0;
    for i in pivots_oxygen {
        pivot = pivot + 2_u32.pow(i);
    }
    println!("The pivots for oxygen have been {}", format!("{:b}", pivot));
    //println!("The closest value to 2^{} is {} at position {}", pivot, entries[position], position);
    
}
