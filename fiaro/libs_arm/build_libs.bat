rustc -C opt-level=2 -Z no-landing-pads --target thumbv7m-none-eabi -g src/libcore/lib.rs --out-dir libs_arm --emit obj,link
rustc -C opt-level=2 -Z no-landing-pads --target thumbv7m-none-eabi -g src/liballoc/lib.rs --out-dir libs_arm -L libs-arm --emit obj,link --cfg feature=\"external_funcs\" 
rustc -C opt-level=2 -Z no-landing-pads --target thumbv7m-none-eabi -g src/librustc_unicode/lib.rs --out-dir libs_arm -L libs-arm --emit obj,link
rustc -C opt-level=2 -Z no-landing-pads --target thumbv7m-none-eabi -g src/libcollections/lib.rs --out-dir libs_arm -L libs-arm --emit obj,link