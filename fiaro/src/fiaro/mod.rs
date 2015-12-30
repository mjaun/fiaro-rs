pub trait BoardApi {
	fn log(&self, msg: &str);
}

pub fn run(api: &mut BoardApi) {
	api.log("Hello from Rust!");
	loop {}
}
