pub fn bigger(a: i32, b: i32) -> i32 {
    // Return the bigger number!
    // Use `if` to implement conditional logic
    if a > b {
        a
    } else {
        b
    }
}

// Tests to make sure bigger() is working as intended
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }
}
