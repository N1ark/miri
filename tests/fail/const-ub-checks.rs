const UNALIGNED_READ: () = unsafe {
    let x = &[0u8; 4];
    let ptr = x.as_ptr().cast::<u32>();
    ptr.read(); //~ERROR: accessing memory based on pointer with alignment 1, but alignment 4 is required
};

#[cfg_attr(kani, kani::proof)]
fn main() {
    let _x = UNALIGNED_READ;
}
