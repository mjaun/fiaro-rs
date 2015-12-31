extern crate collections;

pub mod runtime_support;

use fiaro::BoardApi;
use collections::Vec;

pub struct Target;

impl Target {
	pub fn new() -> Target {
		Target
	}
}

extern {
	fn trace_puts(s: *const u8) -> i32;
}

impl BoardApi for Target {
	fn log(&self, msg: &str) {
		let mut cstr: Vec<u8> = Vec::with_capacity(msg.len() + 1);
		cstr.extend_from_slice(msg.as_bytes());
		cstr.push(0);
		
		unsafe {
			trace_puts(cstr.as_ptr());
		}
	}
}
