// This is similar to from_into.rs, but this time we implement `FromStr` and return errors instead
// of falling back to a default value. Additionally, upon implementing FromStr, we use the `parse`
// method on strings to generate an object of the implementor type.

// You can read more about it at https://doc.rust-lang.org/std/str/trait.FromStr.html

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: usize,
}

// We will use this error type for the `FromStr` implementation.
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // Empty input string
    Empty,
    // Incorrect number of fields
    BadLen,
    // Empty name field
    NoName,
    // Wrapped error from parse::<usize>()
    ParseInt(ParseIntError),
}

// As an aside: `Box<dyn Error>` implements `From<&'_ str>`. This means that if you want to return a
// string error message, you can do so via just using return `Err("my error message".into())`.

impl FromStr for Person {
    type Err = ParsePersonError;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        // If the length of the provided string is 0, an error should be returned
        if s.len() == 0 {
            return Err(ParsePersonError::Empty);
        }

        // Split the given string on the commas present in it
        let split: Vec<_> = s.split(',').collect();

        // Only 2 elements should be returned from the split, otherwise return an error
        if split.len() != 2 {
            return Err(ParsePersonError::BadLen);
        }

        // Extract the first element from the split operation and use it as the name
        let name = split[0];

        // If the name is empty, then return an error
        if name == "" {
            return Err(ParsePersonError::NoName);
        }

        // Extract the other element from the split operation
        let age = split[1]
            // parse it into a `usize` as the age
            .parse::<usize>()
            // If while extracting the age something goes wrong, an error should be returned. Use
            // the `map_err` method of `Result` to wrap the error. Propagate errors using the  `?`
            // operator
            .map_err(ParsePersonError::ParseInt)?;

        // If everything goes well, then return a Result of a Person object
        Ok(Person {
            name: name.to_string(),
            age,
        })
    }
}

fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(ParsePersonError::Empty));
    }
    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }
    #[test]
    fn missing_age() {
        assert!(matches!(
            "John,".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!(
            "John,twenty".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!("John".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(",1".parse::<Person>(), Err(ParsePersonError::NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(matches!(
            ",".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            ",one".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn trailing_comma() {
        assert_eq!("John,32,".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!(
            "John,32,man".parse::<Person>(),
            Err(ParsePersonError::BadLen)
        );
    }
}
