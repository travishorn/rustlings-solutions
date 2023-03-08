fn main() {
    let word = String::from("green");

    // `is_a_color_word()` is expecting an argument of type `&str`, while our `word` is of type
    // `String`. Passing the variable as a reference (with `&`) actually passes a reference to the
    // underlying byte data, which is of type `&str`
    if is_a_color_word(&word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}

fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}
