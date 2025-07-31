#[no_mangle]
fn foo() {}

#[cfg_attr(kani, kani::proof)]
fn main() {
    extern "Rust" {
        fn foo(_: i32);
    }
    unsafe { foo(1) } //~ ERROR: calling a function with more arguments than it expected
}
