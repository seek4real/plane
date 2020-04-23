// main program

// mod message;
mod thread;
mod request;

// use message::Message;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;
use thread::ThreadPool;
use request::header::RequestHeader;

fn main()
{
    println!("Start Up");
    // let write = Message::Write("tttttt".to_string());
    // write.call();

    let listener = TcpListener::bind("127.0.0.1:9999").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming(){
        // println!("Connection Established.");
        let s = stream.unwrap();
        pool.execute(|| {

            handle_connection(s);
        })
    }
}

fn handle_connection(mut stream: TcpStream)
{
    let mut buffer = [0; 512];
    stream.read(&mut buffer).expect("read TcpStream error."); //这里还可以使用unrawp()

    let file_content_path: String = "templates/index.html".to_string();
    let contents = match fs::read_to_string(file_content_path) {
        Ok(s) => s,
        Err(e) => {
            println!("Error:{:?}", e.to_string());
            fs::read_to_string("templates/404.html").unwrap()
        },
    };

    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    // let mut req_buf = String::new();
    // let mut reader = std::io::BufReader::new(&buffer[..]);
    // reader.read_line(&mut req_buf).expect("not found.");
    // println!("Request line: {:?}", req_buf);
    // assert_eq!(req_buf, "GET /hi HTTP/1.1\r\n");
    let request = String::from_utf8_lossy(&buffer[..]);
    let mut req_iter = request.lines();
    // println!("req iterator: {:?}", req_iter.next().unwrap());
    // println!("req iterator: {:?}", req_iter.next().unwrap());
    // println!("req iterator: {:?}", req_iter.next().unwrap());

    let str_header = req_iter.next().unwrap();
    let req_header = RequestHeader::translate(str_header);
    println!("Request:{} ", req_header);

    if req_header.eq_path(String::from("/hi")) {
        println!("say hi");
    } else if req_header.eq_path(String::from("/favicon")) {
        println!("favicon request");
    } else {
        println!("request else");
    }
}

