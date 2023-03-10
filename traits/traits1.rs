// Implements the trait `AppendBar` for the type `String`. The trait AppendBar has only one
// function, which appends "Bar" to any object implementing this trait.

trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    // Implement `AppendBar` for type `String`
    fn append_bar(self) -> String {
        // Return a `String` with the original content + "Bar"
        format!("{}{}", self, "Bar")
    }
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }
}
