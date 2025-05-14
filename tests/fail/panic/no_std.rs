//@compile-flags: -Cpanic=abort
#![feature(core_intrinsics)]
#![no_std]
#![no_main]

use std::fmt::Write;

#[path = "../../utils/mod.no_std.rs"]
mod utils;

#[no_mangle]
fn miri_start(_argc: isize, _argv: *const *const u8) -> isize {
    panic!("blarg I am dead")
}

#[panic_handler]
fn panic_handler(panic_info: &std::panic::PanicInfo) -> ! {
    writeln!(utils::MiriStderr, "{panic_info}").ok();
    std::intrinsics::abort(); //~ ERROR: the program aborted execution
}
