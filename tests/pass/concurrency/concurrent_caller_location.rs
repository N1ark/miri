use std::panic::Location;
use std::thread::spawn;

fn initialize() {
    let _ignore = initialize_inner();
}

fn initialize_inner() -> &'static Location<'static> {
    Location::caller()
}

#[cfg_attr(kani, kani::proof)]
fn main() {
    let j1 = spawn(initialize);
    let j2 = spawn(initialize);
    j1.join().unwrap();
    j2.join().unwrap();
}
