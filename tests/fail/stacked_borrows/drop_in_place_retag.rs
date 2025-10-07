//! Test that drop_in_place mutably retags the entire place, even for a type that does not need
//! dropping, ensuring among other things that it is writeable

//@error-in-other-file: /retag .* for Unique permission .* only grants SharedReadOnly permission/

#[cfg_attr(kani, kani::proof)]
fn main() {
    unsafe {
        let x = 0u8;
        let x = core::ptr::addr_of!(x);
        core::ptr::drop_in_place(x.cast_mut());
    }
}
