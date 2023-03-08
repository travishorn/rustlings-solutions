// Using a tuple index to access the second element of `numbers`.

#[test]
fn indexing_tuple() {
    let numbers = (1, 2, 3);

    // Values inside tuples can be accessed by using a period (.) followed by the index
    let second = numbers.1;

    assert_eq!(2, second, "This is not the 2nd number in the tuple!")
}
