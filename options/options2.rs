#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // Initialize `word` equal to `optional_target`. Since `optional_target` is of type
        // `Option<&str>`, `word` is also of type `Option<&str>`.
        if let word = optional_target {
            // This assertion is true because `Some(target)` wraps the string literal in `Option`.
            // This matches `word` which is the same string slice also wrapped in `Option`.
            assert_eq!(word, Some(target));
        }
    }

    #[test]
    fn layered_option() {
        let mut range = 10;
        let mut optional_integers: Vec<Option<i8>> = Vec::new();
        for i in 0..(range + 1) {
            optional_integers.push(Some(i));
        }

        // `optional_integers` is a vector of integers, each wrapped in `Option`. `pop()` wraps it's
        // values in `Option`, as well (because it's possible that we try to pop an empty vector).
        // This means that the value in this `while` loop will be double-wrapped in `Option`. We can
        // use `Some()` to peel back one of those layers. What we're left with is an
        // `Option`-wrapped integer, or `None`.
        while let Some(integer) = optional_integers.pop() {
            // If there is a value in the option-wrapped integer (not `None`)...
            if let Some(value) = integer {
                // Assert that the value is equal to the current range
                assert_eq!(value, range);
                // Decrement the range before continuing the `while` loop
                range -= 1;
            }
        }
    }
}
