// A classic strut contains named fields
struct ColorClassicStruct {
    red: u8,
    green: u8,
    blue: u8,
}

// A tuple struct contains unnamed fields.
struct ColorTupleStruct(u8, u8, u8);

// A unit-like struct contains no fields. Useful when you need to implement a trait on some type but
// don't have any data that you want to store in the type itself
#[derive(Debug)]
struct UnitLikeStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // Instantiate a classic c struct!
        let green = ColorClassicStruct {
            red: 0,
            green: 255,
            blue: 0,
        };

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // Instantiate a tuple struct!
        let green = ColorTupleStruct(0, 255, 0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // Instantiate a unit-like struct!
        let unit_like_struct = UnitLikeStruct;
        let message = format!("{:?}s are fun!", unit_like_struct);

        assert_eq!(message, "UnitLikeStructs are fun!");
    }
}
