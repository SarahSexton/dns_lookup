mod dns_lookup;
pub use crate::dns_lookup::*;
use std::net::IpAddr;
use cached::proc_macro::cached;
use cached::SizedCache;

// Cache for IP to hostname lookups
#[cached(
    name = "IP_TO_HOSTNAME_CACHE",
    ty = "SizedCache<IpAddr, String>",
    create = r#"{ SizedCache::with_size(100) }"#,
    result = true
)]
pub fn cached_ip_to_hostname(ip: IpAddr) -> Result<String, String> {
    ip_to_hostname(ip).map_err(|e| e.to_string())
}

// Cache for hostname to IP lookups
#[cached(
    name = "HOSTNAME_TO_IP_CACHE",
    ty = "SizedCache<String, Vec<IpAddr>>",
    create = r#"{ SizedCache::with_size(100) }"#,
    result = true
)]
pub fn cached_hostname_to_ip(hostname: String) -> Result<Vec<IpAddr>, String> {
    hostname_to_ip(&hostname).map_err(|e| e.to_string())
}

pub fn main() {
    // Example IP address
    let ip_address: IpAddr = "8.8.8.8".parse().unwrap();
    match cached_ip_to_hostname(ip_address) {
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
    match cached_hostname_to_ip(hostname.to_string()) {
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