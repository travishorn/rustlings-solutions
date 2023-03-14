// This program spawns multiple threads that each run for at least 250ms, and each thread returns
// how much time they took to complete. The program waits until all the spawned threads have
// finished and then collects their return values into a vector.

use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut handles = vec![];
    for i in 0..10 {
        handles.push(thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("thread {} is complete", i);
            start.elapsed().as_millis()
        }));
    }

    let mut results: Vec<u128> = vec![];
    for handle in handles {
        // `handles` is a collection of spawned threads. In each iteration of this `for` loop, we're
        // looking at one handle. We use `join()` to block the current thread until it completes and
        // then return the result of its execution. Since this result is wrapped in a `Result<T,
        // E>`, we use `unwrap()` to get the underlying value.
        results.push(handle.join().unwrap());
    }

    if results.len() != 10 {
        panic!("Oh no! All the spawned threads did not finish!");
    }
    
    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("thread {} took {}ms", i, result);
    }
}
