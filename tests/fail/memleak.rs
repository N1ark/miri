//@normalize-stderr-test: ".*│.*" -> "$$stripped$$"

#[kani::proof]
fn main() {
    std::mem::forget(Box::new(42)); //~ERROR: memory leaked
}
