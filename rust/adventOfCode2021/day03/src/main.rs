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

fn get_oxy_co2(mut pivot: u32, pivots: &mut Vec<u32>, entries: &[u32], oxy_false_co2_true: bool) -> u32 {
    if entries.len() == 1 { return entries[0]; };
    pivot = pivot - 1;
    let position = get_pivot_position(pivot, pivots, entries);
    let more_1s = position <= entries.len() / 2;
    if more_1s ^ oxy_false_co2_true {
        pivots.push(pivot);
        return get_oxy_co2(pivot, pivots, &entries[position..], oxy_false_co2_true);
    } else {
        return get_oxy_co2(pivot, pivots, &entries[..position], oxy_false_co2_true);
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
    // https://turreta.com/2019/09/07/rust-how-to-check-for-null-values/
    let mut line_length: Option<usize> = None;

    for line in reader.lines() {

        let line = line.unwrap();
        count_lines = count_lines + 1;
        digits.resize(line.len(), 0);
        // https://stackoverflow.com/questions/27606616/is-there-anything-in-rust-to-convert-a-binary-string-to-an-integer
        entries.push(u32::from_str_radix(&line, 2).unwrap());
        match line_length {
            None => line_length = Some(line.len()),
            Some(length) => if length != line.len() { panic!("line lengths are inconsistent within the file"); },
        }

        for (i, digit) in line.chars().enumerate() {
            if digit == '1' {
                digits[i] = digits[i] + 1;
            }
        }

    }

    let line_length = line_length.unwrap();

    println!("There was a total of {} lines of length {}", count_lines, line_length);

    let mut gamma: u32 = 0;

    for digit in digits {
        gamma <<= 1;
        if digit >= count_lines/2 {
            gamma = gamma + 1;
        }
    }

    let mask = 2_u32.pow(line_length as u32) - 1;
    let epsilon = gamma ^ mask;

    println!("gamma is: {} and epsilon is: {} for a power consumption of: {}", gamma, epsilon, gamma * epsilon);

    // part two

    let entries = entries.into_sorted_vec();

    let mut pivots: Vec<u32> = Vec::from([]);
    let pivot: u32 = line_length as u32;
    let oxy = get_oxy_co2(pivot, &mut pivots, entries.as_slice(), false);

    let mut pivot = 0;
    for i in pivots {
        pivot = pivot + 2_u32.pow(i);
    }
    // format from https://www.programming-idioms.org/idiom/76/binary-digits-from-an-integer/629/rust
    println!("The pivots for oxygen have been {} which leads us to the value {}", format!("{:b}", pivot), oxy);
    
    let mut pivots: Vec<u32> = Vec::from([]);
    let pivot: u32 = line_length as u32;
    let co2 = get_oxy_co2(pivot, &mut pivots, entries.as_slice(), true);

    let mut pivot = 0;
    for i in pivots {
        pivot = pivot + 2_u32.pow(i);
    }
    println!("The pivots for CO2 have been {} which leads us to the value {}", format!("{:b}", pivot), co2);

    println!("This makes ofr a life support rating of {}", co2 * oxy);
}
