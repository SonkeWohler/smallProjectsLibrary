mod bingo;

use bingo::Board;
use bingo::build_board;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);

    let mut line = String::from("");
    reader.read_line(&mut line).unwrap(); // if this fails then the program can fail
    reader.read_line(&mut String::from("")).unwrap();
    let draw_line: Vec<&str> = line.split(',').collect();   // if this fails program might as well
    let mut draws: Vec<u32> = vec![];
    for draw in draw_line {
        draws.push(draw.trim().parse().unwrap());
    }

    let mut boards: Vec<Board> = vec![];
    let mut board_values: Vec<u32> = vec![];
    let mut board_width = 0;
    let mut last_line_was_empty = true;

    for line in reader.lines() {
        let line = line.unwrap();   // if this fails the program might as well

        last_line_was_empty = false;
        if line.is_empty() {
            let board = build_board(board_values, board_width);
            boards.push(board);
            board_values = vec![];
            last_line_was_empty = true;
        }

        let line = line.trim().split_whitespace();
        for (i, entry) in line.enumerate() {
            let entry = entry.trim().parse().unwrap();
            board_values.push(entry);   // again, might as well panic
            board_width = i + 1;    // no more efficient way to get length of Iterator???
        }
    }

    if !last_line_was_empty {
        let board = build_board(board_values, board_width);
        boards.push(board);
    }

    let mut boards_won = 0;
    for draw in draws {
        if boards_won >= boards.len() {
            break;
        }

        let mut board_i = 0;
        for board in &mut boards {

            if board.draw(draw) { 
                boards_won += 1;

                let mut score = board.score();
                score *= draw;

                println!("A board has won on draw {}", draw);
                println!("The board's score was {} and it was board index {}", score, board_i);
            }

            board_i += 1;

        }

    }

    println!(" -------------- " );
    println!("    ======== " );
    println!(" -------------- " );
    println!("This was the last board");
    println!("Note: board indices starting at 0");

}
