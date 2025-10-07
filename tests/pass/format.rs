#[cfg_attr(kani, kani::proof)]
fn main() {
    println!("Hello {}", 13);
    println!("{:0<width$}", "hello", width = 10);
}
