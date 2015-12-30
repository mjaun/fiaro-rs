mod blinky_rust;
mod simulation;

use simulation::Simulation;

fn main() {
	let mut api = Simulation::new();
	blinky_rust::run(&mut api);
}