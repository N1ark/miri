//@error-in-other-file: the program aborted execution
//@normalize-stderr-test: "\| +\^+" -> "| ^"
//@normalize-stderr-test: "unsafe \{ libc::abort\(\); \}|std::intrinsics::abort\(\);" -> "ABORT();"
//@compile-flags: -C panic=abort

fn main() {
    std::panic!("{}-panicking from libcore", 42);
}
