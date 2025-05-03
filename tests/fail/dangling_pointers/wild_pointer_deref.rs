//@compile-flags: -Zmiri-permissive-provenance

#[cfg_attr(kani, kani::proof)]
fn main() {
    let p = 44 as *const i32;
    let x = unsafe { *p }; //~ ERROR: is a dangling pointer
    panic!("this should never print: {}", x);
}
