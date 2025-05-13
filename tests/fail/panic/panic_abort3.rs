//@error-in-other-file: the program aborted execution
//@normalize-stderr-test: "\| +\^+" -> "| ^"
//@normalize-stderr-test: "unsafe \{ libc::abort\(\); \}|core::intrinsics::abort\(\);" -> "ABORT();"
//@compile-flags: -C panic=abort

#[kani::proof]
fn main() {
    core::panic!("panicking from libcore");
}
