use std::fs::File;
use std::io::{BufReader, BufRead};
use std::str::SplitWhitespace;
use std::fmt;
use std::collections::HashMap;

#[derive(Debug)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Hash)]
struct Position {
    x: u32,
    y: u32,
}
// https://doc.rust-lang.org/std/fmt/trait.Display.html
impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
    
}

#[derive(Debug)]
struct VentLine {
    start: Position,
    end: Position,
}
impl fmt::Display for VentLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, -> {}", self.start, self.end)
    }
}

fn get_valid_vent_line(input: VentLine) -> Option<VentLine> {
    if input.start.x != input.end.x && input.start.y != input.end.y {
        return None;
    }

    let start_hash = input.start.x * input.start.y;
    let end_hash = input.end.x * input.end.y;
    if start_hash > end_hash {
        let input =  VentLine {
            start: input.end,
            end: input.start,
        };
        return Some(input);
    }

    Some(input)

}

/// this one will keep track of the number of vent lines on each position
struct Map {
    map: HashMap<Position, u32>,
}

fn construct_map() -> Map {
    Map {
        map: HashMap::new(),
    }
}

impl Map {
    pub fn read_line(&mut self, line: VentLine) {
        
    }

    fn read_position(&mut self, position: Position) {
        match self.map.get(&position) {
            None => self.map.insert(position, 1),
            Some(value) => self.map.insert(position, value + 1),
        };
    }
}

// I don't seem to understand enough about lifetimes just yet to do this with fewer lines of code,
// so I do it this way.
// What I mean is that in order to pass SplitWhitespace.next() you need to specify lifetimes,
// since you are passing a reference to the original string to another routing, so to make sure it
// doesn't go out of scope and causes a memory error Rust requires you to put lifetimes on the
// reference that somehow makes sure.
fn get_positions_from_input(mut input: SplitWhitespace) -> VentLine {
    let start = input.next().unwrap();
    let mut start = start.split(',');
    let start = (start.next(), start.next() );
    let start = (start.0.unwrap(), start.1.unwrap());
    let start = Position {
        x: start.0.parse().unwrap(),
        y: start.1.parse().unwrap(),
    };

    // skip this part
    input.next();
    
    let end = input.next().unwrap();
    let mut end = end.split(',');
    let end = (end.next(), end.next() );
    let end = (end.0.unwrap(), end.1.unwrap());
    let end = Position {
        x: end.0.parse().unwrap(),
        y: end.1.parse().unwrap(),
    };

    VentLine {
        start,
        end,
    }

}

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut map = construct_map();
    let mut positions: Vec<VentLine> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();   // if this fails the program might as well
        let line = line.trim().split_whitespace();

        let line = get_positions_from_input(line);
        match get_valid_vent_line(line) {
            None => continue,
            Some(line) => positions.push(line),
        }

    }

    for position in &positions {
        println!("{}", position);
    }

}
