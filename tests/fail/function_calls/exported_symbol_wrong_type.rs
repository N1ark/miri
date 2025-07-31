#[no_mangle]
static FOO: () = ();

#[cfg_attr(kani, kani::proof)]
fn main() {
    extern "C" {
        fn FOO();
    }
    unsafe { FOO() } //~ ERROR: attempt to call an exported symbol that is not defined as a function
}
