use std::thread::{self, JoinHandle};
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};

pub type Job = Box<dyn FnOnce() + Send + 'static>;

pub type JobReceiver = Arc<Mutex<Receiver<Job>>>;


/// The worker represents a worker thread that executes jobs.
pub struct Worker {
    pub id: usize,
    pub handle: JoinHandle<()>,
}


impl Worker {

    /// Creates a new worker with specified ID.
    ///
    /// On creation, a new thread is spawned that has the Arc Mutex
    /// of the channel receiver and tries to receive jobs to execute.
    ///
    /// # Panics
    /// 
    /// The method panics if:
    ///     - Fails to spawn worker thread.
    ///     - The lock is poisoned.
    ///     - Fails to receive job from queue.
    ///
    /// # Returns
    ///
    /// A `Worker` storing its ID and handle to the worker thread.
    pub fn new(id: usize, receiver: JobReceiver) -> Self {
        let handle = thread::spawn(move || loop { 
            let job = receiver.lock()
                .expect("Failed to lock mutex")
                .recv()
                .expect("Failed to receive job from queue");
            job();
        });

        Self{ id, handle }
    } 

}
