// use std::net::IpAddr;

use anyhow::Result;


use crate::discovery::Discovery;
use crate::discovery::Device;

pub async fn run(timeout: u64) -> Result<()> {
    println!("{}", "Discovering devices on local network... ");
    println!();

    let discovery: Discovery = Discovery::new()?;
    let devices: Vec<Device> = discovery.discover(timeout).await?;

    if devices.is_empty(){
        println!("No devices found");
    }
    else {
        println!("Found {} device(s):", devices.len());
        println!();

        for(idx, device) in devices.iter().enumerate() {
            println!("{}. {}", idx + 1, device.name);
            println!("Hostname: {}", device.hostname);

            for &addr in &device.addresses {
                // let addr: &IpAddr = addr;
                println!("Address: {}:{}", addr.to_string(), device.port.to_string());
            }

            if !device.properties.is_empty(){
                println!(" Properties: ");
                for (key, value) in &device.properties {
                    println!("  {}: {}", key, value);
                }
            }
            println!();
        }
    }
    discovery.stop()?;
    Ok(())
}