// Building on the last exercise, all of the threads complete their work but this time the spawned
// threads are in charge of updating a shared value: JobStatus.jobs_completed

// In addition to `Arc`, bring `Mutex` (mutual exclusion) from the `sync` module into scope
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    // We cannot just give `JobStatus` directly to `Arc::new()`. We use `Mutex::new` to allow a
    // single thread access to some data at any given time.
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // Now we `lock()` the cloned status `Arc` so this thread can have access to mutate it
            // before incrementing the `jobs_completed` inside it.
            status_shared.lock().unwrap().jobs_completed += 1;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }

    // We `lock()` the status `Arc` again to access to it so we can give it to `println!`. This line
    // is outside the `for` loop above so that it is executed only after all handles have been
    // joined (to put it another way: after all threads are finished executing).
    println!("jobs completed {}", status.lock().unwrap().jobs_completed);
}
