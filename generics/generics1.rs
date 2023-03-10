fn main() {
    // We will be pushing string slices into the vector, so `shopping_list`'s type is `Vec<&str>`.
    let mut shopping_list: Vec<&str> = Vec::new();
    shopping_list.push("milk");
}
