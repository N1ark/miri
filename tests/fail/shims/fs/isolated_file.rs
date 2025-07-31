//@ignore-target: windows # File handling is not implemented yet
//@error-in-other-file: `open` not available when isolation is enabled

#[cfg_attr(kani, kani::proof)]
fn main() {
    let _file = std::fs::File::open("file.txt").unwrap();
}
