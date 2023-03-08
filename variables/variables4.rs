fn main() {
    // Variables are immutable by default. If you need to assign them more than once, use the `mut`
    // keyword.
    let mut x = 3;
    println!("Number {}", x);
    x = 5;
    println!("Number {}", x);
}
