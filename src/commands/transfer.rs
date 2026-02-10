use anyhow::{Ok, Result};
use clap::Subcommand;
use indicatif::{ProgressBar, ProgressStyle};
use sha2::{Digest, Sha256};
use core::hash;
use std::path::Path;
use tokio::fs::File;
use tokio::io::{AsyncReadExt,AsyncWriteExt};

use crate::cli::TransferCommands;
use crate::transport::TcpTransport;

pub async fn run(subcommand: TransferCommands) -> Result<()>{
    match subcommand {
        TransferCommands::Send { file, to } => send_file(&file, &to).await,
        TransferCommands::Receive { port, output } => receive_file(port, &output).await,
    }    
}

async fn send_file(file_path: &str, to: &str) -> Result<()> {
    let path = Path::new(file_path);
    if !path.exists() {
        anyhow::bail!("File not found: {}", file_path);
    }

    let mut file = File::open(path).await?;
    let file_size = file.metadata().await?.len();
    let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("unknown");
    println!("Connecting to: {}", to);

    let mut stream = TcpTransport::connect(to).await?;
    //Send metadata
    let metadata = format!("{}\n{}\n", file_name, file_size);
    stream.write_all(metadata.as_bytes()).await?;
    println!("Sending file");

    //progress bar
    // let pb = ProgressBar::new(file_size);
    // pb.set_style( ProgressStyle::default_bar().template("{spinner:.green} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")?
    //         .progress_chars("#>-"),);

    let mut buffer = vec![0u8; 64 * 1024]; //64 kb chunks
    let mut hasher = Sha256::new();

    loop {
        let n = file.read(&mut buffer).await?;
        if n == 0 {
            break;
        }

        stream.write_all(&buffer[..n]).await?;
        hasher.update(&buffer[..n]);
        // pb.inc(n as u64);
    }
    // pb.finish_with_message("Done");

    let checksum = format!("{:x}", hasher.finalize());
    println!("file sent success");
    println!("checksum: {} ", checksum);

    Ok(())
}

async fn receive_file(port: u16, output_dir: &str) -> Result<()>{
    println!("Waiting for file on port {}", port);

    let mut transport = TcpTransport::listen(port).await?;
    let (mut stream, addr) = transport.accept().await?;

    println!("Connection from {}", addr);
    //Read metadata
    let mut temp = [0u8; 1];
    let mut lines = Vec::new();
    let mut current_line = Vec::new();

    for _ in 0..2 {
        loop{
            stream.read_exact(&mut temp).await?;
            if temp[0] == b'\n' {
                lines.push(String::from_utf8(current_line.clone())?);
                current_line.clear();
                break;
            }
            current_line.push(temp[0]);
        }
    }
    let file_name = &lines[0];
    let file_size: u64 = lines[1].parse()?;

    let output_path = Path::new(output_dir).join(file_name);
    let mut output_file = File::create(&output_path).await?;

    println!("Receiving: {}", file_name);

    // Progress bar
    // let pb = ProgressBar::new(file_size);
    // pb.set_style(
    //     ProgressStyle::default_bar()
    //         .template("{spinner:.green} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")?
    //         .progress_chars("#>-"),
    // );

    // Receive file
    let mut buffer = vec![0u8; 64 * 1024];
    let mut hasher = Sha256::new();
    let mut received = 0u64;

    while received < file_size {
        let to_read = std::cmp::min(buffer.len() as u64, file_size - received) as usize;
        let n = stream.read(&mut buffer[..to_read]).await?;
        
        if n == 0 {
            break;
        }

        output_file.write_all(&buffer[..n]).await?;
        hasher.update(&buffer[..n]);
        received += n as u64;
        // pb.inc(n as u64);
    }

    // pb.finish_with_message("Done");

    let checksum = format!("{:x}", hasher.finalize());
    println!("File received: {}", output_path.display());
    println!("   Checksum: {}", checksum);

    Ok(())
}