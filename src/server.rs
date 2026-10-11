use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn handle_stream(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                let addr = stream.peer_addr().unwrap();
                println!("Client {} disconnected!", addr);
                break;
            }
            Ok(n) => {
                println!("{}", String::from_utf8_lossy(&buffer[..n])); 
                stream.write_all(&buffer[..n]).unwrap();
                buffer = [0; 512];
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;

    println!("Server listening on port 7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let addr = stream.peer_addr().unwrap();
                println!("Client connected: {}", addr);
                
                std::thread::spawn(move || handle_stream(stream));
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}

