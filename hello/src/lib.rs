use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

// ThreadPool contains thread JoinHandles
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

// use type alias to minimise repetition, Job will store a closure that implements FnOnce
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    // creating a ThreadPool with size number of threads
    pub fn new(size: usize) -> ThreadPool {
        // ensure valid number of threads
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        // preallocates size space for our vector
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // need to use arc because we can't simply give receiver to every worker
            // arc ensures all workers can own the receiver and only one can change it at a time
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        // pool contains sender
        ThreadPool { workers, sender }
    }

    // imitating the spawn function on thread
    pub fn execute<F>(&self, f: F)
    // thread running a request will only execute the request's closure once, so we want the Trait FnOnce
    // send is needed to transfer the closure from one thread to another
    // the thread has a static lifetime as we can't know how long it will take to execute
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

// private because external code doesn't need to know implementation details
// workers act as a go between between the threadpool and the actual JoinHandles
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            // call lock to get the mutex
            // call recv to get a job from the channel
            // unwrapping is rudimentary error handling (or lack thereof)
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {} got a job; executing", id);

            job();
        });

        Worker { id, thread }
    }
}