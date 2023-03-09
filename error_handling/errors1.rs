// This function refuses to generate text to be printed on a nametag if you pass it an empty string.
// It's nice because it explains what the problem is, instead of just sometimes returning `None`.
// `Result` is a similar construct to `Option` that can be used to express error conditions. Let's
// use it!

// Instead of using the `Option<String>` return type, we use the `Result<String, String>` return
// type, which allows us to return `Ok()` containing a string with our return value if everything is
// good, or `Err()` with an error string if things are not all good.
pub fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.is_empty() {
        Err("`name` was empty; it must be nonempty.".into())
    } else {
        Ok(format!("Hi! My name is {}", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".into()),
            Ok("Hi! My name is Beyoncé".into())
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".into()),
            // Don't change this line
            Err("`name` was empty; it must be nonempty.".into())
        );
    }
}
