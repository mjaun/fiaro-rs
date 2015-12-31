extern crate core;

extern {
	fn malloc(size: u32) -> *mut u8;
	fn realloc(p: *mut u8, size: u32) -> *mut u8;
	fn free(p: *mut u8);	
}

#[no_mangle]
pub unsafe extern fn __rust_allocate(size: usize, _align: usize) -> *mut u8 {
	malloc(size as u32)
}

#[no_mangle]
pub unsafe extern fn __rust_reallocate(ptr: *mut u8, _old_size: usize, size: usize, _align: usize) -> *mut u8 { 
	realloc(ptr, size as u32)
}

#[no_mangle]
pub unsafe extern fn __rust_deallocate(ptr: *mut u8, _old_size: usize, _align: usize) {
	free(ptr);
}

#[lang = "panic_fmt"]
fn panic_fmt(_args: &core::fmt::Arguments, _file: &str, _line: u32) -> ! {
	loop {}	
}