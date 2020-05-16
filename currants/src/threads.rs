// independently running parts of a program are called threads
// threads can increase speed, but they also increase complexity
// there's no guarantee of what order your code will execute in
// threads can lead to problems such as:
// race conditions, threads accessing and modifying the same data at the same time
// deadlocks, threads waiting for each other to finish
// other bugs

// rust only implements 1:1 threading meaning one operting system thread per language thread

use std::thread;
use std::time::Duration;

pub fn throds() {
    let v = vec![1,2,7,3];

    // the code we want the thread to run is passed in via a closure
    let handle = thread::spawn(|| {
        for i in 0..20 {
            println!("Spawned thread says: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // use move to give ownership of the vector to this thread
    let hundle = thread::spawn(move || {
        println!("We own this vector now: {:?}", v);
    });

    // once the main thread has finished executing so will the spawned thread, regardless of how far it is
    for i in 0..17 {
        println!("Main thread says: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    // join blocks the current thread until the handle thread has finished executing
    handle.join().unwrap();
    hundle.join().unwrap();
}
