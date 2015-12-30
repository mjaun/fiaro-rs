@echo off
cd %~dp0
mkdir target\thumbv7m-none-eabi 2>nul
@echo on

rustc -C opt-level=2 -Z no-landing-pads -g -L libs_arm ^
--target thumbv7m-none-eabi ^
--crate-type lib ^
--crate-name fiaro ^
--out-dir target/thumbv7m-none-eabi ^
--emit obj,link ^
src/lib_target.rs