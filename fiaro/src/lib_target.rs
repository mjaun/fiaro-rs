#![feature(asm)]
#![feature(lang_items)]
#![feature(collections)]

#![no_std]

extern crate collections;

pub mod target;
pub mod fiaro;

use target::Target;

#[no_mangle]
pub extern fn rust_main() {
	let mut api = Target::new();
	fiaro::run(&mut api);
}