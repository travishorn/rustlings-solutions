// The compiler is just validating the references passed to the annotated parameters and the return
// type.

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    // `string2` must be up here (outside the inner block) so that it can be used when printing the
    // `result` below.
    let string2 = String::from("xyz");
    let result;
    {
        // When `string2` was in this inner block, we got a compile error because it only lives
        // until the end of this inner block. When it died, you could not print `result` because
        // `result` was the result of `longest()` here, which relied on `string2`.
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is '{}'", result);
}
