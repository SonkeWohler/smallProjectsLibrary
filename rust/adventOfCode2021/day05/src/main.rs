use std::fs::File;
use std::io::{BufReader, BufRead};
use std::str::SplitWhitespace;

// I don't seem to understand enough about lifetimes just yet to do this with fewer lines of code,
// so I do it this way.
// What I mean is that in order to pass SplitWhitespace.next() you need to specify lifetimes,
// since you are passing a reference to the original string to another routing, so to make sure it
// doesn't go out of scope and causes a memory error Rust requires you to put lifetimes on the
// reference that somehow makes sure.
fn get_positions_from_input(mut input: SplitWhitespace) -> ((u32, u32), (u32, u32)) {
    let start = input.next().unwrap();
    let mut start = start.split(',');
    let start = (start.next(), start.next() );
    let start = (start.0.unwrap(), start.1.unwrap());
    let start: (u32, u32) = (start.0.parse().unwrap(), start.1.parse().unwrap());

    // skip this part
    input.next();
    
    let end = input.next().unwrap();
    let mut end = end.split(',');
    let end = (end.next(), end.next() );
    let end = (end.0.unwrap(), end.1.unwrap());
    let end: (u32, u32) = (end.0.parse().unwrap(), end.1.parse().unwrap());

    (start, end)

}

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();   // if this fails the program might as well
        let line = line.trim().split_whitespace();

        let positions = get_positions_from_input(line);

        println!("{:?} -> {:?}", positions.0, positions.1);

    }



}
