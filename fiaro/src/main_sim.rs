extern crate sdl2;

mod sim;
mod fiaro;

use sim::Simulation;

fn main() {
	let ctx = sdl2::init().unwrap();
	let mut api = Simulation::new(&ctx);
	fiaro::run(&mut api);
}