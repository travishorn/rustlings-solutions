// At compile time, Rust needs to know how much space a type takes up. This becomes problematic for
// recursive types, where a value can have as part of itself another value of the same type. To get
// around the issue, we can use a `Box` - a smart pointer used to store data on the heap, which also
// allows us to wrap a recursive type.
//
// The recursive type we're implementing in this exercise is the `cons list` - a data structure
// frequently found in functional programming languages. Each item in a cons list contains two
// elements: the value of the current item and the next item. The last item is a value called `Nil`.
//
// Here we use a `Box` in the enum definition and create both empty and non-empty cons lists

#[derive(PartialEq, Debug)]
pub enum List {
    // Wrap `List` in the `Box` smart pointer type to allow data allocation on the heap. Boxes have
    // a known size, so Rust doesn't complain about a recursive tpe with infinite size using this
    // method.
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list()
    );
}

pub fn create_empty_list() -> List {
    // Use the enum variant `Nil` to return an empty list
    List::Nil
}

pub fn create_non_empty_list() -> List {
    // Use the enum variant `Cons(...)` to return a non-empty list. The first element must be an i32
    // and the second must be a `Box<List>`. We can end the recursion right here by making the
    // second element an empty list
    List::Cons(0, Box::new(List::Nil))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(List::Nil, create_empty_list())
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list())
    }
}
