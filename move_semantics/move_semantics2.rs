fn main() {
    // Method 3: Make `fill_vec` *mutably* borrow a reference to its argument (which will need to be
    // mutable), modify it directly, then not return anything. Then you can get rid of `vec1`
    // entirely -- note that this will change what gets printed by the first `println!`

    // `vec0` must be mutable so we can modify it directly
    let mut vec0 = Vec::new();
    
    // `vec1` is no longer needed. Instead, we pass `vec0` mutably to `fill_vec()` and allow it to
    // modify it directly.
    fill_vec(&mut vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    // Push to `vec0` since `vec1` is no longer present with this method
    vec0.push(88);

    // Same here. Everywhere that `vec1` appeared need to be changed to `vec0`
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
}

// Take a mutable reference. Return nothing since we are modifying the argument directly
fn fill_vec(vec: &mut Vec<i32>) {
    // Simply push to the vector directly
    vec.push(22);
    vec.push(44);
    vec.push(66);
}