#[kani::proof]
fn main() {
    println!("Hello, world!");
    eprintln!("Hello, error!");
}
