// This function returns how much icecream there is left in the fridge.
// If it's before 10PM, there's 5 pieces left. At 10PM, someone eats them
// all, so there'll be no more left :(
fn maybe_icecream(time_of_day: u16) -> Option<u16> {
    // We use the 24-hour system here, so 10PM is a value of 22 and 12AM is a value of 0
    // The Option output gracefully handles cases where time_of_day > 23.

    // Match on the time of day
    match time_of_day {
        // Between 0 and 21 (12AM and 9PM), return 5. Wrap in `Some()` so it's of type `Option`
        0..=21 => Some(5),

        // Between 22 and 24 (10PM and 12AM), return 0.
        22..=24 => Some(0),

        // Any other time of day should not exist, return `None`
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(10), Some(5));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(25), None);
    }

    #[test]
    fn raw_value() {
        let icecreams = maybe_icecream(12);

        // `icereams` is of type `Option<u16>` because it's the return value from the
        // `maybe_icecream` function. The right half of the assertion must be of type `Option<u16>`,
        // as well. `Some(u16)` and `None` qualify.
        assert_eq!(icecreams, Some(5));
    }
}
