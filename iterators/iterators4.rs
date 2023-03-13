pub fn factorial(num: u64) -> u64 {
    // With a range from 1 to the num inclusive,
    // Fold each number,
    // Starting at 1,
    // Multiply the accumulator by the current value
    (1..=num).fold(1, |acc, x| acc * x)

    // Recursive version.
    // If we reached 0, return 1. Otherwise, multiply the current num by the result of factorial
    // with the number 1 less than the current num
    //
    // if num == 0 {
    //     1
    // } else {
    //     num * factorial(num - 1)
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
