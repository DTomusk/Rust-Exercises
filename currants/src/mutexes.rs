// mutex is a smart pointer that allows different threads to access and modify the same data
// rust ensures that only one thread is accessing a mutex at a time
// mutex comes with the risk of creating deadlocks
// Arc is the concurrent equivalent of Rc and mutex is the concurrent equivalent of RefCell (as it provides interior mutability)
use std::sync::{Arc, Mutex};
// Rc isn't thread safe
// use std::rc::Rc;
use std::thread;

pub fn meux() {
    let m = Mutex::new(String::from("Meow"));

    {
        // lock acquires the mutex, we can treat the value inside as a mutable reference
        let mut num = m.lock().unwrap();
        *num = String::from("Bark");
        // MutexGuard implements drop which releases the lock as soon as it goes out of scope
    }

    println!("m = {:?}", m);
}

pub fn shared() {
    // store the mutex in an Arc to allow for multiple ownership
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    // clone the mutex and alter it in the thread
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // lock guarantees that only this thread has access to the data now
            let mut num = counter.lock().unwrap();

            *num += 1;
            // lock gets released at the end of the scope
        });
        handles.push(handle);
    }

    // join all handles in the end to make sure all the threads have finished executing
    for handle in handles {
        handle.join().unwrap();
    }

    // the ok variant in the result returned by lock contains a reference to the value inside
    println!("Result: {}", *counter.lock().unwrap());
}
