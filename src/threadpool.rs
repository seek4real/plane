/**
 * ThreadPool
 * 多线程支持
 */



use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

use super::work::Worker;
use super::message::Message;

// #[path = "work.rs"]
// mod work;
// #[path = "message.rs"]
// mod message;


// #[allow(unused)]
pub struct ThreadPool{
    workers: Vec<Worker>,
    pub sender: mpsc::Sender<Message>,
}



impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool 
    {
        assert!(size > 0);
        let mut workers: Vec<Worker> = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();

        let receiver_arc = Arc::new(Mutex::new(receiver));

        for index in 0..size {
            workers.push(Worker::new(index, Arc::clone(&receiver_arc)));
        }
        ThreadPool {
            workers,
            sender,
        }
    }

    pub fn execute<F>(&self, f:F)
    where F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &mut self.workers {
            let res = match self.sender.send(Message::ShutDown) {
                Ok(result) => result,
                Err(e) => println!("An Error Happened: {}", e.to_string()),
            };
            res
        }
        for worker in &mut self.workers {
            println!("shutdown worker {} ", worker.id);
            if let Some(t) = worker.handler.take() {
                t.join().unwrap();
            }
        }
    }
}

