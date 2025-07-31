#[no_mangle]
extern "C" fn malloc(_: usize) -> *mut std::ffi::c_void {
    //~^ HELP: the `malloc` symbol is defined here
    unreachable!()
}

#[cfg_attr(kani, kani::proof)]
fn main() {
    extern "C" {
        fn malloc(_: usize) -> *mut std::ffi::c_void;
    }
    unsafe {
        malloc(0);
        //~^ ERROR: found `malloc` symbol definition that clashes with a built-in shim
    }
}
