// main program

// mod message;
mod thread;

// use message::Message;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;
use thread::ThreadPool;

fn main()
{
    println!("Start Up");
    // let write = Message::Write("tttttt".to_string());
    // write.call();

    let listener = TcpListener::bind("127.0.0.1:9999").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming(){
        println!("Connection Established.");
        let s = stream.unwrap();
        pool.execute(|| {

            handle_connection(s);
        })
    }
}

fn handle_connection(mut stream: TcpStream)
{
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let contents = fs::read_to_string("/Users/yuhaibo/live/rust/templates/index.html").unwrap();

    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}