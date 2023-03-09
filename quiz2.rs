// Let's build a little machine in the form of a function.
// As input, we're going to give a list of strings and commands. These commands determine what
// action is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple, the first element is the string, the
//   second one is the command.
// - The output element is going to be a Vector of strings.

pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // `input` is a vector of a 2-length tuple. The first value in the tuple is a string and the
    // second is a `Command` enum we defined earlier
    // The return value of this function is a vector of strings
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        // Remember, the return value is a vector of strings. Since we're returning `output` later
        // it must have that type
        let mut output: Vec<String> = vec![];
        for (string, command) in input.iter() {
            // Match on the type of command
            match command {
                // Uppercase: modify the string so it is uppercase and push it to the output vector
                Command::Uppercase => output.push(string.to_uppercase()),

                // Trim: trim whitespace from the string, convert it to a string (since `trim()`
                // returns a string slice), and push it intot he output vector
                Command::Trim => output.push(string.trim().to_string()),

                // Append: append "bar" to the string a given number of times
                Command::Append(bar_length) => {
                    // Create a new string with the contents of the original string
                    let mut result = string.to_owned();

                    // Loop for the given number of times
                    // We use `_i` here instead of `i` because we don't use `i` at all in the loop
                    // Start at 0 and end at the given `bar_length`
                    // We use `*` because `bar_length` is a reference to the given bar length
                    // (number of times to append "bar"). `*` dereferences `bar_length` so we can
                    // access the underlying value (1 or 5 in this case) it represents
                    for _i in 0..*bar_length {
                        result.push_str("bar");
                    }

                    // Push the result to the output vector
                    output.push(result)
                }
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::Command;

    // Bring the `transformer` function from the `my_module` module into scope
    use crate::my_module::transformer;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
