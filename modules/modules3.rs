// Bring SystemTime and UNIX_EPOCH from the std::time module.
use std::time::{SystemTime, UNIX_EPOCH};

// Alternatively, you can bring them in separately.
// use std::time::SystemTime;
// use std::time::UNIX_EPOCH;

fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
