fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

fn current_favorite_color() -> String {
    // This function's return type is `String`. `"blue"` looks like a `String`, but is actually a
    // string literal of type `&str`. String literals are actually a slice of bytes `&[u8]` that are
    // encoded in UTF-8. This means they are immutable and cannot be modified at runtime like
    // `String`s can. Use the `to_string()` method on a string literal to copy its contents into a
    // new `String`.
    "blue".to_string()
}
