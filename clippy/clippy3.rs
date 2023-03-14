#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    if my_option.is_none() {
        // `unwrap()` will always fail because we're inside a conditional statement which guarantees
        // `my_option()` is `None`, and therefor has no `unwrap()` method. Realistically, you would
        // probably just remove the declaration of `my_option` and this `if` statement. They are
        // left in here for the sake of demonstration.

        // my_option.unwrap();
    }

    // Add missing comma `,` as suggested by clippy
    let my_arr = &[-1, -2, -3, -4, -5, -6];
    println!("My array! Here it is: {:?}", my_arr);

    // `resize()` and `clear()` both return unit `()`. If we want to demonstrate clearing a vector,
    // define the vector as mutable first.
    let mut my_empty_vec = vec![1, 2, 3, 4, 5];

    // Then use `clear()`, not `resize()`
    my_empty_vec.clear();

    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;

    // Let's swap these two!
    // Use the built-in `std::mem:swap()` function
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
