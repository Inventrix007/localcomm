use anyhow::Result;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio_util::bytes::Buf;

use crate::protocol::{Message, message};

pub struct  TcpTransport {
    listener: Option<TcpListener>,
}

impl TcpTransport {
    pub fn new() -> Self {
        Self { listener: None }
    }

    pub async fn listen(port: u16) -> Result<Self> {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
        Ok(Self {
            listener: Some(listener)
        })
    }

    pub async fn accept(&mut self) -> Result<(TcpStream, std::net::SocketAddr)> {
        if let Some(listener) = &self.listener {
            Ok(listener.accept().await?)
        }
        else {
            Err(anyhow::anyhow!("Transport not listening"))
        }
    }

    pub async fn connect(address: &str) -> Result<TcpStream> {
        Ok(TcpStream::connect(address).await?)
    }

    pub async fn send_message(stream: &mut TcpStream, message: &Message) -> Result<()> {
        let json = message.to_json()?;
        stream.write_all(json.as_bytes()).await?;
        stream.write_all(b"\n").await?;
        stream.flush().await?;
        Ok(())
    }

    pub async fn receive_message(stream: &mut TcpStream) -> Result<Message> {
        let mut reader = BufReader::new(stream);
        let mut line = String::new();
        reader.read_line(&mut line).await?;
        Message::from_json(&line)
    }

    pub fn start_receiver(&mut self, tx: mpsc::UnboundedSender<(Message, std::net::SocketAddr)>,) -> Result<()>{
        if let Some(listener) = self.listener.take(){
            tokio::spawn(async move {
                loop {
                    match listener.accept().await {
                        Ok((mut stream, addr)) => {
                            let tx = tx.clone();
                            tokio::spawn(async move {
                                let mut reader = BufReader::new(&mut stream);
                                let mut line = String::new();
                                while let Ok(n) = reader.read_line(&mut line).await {
                                    if n == 0 {
                                        break;
                                    }
                                    if let Ok(message) = Message::from_json(&line){
                                        let _ = tx.send((message, addr));
                                    }
                                    line.clear();
                                }
                            });
                        }
                        Err(e) => {
                            eprintln!("Accept error: {}", e);
                        }
                    }
                }
            });
            Ok(())
        }else {
            Err(anyhow::anyhow!("Transport not listening 2"))
        }
    }
}
