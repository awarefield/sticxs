use std::sync::Arc;
use tokio::io::{self, AsyncReadExt};
use tokio::net::{TcpListener, TcpStream};

pub struct Server {
    pub addr: String,
}

impl Server {
    pub fn new(addr: String) -> Arc<Self> {
        Arc::new(Self { addr })
    }

    pub async fn run(self: Arc<Self>) {
        let listener = TcpListener::bind(&self.addr).await.unwrap();
        println!("Listening on address: {}", self.addr);

        loop {
            let (socket, _addr) = match listener.accept().await {
                Ok(connection) => connection,
                Err(e) => {
                    eprintln!("Failed to accept connection: {:?}", e);
                    continue;
                }
            };
            let server_clone = self.clone();
            tokio::spawn(async move {
                if let Err(e) = server_clone.handle_connection(socket).await {
                    eprintln!("Error handling connection: {:?}", e);
                }
            });
        }
    }

    async fn handle_connection(self: Arc<Self>, mut socket: TcpStream) -> io::Result<()> {
        let mut buffer = Vec::new();
        loop {
            let mut tmp_buffer = vec![0; 1024];
            let n = match socket.read(&mut tmp_buffer).await {
                Ok(n) => n,
                Err(e) => {
                    eprintln!("Error reading from socket: {:?}", e);
                    break;
                }
            };
            if n == 0 {
                break; // Connection closed by the client
            }
            buffer.extend_from_slice(&tmp_buffer[..n]);
        }
        println!("{}", String::from_utf8_lossy(&buffer));
        Ok(())
    }
}
