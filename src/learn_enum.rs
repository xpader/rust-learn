use std::fmt::Display;

use crate::structs::IpAddr;

impl Display for IpAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            IpAddr::V4(ip) => write!(f, "v4 -> {}", ip),
            IpAddr::V6(ip) => write!(f, "v6 -> {}", ip)
        }
    }
}

pub fn learn_enum() {
    let ipv4 = IpAddr::V4("192.168.1.1".to_string());
    let ipv6 = IpAddr::V6("::1".to_string());

    if let IpAddr::V4(v) = &ipv4 {
        println!("This is a ipv4: {}", v);
    }

    match &ipv6 {
        IpAddr::V6(v) => println!("This is a ipv6: {}", v),
        _ => ()
    }

    let ip_list = vec![ipv4, ipv6];

    for ip in &ip_list {
        println!("{}", ip);
    }
}