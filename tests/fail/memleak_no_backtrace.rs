//@compile-flags: -Zmiri-disable-leak-backtraces
//@error-in-other-file: memory leaked
//@normalize-stderr-test: ".*│.*" -> "$$stripped$$"

#[kani::proof]
fn main() {
    std::mem::forget(Box::new(42));
}
