macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    // To use a macro, call it by name with the `!` symbol appended to the end
    my_macro!();
}
