// This may look trivial, but a bunch of code runs in std before
// `main` is called, so we are ensuring that that all works.
#[kani::proof]
fn main() {}
