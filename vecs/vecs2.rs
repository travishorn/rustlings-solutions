// vec_loop and vec_map both take vector and multiply all its elements by 2.
// vec_loop uses mutations and vec_map does not

fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    // the `iter_mut` method provides mutable references to each element in the vector
    for i in v.iter_mut() {
        // Multiply each element in the vector by 2
        // `*` is the dereference operator that dereferences the iterator `i`, giving access to the
        // actual value stored at that memory location. `*= 2` sets a value to itself muliplied by
        // 2.
        *i *= 2
    }

    // `v` is equal to the original given v, with all value multiplied by 2.
    v
}

fn vec_map(v: &Vec<i32>) -> Vec<i32> {
    // Iterate over the given vector `v`
    v.iter()
        // map each value to something else
        .map(|num| {
            // That "something else" in this case is the value itself multiplied by 2
            num * 2
        })
        // The `collect()` method takes an iterator and produces a collection (such as a vector)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_loop(v.clone());

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }

    #[test]
    fn test_vec_map() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_map(&v);

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }
}
