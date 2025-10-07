#[cfg_attr(kani, kani::proof)]
fn main() {
    core::panic!("{}-panicking from libcore", 42);
}
