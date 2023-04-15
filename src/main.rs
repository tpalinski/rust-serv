use std::{net::{TcpListener, TcpStream}, io::{Read, Write}};
use std::str;

mod route_parser;
mod router;

fn handle_request(stream: &mut TcpStream){
    println!("Client request stuff");
    let mut buf = [0; 1024];
    stream.read(&mut buf).expect("Error reading TcpStream");
    let route = route_parser::parse_route(&buf);
    println!("{route}");
    let response_string = "HTTP/1.1 200 OK\r
Content-Type: text/plain\r
Custom-Header: nice\r
\r\n"; 
    stream.write(response_string.as_bytes());
    let content_string = "Test text";
    stream.write(content_string.as_bytes());
}


fn main() -> std::io::Result<()> {
    println!("Starting server");
    let listener = TcpListener::bind("localhost:8080")?;

    for stream in listener.incoming() {
        handle_request(&mut stream?);
    }

    Ok(())
}
