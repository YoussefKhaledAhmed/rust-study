use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("192.168.1.14:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}