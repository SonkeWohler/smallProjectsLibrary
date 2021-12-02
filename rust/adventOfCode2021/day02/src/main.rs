use std::fs::File;
use std::io::{BufReader, BufRead};

// file read line by line copied from https://riptutorial.com/rust/example/4275/read-a-file-line-by-line
// string conversion from https://stackoverflow.com/questions/27043268/convert-a-string-to-int
// Deque docs at https://doc.rust-lang.org/std/collections/struct.VecDeque.html#method.pop_front

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;

    for line in reader.lines() {

        // --- for first exercise

        let line = line.unwrap();
        let vec: Vec<&str> = line.split(' ').collect();
        let magnitude: i32 = vec[1].parse().unwrap();

        // match string against the three
        if vec[0].starts_with('f') {
            horizontal = horizontal + magnitude;
        } else if vec[0].starts_with('d') {
            depth = depth + magnitude;
        } else {
            depth = depth - magnitude;
        }

        println!("{} --- {}", vec[0], magnitude);
        println!("{} X {}", horizontal, depth);

    }

    println!(" =========================== ");
    println!("   ----------------------- ");
    println!(" =========================== ");

    println!(" The final position is {} X {}, for a hash of {}", horizontal, depth, horizontal * depth);
}
