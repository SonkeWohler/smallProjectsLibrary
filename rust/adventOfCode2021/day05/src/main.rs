use std::fs::File;
use std::io::{BufReader, BufRead};
use std::str::SplitWhitespace;
use std::fmt;
use std::collections::HashMap;

#[derive(Debug)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Hash)]
#[derive(Copy)]
#[derive(Clone)]
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
enum VentLineDirection {
    Horizontal,
    Vertical,
    Diagonal,
    Undefined,
}

impl fmt::Display for VentLineDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display_char = match self {
            VentLineDirection::Undefined => '.',
            VentLineDirection::Horizontal => '-',
            VentLineDirection::Vertical => '|',
            VentLineDirection::Diagonal => '/',
        };
        write!(f, "{}", display_char)
    }
}

#[derive(Debug)]
struct VentLine {
    start: Position,
    end: Position,
    direction: VentLineDirection,
}

impl fmt::Display for VentLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, -> {} : {}", self.start, self.end, self.direction)
    }
}

fn get_valid_vent_line(input: VentLine) -> VentLine {
    let mut _direction = VentLineDirection::Undefined;
    if input.start.x == input.end.x {
        _direction = VentLineDirection::Vertical;
    } else if input.start.y == input.end.y {
        _direction = VentLineDirection::Horizontal;
    } else {
        _direction = VentLineDirection::Diagonal;
    }

    let start_hash = input.start.x * input.start.y;
    let end_hash = input.end.x * input.end.y;
    if start_hash > end_hash {
        return VentLine {
            start: input.end,
            end: input.start,
            direction: _direction,
        };
    }

    VentLine{
        start: input.start,
        end: input.end,
        direction: _direction,
    }

}

/// this one will keep track of the number of vent lines on each position
#[derive(Debug)]
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
        let mut position = line.start;
        loop {
            self.read_position(position);
            match &line.direction {
                VentLineDirection::Horizontal => position.x += 1,
                VentLineDirection::Vertical => position.y += 1,
                _ => panic!("VentLine.direction has a value it shouldn't have right now!"),
            };
            if position.x > line.end.x || position.y > line.end.y {
                break;
            }
        }
    }

    fn read_position(&mut self, position: Position) {
        let value = match self.map.get(&position) {
            None => 1,
            Some(value) => value + 1,
        };
        self.map.insert(position, value);
    }
}

// I don't seem to understand enough about lifetimes just yet to do this with fewer lines of code,
// so I do it this way.
// What I mean is that in order to pass SplitWhitespace.next() you need to specify lifetimes,
// since you are passing a reference to the original string to another routing, so to make sure it
// doesn't go out of scope and causes a memory error Rust requires you to put lifetimes on the
// reference that somehow makes sure.
fn get_vent_line_from_input(mut input: SplitWhitespace) -> VentLine {
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
        direction: VentLineDirection::Undefined,
    }

}

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut map = construct_map();

    for line in reader.lines() {
        let line = line.unwrap();   // if this fails the program might as well
        let line = line.trim().split_whitespace();

        let line = get_vent_line_from_input(line);
        let line = get_valid_vent_line(line);
        match line.direction {
            VentLineDirection::Diagonal => continue,
            VentLineDirection::Undefined => panic!("invalid vent line direction after it was supposed to have been validated"),
            _ => map.read_line(line),
        }

    }

    let mut intersections = 0;
    for vent_depth in map.map.values() {
        if vent_depth > &1 {
            intersections += 1;
        }
    }

    println!();
    println!("There are {} intersections of at least two vent lines", intersections);
}
