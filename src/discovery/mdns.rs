use anyhow::Result;
use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo, TxtProperties};
use std::collections::HashMap;
use std::net::IpAddr;
use std::time::Duration;
use tokio::time::{Timeout, sleep};

pub const SERVICE_TYPE: &str = "_localcomm._tcp.local.";

#[derive(Debug, Clone)]
pub struct Device {
    pub name: String,
    pub hostname: String,
    pub addresses: Vec<IpAddr>,
    pub port: u16,
    pub properties: HashMap<String, String>,
}

pub struct Discovery {
    daemon: ServiceDaemon,
}

impl Discovery {
    pub fn new() -> Result<Self>{
        let daemon = ServiceDaemon::new()?;
        Ok(Self { daemon })
    }

    pub fn advertise(&self, name: &str, port: u16, properties: HashMap<String, String>) -> Result<()>{
        let service_info = ServiceInfo::new(
            SERVICE_TYPE,
            name,
            &format!("{}.local.", name),
            "",
            port,
            properties,
        )?;
        
        self.daemon.register(service_info)?;
        Ok(())
    }

    pub async fn discover(&self, timeout_secs: u64) -> Result<Vec<Device>>{
        let receiver = self.daemon.browse(SERVICE_TYPE)?;
        let mut devices = Vec::new();

        let timeout = Duration::from_secs(timeout_secs);
        let start = std::time::Instant::now();
        
        loop {
            if start.elapsed() >= timeout {
                break;
            }

            //Try to receive with a small timeout match
            match tokio::time::timeout(Duration::from_millis(100), async {
                receiver.recv_async().await
            })
            .await {
                Ok(Ok(event)) => match event {
                    ServiceEvent::ServiceResolved(info) => {
                        let device = Device {
                            name: info.get_fullname().to_string(),
                            hostname: info.get_hostname().to_string(),
                            addresses: info.get_addresses().iter().map(|scoped| scoped.to_ip_addr()).collect(),
                            port: info.get_port(),
                            properties: info.get_properties().clone().into_property_map_str(),
                        };
                        devices.push(device);
                    }
                    _ => {}
                },
                Ok(Err(_)) => break,
                Err(_) => {
                    //Timeout, continue loop
                    sleep(Duration::from_millis(100)).await;
                }
            }
        }
        Ok(devices)
    }
     pub fn stop(&self) -> Result<()>{
                self.daemon.shutdown();
                Ok(())
            }
}

impl Drop for Discovery{
        fn drop( &mut self){
            let _ = self.daemon.shutdown();
        }
    }