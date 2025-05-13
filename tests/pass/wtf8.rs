//@only-target: windows

use std::ffi::{OsStr, OsString};
use std::os::windows::ffi::{OsStrExt, OsStringExt};

fn test1() {
    let base = "a\té \u{7f}💩\r";
    let mut base: Vec<u16> = OsStr::new(base).encode_wide().collect();
    base.push(0xD800);
    let _res = OsString::from_wide(&base);
}

fn test2() {
    let mut base: Vec<u16> = OsStr::new("aé ").encode_wide().collect();
    base.push(0xD83D);
    let mut _res: Vec<u16> = OsString::from_wide(&base).encode_wide().collect();
}

#[kani::proof]
fn main() {
    test1();
    test2();
}
