use dns_lookup::{lookup_addr, lookup_host};
use std::net::IpAddr;

/// Performs a reverse DNS lookup to get the hostname from an IP address.
pub fn ip_to_hostname(ip: IpAddr) -> Result<String, String> {
    match lookup_addr(&ip) {
        Ok(hostname) => Ok(hostname),
        Err(e) => Err(format!("Failed to perform reverse DNS lookup: {}", e)),
    }
}

/// Performs a DNS lookup to get the IP addresses from a hostname.
pub fn hostname_to_ip(hostname: &str) -> Result<Vec<IpAddr>, String> {
    match lookup_host(hostname) {
        Ok(ips) => Ok(ips),
        Err(e) => Err(format!("Failed to lookup host: {}", e)),
    }
}