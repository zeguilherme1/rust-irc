use std::net::TcpStream;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:7878")
        .expect("Failed to connect");    
   
    println!("Sucessfully connected");
    
   
    loop {
        let mut input_str = String::new();
  
        io::stdin()
        .read_line(&mut input_str)
        .expect("Error reading input string");

       
        stream.write_all(input_str.as_bytes())?;
        
        if input_str == "exit" { break } ;

    }
    Ok(())
}

