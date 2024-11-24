/*a simple static web server*/

use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {

    const HOST: &str = "localhost";
    const PORT: &str = "1234";
    let conn_str: String = HOST.to_owned() + ":" + PORT;

    let listener = TcpListener::bind(conn_str.clone()).await.unwrap();
    println!("Server listening on {}", conn_str.clone());

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            let mut buf = [0; 1024];

            // Read data from the client
            match socket.read(&mut buf).await {
                Ok(0) => return, // Connection closed
                Ok(n) => {
                    // Echo the data back to the client
                    if socket.write_all(&buf[..n]).await.is_ok() {
                        println!("Echoed {} bytes", n);
                    }
                }
                Err(e) => eprintln!("Failed to read from socket: {:?}", e),
            }
        });
    }
}