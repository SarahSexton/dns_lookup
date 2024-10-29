mod dns_lookup;
pub use crate::dns_lookup::*;
use std::net::IpAddr;
use std::time::Instant;
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

fn main() {
    let total_start = Instant::now();

    for i in 0..100 {
        let start = Instant::now();

        // Example IP address
        let ip_address: IpAddr = "8.8.8.8".parse().unwrap();
        match cached_ip_to_hostname(ip_address) {
            Ok(hostname) => {
                println!("Iteration {}: let ip_address: IpAddr = {:?}", i, ip_address);
                println!("Iteration {}: Hostname: {}", i, hostname);
            }
            Err(e) => {
                eprintln!("Iteration {}: {}", i, e);
            }
        }

        // Example hostname
        let hostname = "example.com";
        match cached_hostname_to_ip(hostname.to_string()) {
            Ok(ips) => {
                for ip_address in ips {
                    println!("Iteration {}: let hostname = {}", i, hostname);
                    println!("Iteration {}: IP address: {}", i, ip_address);
                }
            }
            Err(e) => {
                eprintln!("Iteration {}: {}", i, e);
            }
        }

        let duration = start.elapsed();
        println!("Iteration {}: Time taken: {:?}", i, duration);
    }

    let total_duration = total_start.elapsed();
    println!("Total time taken for 100 iterations: {:?}", total_duration);
}
