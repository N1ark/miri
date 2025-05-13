// Only works on Unix targets
//@ignore-target: windows wasm
//@only-on-host
//@normalize-stderr-test: "OS `.*`" -> "$$OS"

extern "C" {
    fn foo();
}

#[kani::proof]
fn main() {
    unsafe {
        foo(); //~ ERROR: unsupported operation: can't call foreign function `foo`
    }
}
