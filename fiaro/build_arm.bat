@echo off
prompt $G 
cd %~dp0
rd /s /q target\thumbv7m-none-eabi 2>nul
mkdir target\thumbv7m-none-eabi 2>nul
@echo on

rustc -C opt-level=0 -g -Z no-landing-pads -L libs_arm ^
--target thumbv7m-none-eabi ^
--crate-type lib ^
--crate-name fiaro ^
--emit obj ^
--out-dir target/thumbv7m-none-eabi ^
src/lib_target.rs