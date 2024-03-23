use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("172.26.40.197:8080").await?;

    stream.write_all(b"hello world!").await?;

    Ok(())
}
