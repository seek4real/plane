use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

#[allow(unused)]
pub struct Worker {
    id: usize,
    handler: thread::JoinHandle<()>
}

#[allow(unused)]
pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

//Box<FnOnce>

trait FnBox 
{
    fn call_box(self: Box<Self>);
}

impl <F: FnOnce()> FnBox for F 
{
    fn call_box(self: Box<F>)
    {
        (*self)();
    }
}

type Job = Box<dyn FnBox + Send + 'static>;

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
        self.sender.send(job).unwrap();
    }
}


impl Worker {
    fn new(id: usize, receiver:Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker 
    {
        let handler = thread::spawn(move || {
            while let Ok(job) = receiver.lock().unwrap().recv(){
                // println!("Worker {} got a job; executing.", id);
                job.call_box();
            }

        });

        Worker {
            id,
            handler,
        }
    }
}