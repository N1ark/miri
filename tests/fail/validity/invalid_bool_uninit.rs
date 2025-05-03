#![allow(invalid_value)]

union MyUninit {
    init: (),
    uninit: [bool; 1],
}

#[cfg_attr(kani, kani::proof)]
fn main() {
    let _b = unsafe { MyUninit { init: () }.uninit }; //~ ERROR: constructing invalid value
}
