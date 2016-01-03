pub mod types;
pub mod game;

use self::game::Game;
use self::types::Player;
use self::types::CellState;

pub trait SystemApi {
	fn log(&self, msg: &str);
}

pub struct Fiaro<'a> {
    system: &'a SystemApi,
	game: Game<'a>,
}

impl <'a> Fiaro<'a> {
    pub fn new(system: &'a SystemApi) -> Fiaro {
        Fiaro {
            system: system,
            game: Game::new(system),
        }
    }
    
    pub fn on_button_pressed(&mut self, btn: usize) {
        self.system.log("[fiaro] Button {} pressed");
        
        match btn {
            0       => self.game.reset(),
            1 ... 8 => self.game.drop_disc(btn - 1),
            _       => {},
        }
    }
    
    pub fn get_cell_states_col(&self, col: usize) -> &[CellState] {
        self.game.get_cell_states_col(col)
    }
}
