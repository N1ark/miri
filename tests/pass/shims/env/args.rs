#[cfg_attr(kani, kani::proof)]
fn main() {
    for arg in std::env::args() {
        println!("{}", arg);
    }
}
