use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};

use crate::server::worker::{Job, Worker};

/// A structure representing a pool of worker threads that execute jobs.
/// The pool has a fixed size to not overwhelm the system from too many
/// requests.
#[allow(dead_code)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<Sender<Job>>,
}

impl ThreadPool {
    /// Creates a new `ThreadPool` with specified size.
    ///
    /// The size is the number of worker threads in the pool. On creation
    /// a channel is created
    ///
    /// # Panics
    ///
    /// If the size is zero.
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    /// Adds a closure to the queue of jobs, and will be executed by
    /// one of the worker threads in the pool.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender
            .as_ref()
            .expect("There is no sender.")
            .send(job)
            .expect("Execution of job failed.");
    }
}

impl Default for ThreadPool {
    #[inline]
    fn default() -> Self {
        ThreadPool::new(1)
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Worker {} is shutting down", worker.id);
            if let Some(handle) = worker.handle.take() {
                handle.join().expect("Failed to join thread handle");
            }
        }
    }
}
