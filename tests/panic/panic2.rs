#[cfg_attr(kani, kani::proof)]
fn main() {
    std::panic!("{}-panicking from libstd", 42);
}
