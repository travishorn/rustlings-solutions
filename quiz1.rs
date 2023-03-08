// Mary is buying apples. The price of an apple is calculated as follows:
// - An apple costs 2 rustbucks.
// - If Mary buys more than 40 apples, each apple only costs 1 rustbuck!
// Calculate the price of an order of apples given the quantity bought

// Accept an argument called quantity. It is always a positive integer between 35 and 65 so u8 works
// well. The return value will be a positive integer between 41 and 80 so u8 works here, too.
fn calculate_price_of_apples(quantity: u8) -> u8 {
    // Use an `if` to determine whether the quantity is more than 40
    if quantity > 40 {
        // If so, each apple costs 1 rustbuck, so the price is quantity * 1, or just simply quantity
        quantity
    } else {
        // If not, each apple costs 2 rustbucks, so the price is quantity * 1
        quantity * 2
    }
}

#[test]
fn verify_test() {
    let price1 = calculate_price_of_apples(35);
    let price2 = calculate_price_of_apples(40);
    let price3 = calculate_price_of_apples(41);
    let price4 = calculate_price_of_apples(65);

    assert_eq!(70, price1);
    assert_eq!(80, price2);
    assert_eq!(41, price3);
    assert_eq!(65, price4);
}
