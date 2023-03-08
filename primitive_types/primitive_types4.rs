#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    // A slice is a reference to a contiguous sequence of elements in a collection rather than the
    // whole collection. &a gets a reference to the array named a, but only starting at index 1 and
    // ending at (but not including) index 4.
    let nice_slice = &a[1..4];

    assert_eq!([2, 3, 4], nice_slice)
}
