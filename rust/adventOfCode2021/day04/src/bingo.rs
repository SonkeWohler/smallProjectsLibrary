use std::collections::HashMap;

#[derive(Debug)]
struct Entry {
    value: u32,
    location: (usize, usize),
    drawn: bool,
}

impl Entry {
    fn score(&self) -> u32 {
        if self.drawn { 0 } else { self.value }
    }
}

/// represents a row or column of a board in binary (for efficiency)
/// keeps track of whether the values have been drawn
/// keep in mind when debugging that they may not be oriented the way you expect them to be.  This
/// is irrelevant as long as they are consistent, since they only have to detect whether all
/// numbers in a row/column have been drawn, not which row/column that is.
/// keep in mind that the binary number might be oriented such that it represents the draws right
/// to left and upside down
#[derive(Debug)]
struct BoardVector {
    value: u32,
    win_value: u32,
}

impl BoardVector {
    fn draw_value(&mut self, index: usize) -> bool {
        let mut helper = 1;
        helper <<= index;
        self.value |= helper;
        self.win_value == self.value
    }
}

fn build_board_vector(win_value: u32) -> BoardVector {
    BoardVector {
        value: 0,
        win_value,
    }
}

/// keep track of one board
#[derive(Debug)]
pub struct Board {
    entries: HashMap<u32,Vec<Entry>>,
    rows: Vec<BoardVector>,
    cols: Vec<BoardVector>,

    // some hash based way to reference the locations of drawable values
    // must be able to point to multiple locations
}

impl Board {
    pub fn draw(&mut self, draw: u32) -> bool{
        let locations = self.draw_on_entry(draw);
        for location in locations {
            if self.draw_on_vectors(location) { return true };
        }
        false
    }

    pub fn score(&self) -> u32 {
        let mut score = 0;
        for entry_vector in self.entries.values() {
            for entry in entry_vector {
                score += entry.score();
            }
        }
        score
    }

    /// finds the entries whose value matches draw, marks them as such and returns the locations of
    /// them
    fn draw_on_entry(&mut self, draw: u32) -> Vec<(usize, usize)> {
        let entries;
        match self.entries.get_mut(&draw) {
            None => return vec![],
            Some(result) => entries = result,
        }
        let mut locations: Vec<(usize, usize)> = Vec::new();
        // https://stackoverflow.com/questions/39622783/how-can-i-do-a-mutable-borrow-in-a-for-loop
        for entry in entries.iter_mut() {
            entry.drawn = true;
            locations.push(entry.location);
        }
        locations
    }

    fn draw_on_vectors(&mut self, location: (usize, usize)) -> bool {
        if self.rows[location.0].draw_value(location.1) { return true };

        if self.cols[location.1].draw_value(location.0) { return true };

        false
    }

}

/// values are the numbers on the board from left to right, top to bottom.
/// this function uses number_of_columns to arrange them because it is easier
/// if this was an API I would at least provide a wrapper that takes values as a matrix.
pub fn build_board(values: Vec<u32>, number_of_columns: usize) -> Board {
    let mut entries = HashMap::new();
    let mut i: usize = 0;
    let mut j: usize = 0;
    for value in values {
        let entry = Entry{
            value,
            location: (i,j),
            drawn: false,
        };

        let existing_entry_vec = entries.get_mut(&value);
        match existing_entry_vec {
            None => {
                let vec = vec![entry];
                entries.insert(value, vec);
            }
            // this commented line doesn't work because it is an expression that gives the match a
            // value and in this case the types don't match
            //None => entries.insert(value, vec![entry]),
            Some(vec) => vec.push(entry),
        }
        
        i += 1;
        if i >= number_of_columns {
            i = 0;
            j += 1;
        }
    }

    // http://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/casting-between-types.html
    let win_row_number = 2_u32.pow(number_of_columns as u32) - 1;
    let win_col_number = 2_u32.pow(j as u32) - 1;
    let mut rows: Vec<BoardVector> = vec![];
    let mut cols: Vec<BoardVector> = vec![];
    for _i in 0..number_of_columns {
        rows.push(build_board_vector(win_row_number));
    }
    for _i in 0..j {
        cols.push(build_board_vector(win_col_number));
    }

    Board {
        entries,
        rows,
        cols,
    }
}
