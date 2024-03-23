use tokio::net::{TcpListener, TcpStream};
use tokio::io::AsyncReadExt;

use std::{str, io};

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("172.26.40.197:8080").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        println!("I recieved I connection");
        process_socket(socket).await?;
    }
}

async fn process_socket(mut socket: TcpStream) -> io::Result<()>{
    let mut buff: [u8; 1024] = [0; 1024];
    let bytes_read = socket.read(&mut buff).await?;

    if bytes_read == 0 {
        eprintln!("Recieved an empty request!");
    }

    let client_message = match str::from_utf8(&buff) {
        Ok(s) => s,
        Err(error) => panic!("Invalid utf-8 sequence. {}", error),
    };

    match client_message.split_once(" ")  {
        Some((command, rest)) => println!("I got the command: {}\nAlso got the rest: {}", command, rest),
        None => println!("There wasn't a space, got the empty command {}", client_message)
    }


    Ok(()) 
}

