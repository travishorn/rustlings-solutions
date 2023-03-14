fn main() {
    let vec0 = Vec::new();

    // Method 1: Make another, separate version of  the data that's in `vec0` and pass that to
    // `fill_vec` instead.
    //
    // Once we pass `vec0` to `fill_vec()`, we can no longer access it in this scope. We can fix
    // this by using the `clone` method, which makes a separate copy of the data that's in `vec0`
    // and pass that instead
    let mut vec1 = fill_vec(vec0.clone());

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
