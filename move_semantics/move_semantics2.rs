fn main() {
    let vec0 = Vec::new();

    // Method 2: Make `fill_vec` borrow its argument instead of taking ownership of it, and then
    // copy the data within the function in order to return an owned `Vec<i32>`
    //
    // Once we pass `vec0` to `fill_vec()`, we can no longer access it in this scope. We can fix
    // this by passing it as a reference
    let mut vec1 = fill_vec(&vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// Change the argument type to accept a reference
fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
    // Shadow the `vec` variable with a clone of it's original value
    let mut vec = vec.clone();

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
