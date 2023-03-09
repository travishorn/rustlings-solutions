// hashmaps1.rs A basket of fruits in the form of a hash map is defined. The key represents the name
// of the fruit and the value represents how many of that particular fruit is in the basket. We put
// three different types of fruits (apple, banana, mango) in the basket and the total count of all
// the fruits is five.

use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    // Create a new, empty, mutable HashMap
    let mut basket = HashMap::new();

    // Insert fruits into the `basket` HashMap
    basket.insert(String::from("banana"), 2);
    basket.insert(String::from("apple"), 2);
    basket.insert(String::from("mango"), 1);

    basket
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}
