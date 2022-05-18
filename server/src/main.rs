use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener: TcpListener = TcpListener::bind("localhost:7878").unwrap();

    for stream in listener.incoming(){
        let stream: TcpStream = stream.unwrap();
        
        println!("Connection established!");
    }
}
