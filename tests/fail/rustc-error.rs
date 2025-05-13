// Make sure we exit with non-0 status code when the program fails to build.
#[kani::proof]
fn main() {
    println("Hello, world!"); //~ ERROR: expected function, found macro
}
