extern crate sdl2;

use fiaro::SystemApi;
use fiaro::Fiaro;
use fiaro::types::*;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Renderer;
use sdl2::rect::Rect;

pub const SCREEN_WIDTH:  u32 = 480;
pub const SCREEN_HEIGHT: u32 = 480;
pub const SPACING:       u32 = 5;

pub const COLOR_RED: Color = Color::RGB(255, 0, 0);
pub const COLOR_YELLOW: Color = Color::RGB(255, 255, 0);
pub const COLOR_BLACK: Color = Color::RGB(0, 0, 0);
pub const COLOR_GRAY: Color = Color::RGB(128, 128, 128);

pub struct Simulation;

impl Simulation {
    pub fn new() -> Simulation {
        Simulation
    }
    
    pub fn run(&self, fiaro: &mut Fiaro) {
        println!("[simulation] Initializing");
        
        let sdl = sdl2::init().unwrap();
        let video = sdl.video().unwrap();
        let mut event_pump = sdl.event_pump().unwrap();
        
        let window = video.window("Fiaro", SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .opengl()
            .build()
            .unwrap();
        
        let mut renderer = window.renderer().build().unwrap();
        self.redraw(fiaro, &mut renderer);
        
        println!("[simulation] Starting main loop");
        
        loop {
            match event_pump.wait_event() {
                Event::KeyDown { keycode: Some(key), .. } => {
                    self.handle_key_press(fiaro, key);
					self.redraw(fiaro, &mut renderer);
                },
                
                Event::Quit { .. } => {
                    break
                },
                
                _ => {},
            }
        }
        
        println!("[simulation] Main loop terminated");
    }
    
    fn handle_key_press(&self, fiaro: &mut Fiaro, key: Keycode) {
        match key {
            Keycode::Num0 => fiaro.on_button_pressed(0),
            Keycode::Num1 => fiaro.on_button_pressed(1),
            Keycode::Num2 => fiaro.on_button_pressed(2),
            Keycode::Num3 => fiaro.on_button_pressed(3),
            Keycode::Num4 => fiaro.on_button_pressed(4),
            Keycode::Num5 => fiaro.on_button_pressed(5),
            Keycode::Num6 => fiaro.on_button_pressed(6),
            Keycode::Num7 => fiaro.on_button_pressed(7),
            _ => {},
        }
    }
    
    fn redraw(&self, fiaro: &mut Fiaro, renderer: &mut Renderer) {
        renderer.set_draw_color(COLOR_BLACK);
        renderer.clear();
        
        for col in 0..BOARD_COLS {
			let cell_states = fiaro.get_cell_states_col(col);
			
			for row in 0..BOARD_ROWS {
			    let color = self.cell_state_to_color(cell_states[row]);
			    let rect = self.get_disc_rect(col, row);
			    
			    renderer.set_draw_color(color);
			    renderer.fill_rect(rect);
			}
        }
        
        renderer.present();
    }
    
    fn cell_state_to_color(&self, state: CellState) -> Color {
        match state {
	        CellState::Occupied(Player::One) => COLOR_RED,
	        CellState::Occupied(Player::Two) => COLOR_YELLOW,
	        CellState::Free => COLOR_GRAY,
	    }
    }
    
    fn get_disc_rect(&self, col: usize, row: usize) -> Rect {
        const DISC_WIDTH: u32 = SCREEN_WIDTH / BOARD_COLS as u32;
        const DISC_HEIGHT: u32 = SCREEN_HEIGHT / BOARD_ROWS as u32;
        
	    let x = (col as i32) * (DISC_WIDTH as i32);
	    let mut y = (row as i32) * (DISC_HEIGHT as i32);
	    
	    y = (SCREEN_HEIGHT as i32) - (DISC_HEIGHT as i32) - y;
        
        Rect::new(x, y, DISC_WIDTH - SPACING, DISC_HEIGHT - SPACING)
    		.unwrap().unwrap()
    }
}

impl SystemApi for Simulation {
    fn log(&self, msg: &str) {
        println!("{}", msg);
    }
}
