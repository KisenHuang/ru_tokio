use tokio::net::TcpStream;
use std::error::Error;
use std::io::Write;
use tokio::io::{AsyncWriteExt, AsyncReadExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    stream.write_all(b"hello").await?;

    let mut buf = [0; 1024];
    let n = stream.read(&mut buf).await?;

    println!("GET {}", String::from_utf8_lossy(&buf[0..n]));

    Ok(())
}
