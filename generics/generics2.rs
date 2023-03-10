// This powerful wrapper provides the ability to store ANY type.

// To use a generic type on a struct, add `<T>` after the name
struct Wrapper<T> {
    // Now we can use `T` (the generic type) when defining types where you'd normally see something
    // like `u32`, etc.
    value: T,
}

// In the implementation of the struct, we must add `<T>` after both `impl` and the name
impl<T> Wrapper<T> {
    // Now we can use `T` (the generic type) when annotating function parameters
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
