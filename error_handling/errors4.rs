#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        // Use a `match` expression to test different conditions of the given value
        match value {
            // If the value is less than 0, it's negative; return the appropriate error
            v if v < 0 => Err(CreationError::Negative),

            // If the value is 0, return the appropriate error
            v if v == 0 => Err(CreationError::Zero),

            // If it's neither of the above, return `Ok` with the value as a `u64`
            _ => Ok(PositiveNonzeroInteger(value as u64)),
        }
    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}
