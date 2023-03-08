fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    // Lines without a semicolon (;) at the end are considered expressions and return a value
    num * num

    // Alternatively, you can return a value with the `return` keyword
    // return num * num;
}
