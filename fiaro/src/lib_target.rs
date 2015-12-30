#![feature(asm)]
#![feature(lang_items)]
#![feature(collections)]

#![no_std]

extern crate collections;

pub mod target;
pub mod blinky_rust;

use target::Target;

#[no_mangle]
pub extern fn rust_main() {
	let mut api = Target::new();
	blinky_rust::run(&mut api);
}