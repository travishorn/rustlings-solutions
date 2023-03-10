// This is a program that uses a completed version of the `total_cost` function from the previous
// exercise.

use std::num::ParseIntError;

// Since we use `?` to propagate errors inside this function, we need to define the return type as
// `Result<(), ParseIntError>`. `()` is the unit type. Basically, we're saying we don't really care
// about the return value if it's not an error.
fn main() -> Result<(), ParseIntError> {
    let mut tokens = 100;
    let pretend_user_input = "8";

    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
    }

    // If we got to this point, the function has done its job. We can return `Ok` with a unit `()`
    Ok(())
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}
