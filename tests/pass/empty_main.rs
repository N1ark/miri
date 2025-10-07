// This may look trivial, but a bunch of code runs in std before
// `main` is called, so we are ensuring that that all works.
#[cfg_attr(kani, kani::proof)]
fn main() {}
