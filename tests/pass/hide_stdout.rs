//@compile-flags: -Zmiri-mute-stdout-stderr

#[cfg_attr(kani, kani::proof)]
fn main() {
    println!("print to stdout");
    eprintln!("print to stderr");
}
