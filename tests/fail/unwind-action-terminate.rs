//@error-in-other-file: aborted execution
//@normalize-stderr-test: "\|.*::abort\(\).*" -> "| ABORT()"
//@normalize-stderr-test: "\| +\^+" -> "| ^"
//@normalize-stderr-test: "\n +[0-9]+:[^\n]+" -> ""
//@normalize-stderr-test: "\n +at [^\n]+" -> ""
extern "C" fn panic_abort() {
    panic!()
}

#[cfg_attr(kani, kani::proof)]
fn main() {
    panic_abort();
}
