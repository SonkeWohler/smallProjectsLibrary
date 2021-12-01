use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    // file read line by line copied from https://riptutorial.com/rust/example/4275/read-a-file-line-by-line
    // string conversion from https://stackoverflow.com/questions/27043268/convert-a-string-to-int

    let filename = "1.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut last_depth: i32 = 0;
    let mut rises: i32 = 0;    // number of times depth increases

    for line in reader.lines() {
        let line = line.unwrap();
        let depth: i32 = line.parse().unwrap();

        let diff: i32 = depth - last_depth;
        last_depth = depth;

        let diff_symbol;    // easy display of increase, decrease etc
        if diff > 0 {
            diff_symbol = '^';
            rises = rises + 1;
        } else if diff == 0 {
            diff_symbol = '=';
        } else {
            diff_symbol = 'v';
        }

        println!("{} --- {}", line, diff_symbol);
    }

    rises = rises - 1;    // "there is no measurement before the first measurement"
    println!("There have been {} rises.", rises);
}
