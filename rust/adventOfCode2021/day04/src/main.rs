
/// represents a row or column of a board
/// keeps track of whether the values have been drawn
// should implement the copy trait
#[derive(Debug)]
struct BoardVector {
    value: u32,
}

impl BoardVector {
    fn draw_value(&mut self, index: usize) {
        let mut helper = 1;
        helper <<= index;
        self.value |= helper;
    }
}


/// keep track of one board
#[derive(Debug)]
struct Board {
    row: Vec<BoardVector>,
    col: Vec<BoardVector>,
    length: u32,    // the length of each row/column
    win_row_number: u32,    // 2_u32.pow(length), this is when the row wins
    win_col_number: u32,    // ditto

    // some hash based way to reference the locations of drawable values
    // must be able to point to multiple locations

    // some hash way to reference back from location to values
}

impl Board {
    fn draw(&mut self, draw: u32) -> bool{
        let location = self.get_location(draw);

        self.row[location.0].draw_value(location.1);
        if self.row[location.0].value == self.win_row_number { return true };

        self.col[location.1].draw_value(location.0);
        if self.col[location.1].value == self.win_col_number { return true };

        false
    }

    fn get_location(&self, _draw: u32) -> (usize, usize) {
        // use hashing to find position
        let row: usize = 0;
        let col: usize = 0;
        (row, col)
    }
}

fn build_board_vector() -> BoardVector {
    BoardVector{
        value: 0,
    }
}

fn build_board(values: Vec<Vec<u32>>, length: u32) -> Board {
    let mut row: Vec<BoardVector> = Vec::new();
    let mut count_values = 0;   // to avoid converting usize to u32
    for i in values {
        row.push(build_board_vector());
        count_values += 1;
    }
    let mut col: Vec<BoardVector> = Vec::new();
    for _i in 0..length {
        col.push(build_board_vector());
    }

    Board {
        row: row,
        col: col,
        length,
        win_row_number: 2_u32.pow(length) - 1,
        win_col_number: 2_u32.pow(count_values) - 1,
    }
}

fn main() {
    println!("compiled!!!");

    let mut values: Vec<u32> = Vec::new();
    let length = 10;
    for i in 0..length {
        values.push(i);
    }
    let mut greater_values: Vec<Vec<u32>> = Vec::new();
    greater_values.push(values.clone());
    greater_values.push(values.clone());
    greater_values.push(values.clone());
    let mut board = build_board(greater_values, length);
    board.draw(42);
    dbg!(board);
}
