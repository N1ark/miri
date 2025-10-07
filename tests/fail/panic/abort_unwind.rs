//@error-in-other-file: the program aborted execution
//@normalize-stderr-test: "\|.*::abort\(\).*" -> "| ABORT()"
//@normalize-stderr-test: "\| +\^+" -> "| ^"
//@normalize-stderr-test: "\n +[0-9]+:[^\n]+" -> ""
//@normalize-stderr-test: "\n +at [^\n]+" -> ""

#![feature(abort_unwind)]

#[cfg_attr(kani, kani::proof)]
fn main() {
    std::panic::abort_unwind(|| panic!("PANIC!!!"));
}
