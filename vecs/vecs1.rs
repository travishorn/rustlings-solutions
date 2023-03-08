// Create a `Vec` which holds the exact same elements as in the array `a`.

// While arrays are always a fixed value, vectors are similar, but their size can be changed.

fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // a plain array

    // Vectors can be created with the `vec!` macro or with `Vec::new()`
    let v = vec![10, 20, 30, 40]; // a vector

    (a, v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, v[..]);
    }
}
