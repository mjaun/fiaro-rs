#![feature(convert)]

mod sim;
mod fiaro;

extern crate sdl2;

use sim::Simulation;
use fiaro::Fiaro;

fn main() {
	let simulation = Simulation::new();
	let mut fiaro = Fiaro::new(&simulation);
	
	simulation.run(&mut fiaro);
}