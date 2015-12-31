#![feature(asm)]
#![feature(lang_items)]
#![feature(collections)]

#![no_std]

extern crate collections;

pub mod arm;
pub mod fiaro;

use arm::Target;

#[no_mangle]
pub extern fn rust_main() {
	let mut api = Target::new();
	fiaro::run(&mut api);
}