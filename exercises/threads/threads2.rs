// threads2.rs
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a hint.
// Building on the last exercise, we want all of the threads to complete their work but this time
// the spawned threads need to be in charge of updating a shared value: JobStatus.jobs_completed

// reference to https://github.com/akhildevelops/2022-rustlings-solutions/blob/main/exercises/threads/threads2.rs
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = status.clone();
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // TODO: You must take an action before you update a shared value
            // get lock before data using
            let mut s_shard = status_shared.lock().unwrap();
            s_shard.jobs_completed += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        
        // TODO: Print the value of the JobStatus.jobs_completed. Did you notice anything
        // interesting in the output? Do you have to 'join' on all the handles?
        // unlock after data using
        println!("jobs completed {}", status.lock().unwrap().jobs_completed);
        handle.join().unwrap();
    }
}
