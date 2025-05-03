//@error-in-other-file: the program aborted execution
//@normalize-stderr-test: "\| +\^+" -> "| ^"
//@normalize-stderr-test: "\|.*::abort\(\).*" -> "| ABORT()"
//@compile-flags: -C panic=abort

#[cfg_attr(kani, kani::proof)]
fn main() {
    std::panic!("{}-panicking from libstd", 42);
}
