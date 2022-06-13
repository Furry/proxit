use std::sync::{Arc, Mutex};
use crate::utils::time;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use std::time::Duration;
use super::{ProxyAnonymity, ProxyType, ProxyV4};
use anyhow::Result;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};


lazy_static! {
    pub static ref CHECK_DOMAIN: String =
        "http://naminginprogress.com/api/proxies/transparency".to_string();
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NamingInProgressResponse {
    at: u64,
    level: ProxyAnonymity,
}

pub enum NetCheckType {
    Socks5,
    Socks4,
    Https,
    Http,
}

pub struct Checker {
    pub queue: Arc<deadqueue::unlimited::Queue<ProxyV4>>,
    pub tx: Sender<ProxyV4>,
    pub rx: Arc<Mutex<Receiver<ProxyV4>>>,

    worker_count: usize,
}

impl Checker {
    pub async fn new(worker_count: usize) -> Self {
        let (tx, rx) = channel(1024);
        let queue = Arc::new(deadqueue::unlimited::Queue::new());
        let checker = Self {
            queue,
            tx,
            rx: Arc::new(Mutex::new(rx)),
            worker_count,
        };

        // The checker is created, now it's just a matter of starting the workers.
        // TODO: Make checker.start syncronous.
        checker.start().await;
        return checker;
    }

    pub fn is_idle(&self) -> bool {
        return self.queue.len() == 0;
    }

    pub fn get_reciever(self) -> Arc<Mutex<Receiver<ProxyV4>>> {
        return self.rx.clone();
    }

    pub fn get_sender(self) -> Sender<ProxyV4> {
        return self.tx.clone();
    }

    pub fn add(&self, proxies: Vec<ProxyV4>) {
        for proxy in proxies {
            self.queue.push(proxy);
        }
    }

    pub async fn get_ping_offset() -> u128 {
        let then = time::now();
        match reqwest::get(CHECK_DOMAIN.clone()).await {
            Ok(_) => time::now() - then,
            _ => 250,
        }
    }

    async fn check(&self, proxy: ProxyV4, ping: u128) -> ProxyV4 {
        let mut proxy = proxy;
        if Checker::net(&mut proxy, ping.clone(), NetCheckType::Http)
        .await
        .is_err()
            {
                if Checker::net(&mut proxy, ping.clone(), NetCheckType::Https)
                    .await
                    .is_err()
                {
                    if Checker::net(&mut proxy, ping.clone(), NetCheckType::Socks4)
                        .await
                        .is_err()
                    {
                        Checker::net(&mut proxy, ping.clone(), NetCheckType::Socks5)
                            .await
                            .ok();
                    }
                };
            }

        Checker::google(&mut proxy).await.ok();

        return proxy;
    }

    async fn start(&self) {
        let oping = Checker::get_ping_offset().await;
        for _ in 0..self.worker_count {
            let tx = self.tx.clone();
            let queue = self.queue.clone();
            let ping = oping.clone();
            tokio::spawn(async move {
                loop {
                    let mut proxy = queue.pop().await;

                    if Checker::net(&mut proxy, ping.clone(), NetCheckType::Http)
                        .await
                        .is_err()
                    {
                        if Checker::net(&mut proxy, ping.clone(), NetCheckType::Https)
                            .await
                            .is_err()
                        {
                            if Checker::net(&mut proxy, ping.clone(), NetCheckType::Socks4)
                                .await
                                .is_err()
                            {
                                Checker::net(&mut proxy, ping.clone(), NetCheckType::Socks5)
                                    .await
                                    .ok();
                            }
                        };
                    }

                    Checker::google(&mut proxy).await.ok();
                    tx.send(proxy).await.unwrap();
                }
            });
        }
    }

    async fn net(
        proxy: &mut ProxyV4,
        server_ping: u128,
        check_type: NetCheckType,
    ) -> Result<(), anyhow::Error> {
        let client = {
            let client = reqwest::ClientBuilder::new();
            match check_type {
                NetCheckType::Https => {
                    client.proxy(reqwest::Proxy::all(proxy.uri(ProxyType::HTTP)).unwrap())
                }
                NetCheckType::Http => {
                    client.proxy(reqwest::Proxy::all(proxy.uri(ProxyType::HTTPS)).unwrap())
                }
                NetCheckType::Socks4 => {
                    client.proxy(reqwest::Proxy::all(proxy.uri(ProxyType::SOCKS4)).unwrap())
                }
                NetCheckType::Socks5 => {
                    client.proxy(reqwest::Proxy::all(proxy.uri(ProxyType::SOCKS5)).unwrap())
                }
            }
            .user_agent(randua::new().desktop().to_string())
            .timeout(Duration::from_millis(2500))
            .build()
            .unwrap()
        };

        let then = time::now();
        let response = client.get(CHECK_DOMAIN.clone()).send().await?;

        let text = response.text().await?;

        let body = serde_json::from_str::<NamingInProgressResponse>(text.as_str())?;

        proxy.last_checked = time::now();
        proxy.anonymity = body.level;
        proxy.ping = time::now() - then - server_ping;

        proxy.proxy_type = match check_type {
            NetCheckType::Socks5 => ProxyType::SOCKS5,
            NetCheckType::Socks4 => ProxyType::SOCKS4,
            NetCheckType::Https => ProxyType::HTTPS,
            NetCheckType::Http => ProxyType::HTTP,
        };

        Ok(())
    }

    async fn google(proxy: &mut ProxyV4) -> Result<(), anyhow::Error> {
        let builder = reqwest::ClientBuilder::new()
            .proxy(reqwest::Proxy::all(proxy.as_https()).unwrap())
            .user_agent(randua::new().desktop().to_string())
            .timeout(Duration::from_millis(2500));

        let client_ = match proxy.proxy_type {
            ProxyType::HTTP => {
                Ok(builder.proxy(reqwest::Proxy::all(proxy.uri(ProxyType::HTTP)).unwrap()))
            }
            ProxyType::HTTPS => {
                Ok(builder.proxy(reqwest::Proxy::all(proxy.uri(ProxyType::HTTPS)).unwrap()))
            }
            ProxyType::SOCKS4 => {
                Ok(builder.proxy(reqwest::Proxy::all(proxy.uri(ProxyType::SOCKS4)).unwrap()))
            }
            ProxyType::SOCKS5 => {
                Ok(builder.proxy(reqwest::Proxy::all(proxy.uri(ProxyType::SOCKS5)).unwrap()))
            }
            _ => Err(()),
        };

        if client_.is_err() {
            return Ok(());
        }

        let client = client_.unwrap().build().unwrap();

        match client
            .get("https://www.google.com/search?client=firefox-b-d&q=string")
            .send()
            .await
        {
            Ok(response) => {
                if response.status().is_success() && response.status().as_u16() != 409 {
                    proxy.last_checked = time::now();
                    proxy.google = true;
                }
            }
            Err(_) => (),
        };

        return Ok(());
    }
}
