mod dns_lookup;
pub use crate::dns_lookup::*;
use std::net::IpAddr;
use std::time::Instant;

fn main() {
    let total_start = Instant::now();

    for i in 0..100 {
        let start = Instant::now();

        // Example IP address
        let ip_address: IpAddr = "8.8.8.8".parse().unwrap();
        match ip_to_hostname(ip_address) {
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
        match hostname_to_ip(hostname) {
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

