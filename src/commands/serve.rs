use anyhow::Result;
use std::collections::HashMap;
use tokio::sync::mpsc;

use crate::discovery::Discovery;
use crate::protocol::{Message, MessageType};
use crate::transport::TcpTransport;

pub async fn run(port: u16, name: Option<String>) -> Result<()>{
    let device_name = name.unwrap_or_else(|| {
        hostname::get().unwrap().to_string_lossy().to_string()
    });
    println!("{}", "=== localComm Server ===");
    println!("Device Name: {}", device_name);
    println!("Port: {}", port.to_string());

    //Start TCP server
    let mut transport = TcpTransport::listen(port).await?;
    println!("Server listening on port {}", port);

    //Start mDNS advertising
    let discovery = Discovery::new()?;
    let mut properties = HashMap::new();
    properties.insert("name".to_string(), device_name.clone());
    properties.insert("version".to_string(), "1.0.0".to_string());

    discovery.advertise(&device_name, port, properties)?;
    println!("Advertising via mDNS");
    println!();

    println!("Waiting for msgs...");

    //Message receiver channel
    let (tx, mut rx) = mpsc::unbounded_channel();
    transport.start_receiver(tx)?;

    //handle incoming msgs
    while let Some((message, addr)) = rx.recv().await {
        let timestamp = chrono::Local::now().format("%H:%M:%S");

        match message.msg_type {
            MessageType::Text => {
                println!("[{}] {}: {}", timestamp.to_string(), message.from, message.content);
            }
            MessageType::File => {
                if let Some(filename) = message.metadata.get("filename") {
                    println!("[{}] {}: File - {}",timestamp.to_string(),message.from,filename)
                    }
            }
            MessageType::Heartbeat => {
                //Silent
            }
            _ => {
                 println!("[{}] Unknown message from {}",timestamp.to_string(),addr);
            }
        }
    }

    discovery.stop();
    Ok(())
}