// message


use super::work::Job;

pub enum Message
{
    NewJob(Job),
    ShutDown,
}


// impl Message{
//     pub fn call(&self)->&Message{
//         println!("quit!");
//         return self;
//     }
// }