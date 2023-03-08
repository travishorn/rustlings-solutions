fn main() {
    // Characters (`char`)

    // Note the _single_ quotes, these are different from double quotes. Single quotes represent
    // single characters instead of strings of characters.
    let my_first_initial = 'C';
    if my_first_initial.is_alphabetic() {
        println!("Alphabetical!");
    } else if my_first_initial.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }

    // chars can be aphabetic, numeric, or even something else like an emoji
    let your_character = '🦀';
    if your_character.is_alphabetic() {
        println!("Alphabetical!");
    } else if your_character.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }
}
