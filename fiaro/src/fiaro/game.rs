use super::SystemApi;
use super::types::*;

pub struct Game<'a> {
    system: &'a SystemApi,
    board: [[CellState; BOARD_ROWS]; BOARD_COLS], // board[col][row]
    current_player: Player,
    game_state: GameState,
}

impl <'a> Game<'a> {
    pub fn new(system: &'a SystemApi) -> Game {
        Game {
            system: system,
            board: [[CellState::Free; BOARD_ROWS]; BOARD_COLS],
            current_player: Player::One,
            game_state: GameState::Running,
        }
    }
    
    pub fn reset(&mut self) {
        self.system.log("[game] Reset");
        
        self.board = [[CellState::Free; BOARD_ROWS]; BOARD_COLS];
        self.current_player = Player::One;
        self.game_state = GameState::Running;
    }

    pub fn drop_disc(&mut self, col: usize) {
        let index = self.get_drop_index(col);
        
        if let Some(index) = index {
        	let state = CellState::Occupied(self.current_player);
            self.set_cell(&index, state);
            self.update_finished_state(&index);
            self.switch_player();
            
            self.system.log("[game] Disc dropped in column {}");
        } else {
			self.system.log("[game] Dropping in column {} not possible");
        }
    }

    pub fn get_game_state(&self) -> GameState {
        return self.game_state;
    }
    
    pub fn get_current_player(&self) -> Option<Player> {
        match self.game_state {
            GameState::Running => Some(self.current_player),
            GameState::Finished(w) => None,
        }
    }
    
    pub fn get_cell_states_col(&self, col: usize) -> &[CellState] {
        &self.board[col]
    }
    
    fn switch_player(&mut self) {
        self.current_player = match self.current_player {
            Player::One => Player::Two,
            Player::Two => Player::One,
        }
    }
    
    fn update_finished_state(&mut self, modified_pos: &CellIndex) {
        if self.check_finished_pos(modified_pos) {
            let player = match *self.get_cell(modified_pos) {
                CellState::Occupied(player) => player,
                CellState::Free => panic!("no disc on position!"),
            };
            
			self.game_state = GameState::Finished(player);            
        }
    }
    
    fn check_finished_pos(&self, pos: &CellIndex) -> bool {
        // disc at pos is counted twice
        self.count_line(pos, 1,  0) + self.count_line(pos, -1,  0) > 4 ||
        self.count_line(pos, 0,  1) + self.count_line(pos,  0, -1) > 4 ||
        self.count_line(pos, 1,  1) + self.count_line(pos, -1, -1) > 4 ||
        self.count_line(pos, 1, -1) + self.count_line(pos, -1,  1) > 4
    }
    
    fn count_line(&self, pos: &CellIndex, dcol: isize, drow: isize) -> usize {
		let state = *self.get_cell(pos);
		let mut curr_pos = *pos;
		let mut count = 0;
		
		while curr_pos.is_valid() && *self.get_cell(pos) == state {
		    count += 1;
		    curr_pos.add(dcol, drow);
		}
		
		count
    }
    
    fn get_drop_index(&self, col: usize) -> Option<CellIndex> {
        let row = self.board[col].iter()
        	.position(|x| *x == CellState::Free);
        
        match row {
            Some(row) => Some(CellIndex::from_usize(col, row)),
            None => None,
        }
    }

    fn get_cell(&self, pos: &CellIndex) -> &CellState {
        &self.board[pos.col as usize][pos.row as usize]
    }
    
    fn set_cell(&mut self, pos: &CellIndex, state: CellState) {
        self.board[pos.col as usize][pos.row as usize] = state;
    }
}