fn main() {
    let mut x = 100;

    // Only one mutable reference to a variable can be active at a time.
    // Since `y` is a mutable reference to `x`, we need to do things with it now...
    let y = &mut x;
    *y += 100;

    // ...before we make another mutable reference to x and do something else with it.
    let z = &mut x;
    *z += 1000;

    assert_eq!(x, 1200);
}
