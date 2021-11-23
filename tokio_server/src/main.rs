use tokio::io;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {

    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    println!("启动服务...");

    loop {
        let (mut socket, addr) = listener.accept().await?;

        println!("请求开始，访问地址 {}", addr);

        tokio::spawn(async move {
            let mut buf = [0; 1024];
            // In a loop, read data from the socket and write the data back.
            loop {
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                println!("接收数据 {:?}", String::from_utf8_lossy(&buf[0..n]));

                // Write the data back
                if let Err(e) = socket.write_all("嗯哼".as_bytes()).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }

            }
        });
    }
}