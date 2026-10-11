use std::net::TcpStream;
use std::io::{self, Write};

fn try_connection() -> io::Result<TcpStream> {
    // read input of the server addr and port
    // and try to connect to it

    let mut addr_input = String::new();
    let mut port_input = String::new();

    print!("> Digite o endereço IP: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut addr_input)?;

    print!("> Digite a porta: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut port_input)?;

    let address = format!(
        "{}:{}",
        addr_input.trim(),
        port_input.trim()
    );

    let stream = TcpStream::connect(address)?;
    Ok(stream)
}

fn get_nickname() -> String {
    // get username nickname
    
    let mut nickname = String::new();
    print!("> Digite seu nickname: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut nickname).expect("Falha ao ler nickname");
    
    nickname.trim().to_string()
}

fn get_user_message(nickname: &String) -> String {
    // gets user message by input
    // and concatenates with the user nickname

    print!("> ");
    io::stdout().flush().unwrap();
    let mut input_str = String::new();

    io::stdin().read_line(&mut input_str).expect("Falha ao ler mensagem");

    let message = format!(
            "[{}] says: {}",
            nickname,
            input_str
        );

    message.trim().to_string()
}

fn main() -> io::Result<()> {

    let mut stream = match try_connection() {
        Ok(stream) => {
            println!("Successfully connected!");
            stream
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
            return Err(e);
        }
    };

    let nickname = get_nickname();
    loop {
        let user_message = get_user_message(&nickname);

        stream.write_all(user_message.as_bytes())?;
        
        if user_message == "exit" { break } ;
    }
    Ok(())
}

