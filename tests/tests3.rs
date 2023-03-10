pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    // This test tests our function in such a way that the test passes.
    #[test]
    fn is_true_when_even() {
        assert!(is_even(0));
    }

    // This second test tests whether we get the result we expect to get when we call `is_even(5)`.
    #[test]
    fn is_false_when_odd() {
        assert!(is_even(5) == false);

        // This test could also be written with `assert_eq!()` like so
        // assert_eq!(is_even(5), false);
    }
}
