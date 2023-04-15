use std::net::{TcpListener, TcpStream};

fn handle_request(stream: TcpStream){
    println!("Client request stuff");
}


fn main() -> std::io::Result<()> {
    println!("Starting server");
    let listener = TcpListener::bind("localhost:8080")?;

    for stream in listener.incoming() {
        handle_request(stream?);
    }

    Ok(())
}
