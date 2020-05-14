// work
/**
 * 
 */


 
use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

use super::message::Message;

#[allow(unused)]
pub struct Worker {
    pub id: usize,
    pub handler: Option<thread::JoinHandle<()>>
}



impl Worker {
    pub fn new(id: usize, receiver:Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker 
    {
        let handler = thread::spawn(move || {
            // while let Ok(msg) = receiver.lock().unwrap().recv(){
                
            //     match msg {
            //         Message::NewJob(job) => {
            //             println!("Worker {} got a job; executing.", id);
            //             job();
            //         },
            //         Message::ShutDown => {
            //             println!("Worker {} got Shutdown sign . ", id);
            //             break;
            //         },
            //     }
            // }

            loop {
                let msg = receiver.lock().unwrap().recv().unwrap();
                match msg {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);
                        job();
                    },
                    Message::ShutDown => {
                        println!("Worker {} got Shutdown sign . ", id);
                        break;
                    },
                }
            }
            println!("Thread is dead. ID is {}", id);
        });

        Worker {
            id,
            handler: Some(handler),
        }
    }
}

/**
 * 定义Job 包装Job
 */
//Box<FnOnce>

// pub trait FnBox 
// {
//     fn call_box(self: Box<Self>);
// }

// impl <F: FnOnce()> FnBox for F 
// {
//     fn call_box(self: Box<F>)
//     {
//         (*self)();
//     }
// }

pub type Job = Box<dyn FnOnce() + Send + 'static>;
