// When performing operations on elements within a collection, iterators are essential.
// This module helps you get familiar with the structure of using an iterator and
// how to go through elements within an iterable collection.

fn main() {
    let my_fav_fruits = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];

    // Get an iterator from `my_fav_fruits`
    let mut my_iterable_fav_fruits = my_fav_fruits.iter();

    // One-by-one, go through each element in the vector and assert its value
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"banana"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"custard apple"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"avocado"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"peach"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"raspberry"));

    // If you try to get the next value while at the end of the iterator, the return value is `None`
    assert_eq!(my_iterable_fav_fruits.next(), None);
}
