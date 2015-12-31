rustc -C opt-level=3 -Z no-landing-pads --target thumbv7m-none-eabi src/libcore/lib.rs --out-dir libs_arm --emit obj,link
rustc -C opt-level=3 -Z no-landing-pads --target thumbv7m-none-eabi src/liballoc/lib.rs --out-dir libs_arm -L libs_arm --emit obj,link --cfg feature=\"external_funcs\" 
rustc -C opt-level=3 -Z no-landing-pads --target thumbv7m-none-eabi src/librustc_unicode/lib.rs --out-dir libs_arm -L libs_arm --emit obj,link
rustc -C opt-level=3 -Z no-landing-pads --target thumbv7m-none-eabi src/libcollections/lib.rs --out-dir libs_arm -L libs_arm --emit obj,link
pause