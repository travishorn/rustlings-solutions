// Macros must be defined before they are used. It is common practice to place macro definitions at
// the top of a file.
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
