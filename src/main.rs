use std::{net::{TcpListener, TcpStream}, io::{Read, Write}};
use std::str;

use crate::router::Router;

mod route_parser;
mod router;
mod html_parser;

fn handle_request(stream: &mut TcpStream, router: &mut Router){
    let mut buf = [0; 1024];
    stream.read(&mut buf).expect("Error reading TcpStream");
    let route = route_parser::parse_route(&buf);
    println!("{route}");
    let (response_string, content_string) = router.handle_route(route);
    stream.write(response_string.as_bytes());
    stream.write(content_string.as_bytes());
}


fn main() -> std::io::Result<()> {
    println!("Starting server");
    let listener = TcpListener::bind("localhost:8080")?;
    let mut router = Router::default();
    router.add_route(String::from("/"), Box::new(|| {"Hello from main route!".to_string()}));
    router.add_route(String::from("/test"), Box::new(|| {"And this is a test route!".to_string()}));
    router.add_route(String::from("/hello"), Box::new(|| {html_parser::parse_html("index.html")}));
    for stream in listener.incoming() {
        handle_request(&mut stream?, &mut router);
    }

    Ok(())
}
