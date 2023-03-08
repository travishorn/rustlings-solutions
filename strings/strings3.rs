fn trim_me(input: &str) -> String {
    // Remove whitespace from both ends of a string!
    // `.trim()` returns another `&str` since it is just a slice of the original data. That way, it
    // is more efficient for memory management. So we use `to_string()` to coerce it into the
    // expected `String` type
    input.trim().to_string()
}

fn compose_me(input: &str) -> String {
    // Add " world!" to the string!
    // The `+` operator can concatenate strings, but not string literals (`&str`s). This is because
    // It takes ownership of the left-hand side of the operation. Since `input` is a `&str`, we
    // cannot use take ownership of it, and cannot use `+`. We can, however, use the `format!` macro
    // instead.
    format!("{}{}", input, " world!")

    // Another option, if we were using `input` more than once, would be to create a single new
    // `String instance with `let mut result = input.to_owned();` and then modify it as many times
    // as necessary. For example, we could append to it with `result.push_str(" world!");`.
}

fn replace_me(input: &str) -> String {
    // Replace "cars" in the string with "balloons"!
    // `.replace()` returns a `String` because it is mutating `input`
    input.replace("cars", "balloons")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool"
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons"
        );
    }
}
