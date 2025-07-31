#[cfg_attr(kani, kani::proof)]
fn main() {
    std::panic!("panicking from libcore");
}
