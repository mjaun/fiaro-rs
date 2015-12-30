use blinky_rust::BoardApi;

pub struct Simulation;

impl Simulation {
	pub fn new() -> Simulation {
		Simulation
	}
}

impl BoardApi for Simulation {
	fn log(&self, msg: &str) {
		println!("{}", msg);
	}
}
