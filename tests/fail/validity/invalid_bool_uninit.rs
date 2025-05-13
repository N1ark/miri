#![allow(invalid_value)]

union MyUninit {
    init: (),
    uninit: [bool; 1],
}

#[kani::proof]
fn main() {
    let _b = unsafe { MyUninit { init: () }.uninit }; //~ ERROR: constructing invalid value
}
