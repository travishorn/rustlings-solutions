fn main() {
    let mut res = 42;
    let option = Some(12);

    // Using `if let` here is more readable. The intent is clearer.
    if let Some(x) = option {
        res += x;
    }

    println!("{}", res);
}
