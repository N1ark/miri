//@compile-flags: -Cpanic=abort
#![no_std]
#![no_main]

use std::fmt::Write;

#[path = "../utils/mod.no_std.rs"]
mod utils;

#[no_mangle]
fn miri_start(_argc: isize, _argv: *const *const u8) -> isize {
    writeln!(utils::MiriStdout, "hello, world!").unwrap();
    0
}

#[panic_handler]
fn panic_handler(_: &std::panic::PanicInfo) -> ! {
    loop {}
}
