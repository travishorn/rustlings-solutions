fn main() {
    let vec0 = Vec::new();

    // Must declare vec1 as mutable (`mut`) here so it can be borrowed as mutable later
    let mut vec1 = fill_vec(vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    // vec1 is borrowed as mutable here. It can be borrowed in this way here because the variable
    // was declared as `mut`
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
