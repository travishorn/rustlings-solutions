// Implements the trait `AppendBar` for a vector of strings.
//
// Append "Bar" to a vector of strings means to push the value "Bar" into the vector

trait AppendBar {
    fn append_bar(self) -> Self;
}

// Implement trait `AppendBar` for a vector of strings.
impl AppendBar for Vec<String> {
    // Implement `AppendBar` for type `Vec<String>`. `Vec<String>` implements the `Copy` trait which
    // means it is copied by default instead of being moved. We can use `mut self` instead of `&mut
    // self` since we have a copy of the data and not just a reference.
    fn append_bar(mut self) -> Vec<String> {
        // Push "Bar" into the vector and return it
        self.push(String::from("Bar"));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}
