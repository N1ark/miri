#![allow(invalid_value)]

union MyUninit {
    init: (),
    uninit: [char; 1],
}

#[kani::proof]
fn main() {
    let _b = unsafe { MyUninit { init: () }.uninit }; //~ ERROR: constructing invalid value
}
