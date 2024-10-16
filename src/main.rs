mod dns_lookup;
pub use crate::dns_lookup::*;
use std::net::IpAddr;
use std::io::{self, Write};

fn main() {
    let mut input = String::new();

    // Ask the user for an IP address or hostname
    println!("Enter an IP address or a hostname:");

    // Read the user input
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    // Try to parse the input as an IP address
    if let Ok(ip_address) = input.parse::<IpAddr>() {
        match ip_to_hostname(ip_address) {
            Ok(hostname) => {
                println!("let ip_address: IpAddr = {:?}", ip_address);
                println!("Hostname: {}", hostname);
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    } else {
        // Treat the input as a hostname
        let hostname = input;
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
}