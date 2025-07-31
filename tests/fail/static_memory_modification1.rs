// Stacked Borrows detects that we are casting & to &mut and so it changes why we fail
//@compile-flags: -Zmiri-disable-stacked-borrows

static X: usize = 5;

#[allow(mutable_transmutes)]
#[cfg_attr(kani, kani::proof)]
fn main() {
    unsafe {
        *std::mem::transmute::<&usize, &mut usize>(&X) = 6; //~ ERROR: read-only
        assert_eq!(X, 6);
    }
}
