fn f() {}

#[cfg_attr(kani, kani::proof)]
fn main() {
    let x: u8 = unsafe {
        *std::mem::transmute::<fn(), *const u8>(f) //~ ERROR: contains a function
    };
    panic!("this should never print: {}", x);
}
