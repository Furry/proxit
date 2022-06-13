use std::{fmt::Display, time::SystemTime, str::FromStr};
use serde::{ Serialize, Deserialize };

use crate::utils::time;
pub mod checker;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProxyType {
    HTTP,
    HTTPS,
    SOCKS4,
    SOCKS5,
    UNKNOWN,
    INVALID
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProxyAnonymity {
    #[serde(rename = "anonymous")]
    Anonymous,
    #[serde(rename = "transparent")]
    Transparent,
    #[serde(rename = "elite")]
    Elite
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyV4 {
    pub addr: [u8; 4],
    pub port: u16,
    pub found_at: u128,
    pub last_checked: u128,
    pub proxy_type: ProxyType,
    pub anonymity: ProxyAnonymity,
    pub google: bool,
    pub ping: u128
}

#[derive(Debug, Clone)]
pub struct ProxyV6 {
    addr: [u8; 16],
    port: u16,
    found_at: u64,
    last_checked: u64,
    proxy_type: ProxyType,
    anonymity: ProxyAnonymity,
    google: bool,
    speed: u32, // bps
    ping: u128

}

impl ProxyV4 {
    pub fn parse(proxy: &String) -> Option<ProxyV4> {
        // Split apart the proxy string
        let split = proxy.split(":").collect::<Vec<&str>>();
        let addr = split.get(0)?;
        let port = split.get(1)?;
        let port = port.parse::<u16>().unwrap();
        let addr = addr.split(".").map(|x| x.parse::<u8>().unwrap()).collect::<Vec<u8>>();
        return Some(Self {
            addr: [addr[0], addr[1], addr[2], addr[3]],
            port,
            found_at: time::now(),
            last_checked: 0,
            proxy_type: ProxyType::UNKNOWN,
            anonymity: ProxyAnonymity::Transparent,
            google: false,
            ping: 0,
        });
    }

    pub fn uri(&self, proxy_type: ProxyType) -> String {
        match proxy_type {
            ProxyType::HTTP => {
                format!("http://{}", self.to_string())
            },
            ProxyType::HTTPS => {
                format!("https://{}", self.to_string())
            },
            ProxyType::SOCKS4 => {
                format!("socks4://{}:{}", self.to_string(), self.port)
            },
            ProxyType::SOCKS5 => {
                format!("socks5://{}:{}", self.to_string(), self.port)
            },
            _ => {
                format!("{}:{}", self.to_string(), self.port)
            }
        }
    }

    pub fn as_http(&self) -> String {
        return format!("http://{}", self.to_string());
    }

    pub fn as_https(&self) -> String {
        return format!("https://{}", self.to_string());
    }
}

impl Display for ProxyV4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut addr = String::new();
        for i in 0..4 {
            addr.push_str(&format!("{}", self.addr[i]));
            if i != 3 {
                addr.push_str(".");
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

impl FromStr for ProxyAnonymity {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "anonymous" => Ok(ProxyAnonymity::Anonymous),
            "transparent" => Ok(ProxyAnonymity::Transparent),
            "elite" => Ok(ProxyAnonymity::Elite),
            _ => Err(())
        }
    }
}