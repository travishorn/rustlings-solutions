// Here we are given a Vec of u32 called "numbers" with values ranging from 0 to 99 -- [ 0, 1, 2,
// ..., 98, 99 ] We use this set of numbers within 8 different threads simultaneously. Each thread
// gets the sum of every eighth value, with an offset.

// The first thread (offset 0), will sum 0, 8, 16, ...
// The second thread (offset 1), will sum 1, 9, 17, ...
// The third thread (offset 2), will sum 2, 10, 18, ...
// ...
// The eighth thread (offset 7), will sum 7, 15, 23, ...

// Because we are using threads, our values need to be thread-safe.  Therefore, we are using Arc.

#![forbid(unused_imports)] // Do not change this, (or the next) line.
use std::sync::Arc;
use std::thread;

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();

    // Create an `Arc` pointer to share ownership of the `numbers` vector between multiple threads
    let shared_numbers = Arc::new(numbers);
    let mut joinhandles = Vec::new();

    for offset in 0..8 {
        // For each thread, use `clone()` to create a new `Arc` pointer that points to the same
        // value. We must clone a **reference** to `shared_numbers` here. Otherwise, each thread
        // would have its own copy of the data, defeating the purpose of using an `Arc` pointer to
        // share ownership between multiple threads.
        let child_numbers = Arc::clone(&shared_numbers);
        joinhandles.push(thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|n| *n % 8 == offset).sum();
            println!("Sum of offset {} is {}", offset, sum);
        }));
    }
    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
}
