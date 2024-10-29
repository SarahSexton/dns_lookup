    use criterion::{black_box, criterion_group, criterion_main, Criterion};
    use std::net::IpAddr;
    use dns_lookup::{ip_to_hostname, hostname_to_ip};

    fn benchmark_ip_to_hostname(c: &mut Criterion) {
        let ip_address: IpAddr = "8.8.8.8".parse().unwrap();
        c.bench_function("ip_to_hostname", |b| {
            b.iter(|| ip_to_hostname(black_box(ip_address.clone())))
        });
    }

    fn benchmark_hostname_to_ip(c: &mut Criterion) {
        let hostname = "example.com";
        c.bench_function("hostname_to_ip", |b| {
            b.iter(|| hostname_to_ip(black_box(hostname)))
        });
    }

    criterion_group!(benches, benchmark_ip_to_hostname, benchmark_hostname_to_ip);
    criterion_main!(benches);
    