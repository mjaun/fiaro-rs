pub const BOARD_COLS:  usize = 7;
pub const BOARD_ROWS: usize = 7;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Player {
    One,
    Two,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum GameState {
    Running,
    Finished(Player), // Winner
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CellState {
    Free,
    Occupied(Player), // Occupant
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct CellIndex {
    pub col: isize,
    pub row: isize,
}

impl CellIndex {
    pub fn from_usize(col: usize, row: usize) -> CellIndex {
        CellIndex {
            col: col as isize,
            row: row as isize,
        }
    }
    
    pub fn is_valid(&self) -> bool {
        self.col >= 0 &&
        self.row >= 0 &&
        self.col < BOARD_COLS as isize &&
        self.row < BOARD_ROWS as isize
    }
    
    pub fn add(&mut self, dcol: isize, drow: isize) {
        self.col += dcol;
        self.row += drow;
    }
}