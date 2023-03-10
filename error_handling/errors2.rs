// Say we're writing a game where you can buy items with tokens. All items cost 5 tokens, and
// whenever you purchase items there is a processing fee of 1 token. A player of the game will type
// in how many items they want to buy, and the `total_cost` function will calculate the total number
// of tokens. Since the player typed in the quantity, though, we get it as a string-- and they might
// have typed anything, not just numbers!

// This function handles the error case. if we call the `parse` function on a string that is not a
// number, that function will return a `ParseIntError`, and in that case, we immediately return that
// error from our function and not try to multiply and add.

use std::num::ParseIntError;

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>();

    // Use a match expression to determine if the result of the `parse` was an error. If so, return
    // that same error (aka propagating the error). Otherwise, if the result is `Ok`, do the math.
    match qty {
        Err(e) => Err(e),
        Ok(num) => Ok(num * cost_per_item + processing_fee),
    }

    // There is a shortcut to doing the above, since all we're doing is propagating the error if one
    // exists. You can add a `?` operator to the `parse`.
    // let qty = item_quantity.parse::<i32>()?;
    //
    // And then you can do away with the match expression. Just do the math. `qty` now contains the
    // i32 instead of `Result<i32, ParseIntError>`. Ok(qty * cost_per_item + processing_fee)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().to_string(),
            "invalid digit found in string"
        );
    }
}
