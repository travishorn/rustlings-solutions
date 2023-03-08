// Instead of passing `vec0` into the `fill_vec` function, the Vector gets created in the function
// itself and passed back to the main function.

fn main() {
    // `fill_vec` no longer takes any arguments
    let mut vec1 = fill_vec();

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// `fill_vec()` no longer takes `vec: Vec<i32>` as argument
fn fill_vec() -> Vec<i32> {
    // vec is now created as a mutable new vector
    let mut vec = Vec::new();

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
