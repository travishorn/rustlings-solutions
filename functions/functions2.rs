fn main() {
    call_me(3);
}

// All arguments must have a type annotation
fn call_me(num: u8) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
