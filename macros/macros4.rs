macro_rules! my_macro {
    // Macros use a pattern matching pattern similar to a `match` expression.
    // Make sure to separate arms of the macro with semicolons `;`
    () => {
        println!("Check out my macro!");
    };
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    };
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
