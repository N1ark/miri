static X: usize = 5;

#[allow(mutable_transmutes)]
#[cfg_attr(kani, kani::proof)]
fn main() {
    let _x = unsafe {
        std::mem::transmute::<&usize, &mut usize>(&X) //~ ERROR: writing to alloc1 which is read-only
    };
}
