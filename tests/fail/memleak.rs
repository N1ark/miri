//@normalize-stderr-test: ".*│.*" -> "$$stripped$$"

#[cfg_attr(kani, kani::proof)]
fn main() {
    std::mem::forget(Box::new(42)); //~ERROR: memory leaked
}
