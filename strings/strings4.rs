fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    // A string literal is a `&str`. So "blue" is a `&str
    string_slice("blue");

    // `to_string()` creates a new string and copies the contents of the string literal into it
    string("red".to_string());

    // `String::from()` is a generic function that does the same thing as `to_string()`
    string(String::from("hi"));

    // `to_owned()` is similar to `to_string()` except that it creates a new instance that is
    // completely independent and owns its own memory while `to_string()` creates a new `String`
    // instance that contains a copy of the original data
    string("rust is fun!".to_owned());

    // `into()` is a more generic method that does the same thing as `to_owned()`
    string("nice weather".into());

    // The `format!` macro returns a new `String`
    string(format!("Interpolation {}", "Station"));

    // Here we are creating a reference to a string slice by accessing its indexes with a range
    string_slice(&String::from("abc")[0..1]);

    // `trim()` returns a string slice as it's simply a substring of another string. It can be more
    // memory efficient this way.
    string_slice("  hello there ".trim());

    // `replace()` returns a string because it's mutating some other string
    string("Happy Monday!".to_string().replace("Mon", "Tues"));

    // `to_lowercase()` returns a string for the same reason
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
