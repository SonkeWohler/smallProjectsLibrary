use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);

    // for reading in the first line (or few lines) before looping over the rest
    // delete this paragraph if not needed, and reader won't have to be mutable
    let mut line = String::from("");
    reader.read_line(&mut line).unwrap();
    line = String::from(line.trim());
    for element in line.split_whitespace() {
        println!("the first line has an element '{}'", element);
    }

    let result = String::from("Hello World!");

    for line in reader.lines() {
        let line = line.unwrap();
        // change this based on your needs
        let line: f32 = line.trim().parse().unwrap();
        println!("{}", line);
    }

    println!(" ----------------------");
    println!("    =============");
    println!("    =============");
    println!("    =============");
    println!(" ----------------------");

    println!("The result is {}", result);
}
