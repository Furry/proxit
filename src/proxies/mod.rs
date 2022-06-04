use std::{fmt::Display, time::SystemTime};

pub mod checker;

#[derive(Debug, Clone)]
pub enum ProxyType {
    HTTP,
    HTTPS,
    SOCKS4,
    SOCKS5,
    UNKNOWN
}

#[derive(Debug, Clone)]
pub struct ProxyV4 {
    addr: [u8; 4],
    port: u16,
    found_at: u64,
    last_checked: u64,
    proxy_type: ProxyType,
    google: bool
}

#[derive(Debug, Clone)]
pub struct ProxyV6 {
    addr: [u8; 16],
    port: u16,
    found_at: u64,
    last_checked: u64,
    proxy_type: ProxyType,
    google: bool,
    speed: u32 // bps
}

impl ProxyV4 {
    pub fn parse(proxy: &String) -> Self {
        // Split apart the proxy string
        let split = proxy.split(":").collect::<Vec<&str>>();
        let addr = split.get(0).unwrap();
        let port = split.get(1).unwrap();
        let port = port.parse::<u16>().unwrap();
        let addr = addr.split(".").map(|x| x.parse::<u8>().unwrap()).collect::<Vec<u8>>();
        return Self {
            addr: [addr[0], addr[1], addr[2], addr[3]],
            port,
            found_at: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
            last_checked: 0,
            proxy_type: ProxyType::UNKNOWN,
            google: false
        };
    }
}

impl Display for ProxyV4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut addr = String::new();
        for i in 0..4 {
            addr.push_str(&format!("{}", self.addr[i]));
            if i != 3 {
                addr.push_str(":");
            }
        }
        write!(f, "{}:{}", addr, self.port)
    }
}

impl Display for ProxyV6 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut addr = String::new();
        for i in 0..16 {
            addr.push_str(&format!("{}", self.addr[i]));
            if i != 15 {
                addr.push_str(":");
            }
        }
        write!(f, "{}:{}", addr, self.port)
    }
}