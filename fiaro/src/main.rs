mod fiaro;
mod simulation;

use simulation::Simulation;

fn main() {
	let mut api = Simulation::new();
	fiaro::run(&mut api);
}