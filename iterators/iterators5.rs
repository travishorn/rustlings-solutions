// A simple model to track Rustlings exercise progress. Progress is modelled using a hash map. The
// name of the exercise is the key and the progress is the value. Four counting functions exist to
// count the number of exercises with a given progress. Two of the functions use imperative style
// `for` loops. The other two use iterators.

use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Progress {
    None,
    Some,
    Complete,
}

fn count_for(map: &HashMap<String, Progress>, value: Progress) -> usize {
    let mut count = 0;
    for val in map.values() {
        if val == &value {
            count += 1;
        }
    }
    count
}

fn count_iterator(map: &HashMap<String, Progress>, value: Progress) -> usize {
    // Starting with the reference to the hashmap,
    // Create an iterator over the values,
    // Filter them by comparing each to the given value,
    // Then count the filtered values.
    
    //Note: Creating a reference to `val` like this  is almost always necessary when working with
    // closures on iterators. Of course, you must dereference it before comparing.
    map.values().filter(|&val| *val == value).count()
}

fn count_collection_for(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    let mut count = 0;
    for map in collection {
        for val in map.values() {
            if val == &value {
                count += 1;
            }
        }
    }
    count
}

fn count_collection_iterator(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    // Starting with the collection,
    // Convert it to an iterator
    // Fold its values, starting at 0
    // Add the result of `count_iterator()` to the accumulator
    // Give `count_iterator()` a reference to the map and the given `Progress` we're looking for
    collection.iter().fold(0, |acc, map| acc + count_iterator(&map, value))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_complete() {
        let map = get_map();
        assert_eq!(3, count_iterator(&map, Progress::Complete));
    }

    #[test]
    fn count_equals_for() {
        let map = get_map();
        assert_eq!(
            count_for(&map, Progress::Complete),
            count_iterator(&map, Progress::Complete)
        );
    }

    #[test]
    fn count_collection_complete() {
        let collection = get_vec_map();
        assert_eq!(
            6,
            count_collection_iterator(&collection, Progress::Complete)
        );
    }

    #[test]
    fn count_collection_equals_for() {
        let collection = get_vec_map();
        assert_eq!(
            count_collection_for(&collection, Progress::Complete),
            count_collection_iterator(&collection, Progress::Complete)
        );
    }

    fn get_map() -> HashMap<String, Progress> {
        use Progress::*;

        let mut map = HashMap::new();
        map.insert(String::from("variables1"), Complete);
        map.insert(String::from("functions1"), Complete);
        map.insert(String::from("hashmap1"), Complete);
        map.insert(String::from("arc1"), Some);
        map.insert(String::from("as_ref_mut"), None);
        map.insert(String::from("from_str"), None);

        map
    }

    fn get_vec_map() -> Vec<HashMap<String, Progress>> {
        use Progress::*;

        let map = get_map();

        let mut other = HashMap::new();
        other.insert(String::from("variables2"), Complete);
        other.insert(String::from("functions2"), Complete);
        other.insert(String::from("if1"), Complete);
        other.insert(String::from("from_into"), None);
        other.insert(String::from("try_from_into"), None);

        vec![map, other]
    }
}
