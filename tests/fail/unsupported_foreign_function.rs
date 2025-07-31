//@normalize-stderr-test: "OS `.*`" -> "$$OS"

#[cfg_attr(kani, kani::proof)]
fn main() {
    extern "Rust" {
        fn foo();
    }

    unsafe {
        foo(); //~ ERROR: unsupported operation: can't call foreign function `foo`
    }
}
