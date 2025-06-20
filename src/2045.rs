mod resp;
mod resp_result;

mod leetcode;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                tokio::spawn(handle_connection(stream));
            }
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        }
    }
}

async fn handle_connection(mut s: TcpStream) {
    let mut buf = [0; 512];

    loop {
        match s.read(&mut buf).await {
            Ok(sz) if sz != 0 => {
                println!("recved: {:?}", String::from_utf8_lossy(&buf[..sz]));

                let resp = "+PONG\r\n";
                if let Err(e) = s.write_all(resp.as_bytes()).await {
                    eprintln!("Error writing to socket: {}", e);
                }
            }
            Ok(_) => {
                println!("Connection closed");
                break;
            }
            Err(e) => {
                println!("error: {}", e);
                break;
            }
        }
    }
}
