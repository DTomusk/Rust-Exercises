use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

// ThreadPool contains thread JoinHandles
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>
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

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

// behaviour when threadpool goes out of scope
impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending termiante message to all workers.");

        // workers need to be told to stop what they're doing or they'll just pool forever
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers");

        // drop each worker individually by calling join on their thread
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            // take gets ownership of the value in the some variant and leaves a none in its place
            // if however take() returns a none we don't have to do anything because that thread has already been cleaned up
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

// private because external code doesn't need to know implementation details
// workers act as a go between between the threadpool and the actual JoinHandles
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            // call lock to get the mutex
            // call recv to get a job from the channel
            // unwrapping is rudimentary error handling (or lack thereof)
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing", id);

                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate", id);

                    break;
                }
            }
        });

        Worker { id, thread: Some(thread) }
    }
}

enum Message {
    NewJob(Job),
    Terminate
}
