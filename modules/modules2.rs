// You can bring module paths into scopes and provide new names for them with the `use` and `as`
// keywords.

mod delicious_snacks {
    // Although `mod fruits` and `mod veggies` are not public below, we can still bring them into
    // scope with `pub use`. We also give them a different name with `as`. Now they can be accessed
    // with these names in `main()` below
    pub use self::fruits::PEAR as fruit;
    pub use self::veggies::CUCUMBER as veggie;

    mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie
    );
}
