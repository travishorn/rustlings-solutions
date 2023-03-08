fn main() {
    // We will mutate data later, so it must be declared as mutable here
    let mut data = "Rust is great!".to_string();

    // `get_char` does not take ownership, so we create a copy of the data before passing it to that
    // function.
    get_char(data.clone());

    // We are passing a reference to data, and it needs to be mutable
    string_uppercase(&mut data);
}

// Does not take ownership
fn get_char(data: String) -> char {
    data.chars().last().unwrap()
}

// Does take ownership
// `mut data: &String` would take an immutable reference to a string and then make a mutable copy.
// `data: &mut String` takes a mutable reference directly so the function can modify the string in
// place.
fn string_uppercase(data: &mut String) {
    // `data = &data.to_uppercase();` would create a new reference to the new string instance
    // returned by `to_uppercase()` and assign it to the `data` variable.
    // `*data = data.to_uppercase();` dereferences the data pointer to access the original string
    // instance and replaces the contents with the new string instance created by `to_uppercase()`.
    *data = data.to_uppercase();

    println!("{}", data);
}
