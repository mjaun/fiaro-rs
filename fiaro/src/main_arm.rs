#![feature(asm)]
#![feature(lang_items)]
#![feature(collections)]

#![no_std]

extern crate collections;

pub mod arm;
pub mod fiaro;

use arm::Target;
use fiaro::Fiaro;

#[no_mangle]
pub extern fn rust_main() {
	let target = Target::new();
	let mut fiaro = Fiaro::new(&target);
	
	target.run(&mut fiaro);
}