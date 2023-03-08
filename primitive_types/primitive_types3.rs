fn main() {
    // Create an array with 100 elements in it. All of the elements are the boolean value `true`
    let a = [true; 100];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
