extern crate sdl2;

use sdl2::Sdl;
use fiaro::BoardApi;

pub struct Simulation;

impl Simulation {
	pub fn new(ctx: &Sdl) -> Simulation {
		let video_ctx = ctx.video().unwrap();
		let mut timer = ctx.timer().unwrap();
		
		let mut window = match video_ctx.window("Fiaro", 640, 480)
			.position_centered()
			.opengl()
			.build() {
			Ok(window) => window,
			Err(err)   => panic!("failed to create window: {}", err)		
		};
		
		window.show();
		timer.delay(3000);
		
		Simulation
	}
}

impl BoardApi for Simulation {
	fn log(&self, msg: &str) {
		println!("{}", msg);
	}
}
