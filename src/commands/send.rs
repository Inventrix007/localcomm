use anyhow::Result;

use crate::protocol::Message;
use crate::transport::TcpTransport;

pub async fn run(to: &str, message_text: &str) -> Result<()>{
    let hostname = hostname::get()?.to_string_lossy().to_string();

    let message = Message::text(hostname.clone(), to.to_string(), message_text.to_string());

    println!("Connecting to {}...", to);

    let mut stream = TcpTransport::connect(to).await?;
    TcpTransport::send_message(&mut stream, &message).await?;

    println!("Message sent successfully");
    
    Ok(())
}