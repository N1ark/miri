//@compile-flags: -Zmiri-mute-stdout-stderr

#[kani::proof]
fn main() {
    println!("print to stdout");
    eprintln!("print to stderr");
}
