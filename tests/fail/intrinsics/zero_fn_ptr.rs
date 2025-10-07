#[allow(deprecated, invalid_value)]
#[cfg_attr(kani, kani::proof)]
fn main() {
    let _ = unsafe { std::mem::zeroed::<fn()>() }; //~ERROR: constructing invalid value
}
