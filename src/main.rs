mod dns_lookup;
pub use crate::dns_lookup::*;
use std::net::IpAddr;

fn main() {
    // Example IP address
    let ip_address: IpAddr = "8.8.8.8".parse().unwrap();
    match ip_to_hostname(ip_address) {
        Ok(hostname) => {
            println!("let ip_address: IpAddr = {:?}", ip_address);
            println!("Hostname: {}", hostname);
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }

    // Example hostname
    let hostname = "example.com";
    match hostname_to_ip(hostname) {
        Ok(ips) => {
            for ip_address in ips {
                println!("let hostname = {}", hostname);
                println!("IP address: {}", ip_address);
            }
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}