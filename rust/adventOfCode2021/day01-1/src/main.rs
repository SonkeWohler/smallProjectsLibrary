use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::VecDeque;

// file read line by line copied from https://riptutorial.com/rust/example/4275/read-a-file-line-by-line
// string conversion from https://stackoverflow.com/questions/27043268/convert-a-string-to-int
// Deque docs at https://doc.rust-lang.org/std/collections/struct.VecDeque.html#method.pop_front

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut last_depth: i32 = 0;
    let mut rises_single: i32 = 0;    // number of times depth increases
    let mut rises_triple: i32 = 0;    // rises for three measurement psuedo-averages
    let mut dec: VecDeque<i32> = VecDeque::new();
    //let mut dec:VecDeque<i32> = VecDeque::with_capacity(7);

    for line in reader.lines() {

        let line = line.unwrap();
        let depth: i32 = line.parse().unwrap();

        // --- for first exercise

        let diff: i32 = depth - last_depth;
        last_depth = depth;

        let diff_symbol;    // easy display of increase, decrease etc
        if diff > 0 {
            diff_symbol = '^';
            rises_single = rises_single + 1;
        } else if diff == 0 {
            diff_symbol = '=';
        } else {
            diff_symbol = 'v';
        }

        println!("{} --- {}", line, diff_symbol);

        // --- for second exercise

        if dec.len() >= 4 {
            dec.pop_front();
        }

        dec.push_back(depth);

        if dec.len() < 4 {
            continue;
        }

        let mut old_depths = 0;
        for depth in dec.range(..3) {
            old_depths = old_depths + depth;
        }
        let mut new_depths = 0;
        for depth in dec.range(1..) {
            new_depths = new_depths + depth;
        }
        let diff: i32 = new_depths - old_depths;

        let diff_symbol;
        if diff > 0 {
            diff_symbol = '^';
            rises_triple = rises_triple + 1;
        } else if diff == 0 {
            diff_symbol = '=';
        } else {
            diff_symbol = 'v';
        }

        println!("{} : {}", diff, diff_symbol);

    }

    rises_single = rises_single - 1;    // "there is no measurement before the first measurement"
    println!("There have been {} rises in single measurement.", rises_single);

    println!("And {} rises in running average like measurements", rises_triple);
}
