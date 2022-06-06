use std::{collections::VecDeque, sync::{Arc, Mutex}};

use tokio::sync::mpsc::{channel, Sender, Receiver};
use crate::logger::LOGGER;

use super::{ProxyV4, ProxyType};
use std::time::{ SystemTime, Duration };
use colored::*;

pub struct Checker {
    pub queue: Arc<deadqueue::unlimited::Queue<ProxyV4>>,
    pub tx: Sender<ProxyV4>,
    pub rx: Arc<Mutex<Receiver<ProxyV4>>>,

    worker_count: usize,
}

impl Checker {
    pub fn new(worker_count: usize) -> Self {
        let (tx, rx) = channel(1024);
        let queue = Arc::new(deadqueue::unlimited::Queue::new());
        return Self {
            queue,
            tx,
            rx: Arc::new(Mutex::new(rx)),
            worker_count,
        };
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
        };
    }

    pub fn start(&self) {
        for _ in 0..self.worker_count {
            let tx = self.tx.clone();
            let queue = self.queue.clone();
            tokio::spawn(async move {
                loop {
                    let mut proxy = queue.pop().await;

                    let http = Checker::http(proxy.clone()).await;
                    let https = Checker::https(proxy.clone()).await;

                    if http {
                        proxy.proxy_type = ProxyType::HTTP;
                    } else if https {
                        proxy.proxy_type = ProxyType::HTTPS;
                    } else {
                        proxy.proxy_type = ProxyType::INVALID;
                    }

                    if http || https {
                        proxy.last_checked = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

                        if Checker::google(proxy.clone()).await {
                            proxy.google = true;
                        }
                    }

                    tx.send(proxy).await.unwrap();
                }
            }); 
        }
    }

    pub async fn http(mut proxy: ProxyV4) -> bool {
        let client = reqwest::ClientBuilder::new()
            .proxy(reqwest::Proxy::all(proxy.as_http()).unwrap())
            .user_agent(randua::new().desktop().to_string())
            .timeout(Duration::from_millis(2500))
            .build()
            .unwrap();

        return match client.get("https://api.ipify.org?format=json").send().await {
            Ok(response) => {
                if response.status().is_success() {
                    proxy.last_checked = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
                    true
                } else {
                    false
                }
            },
            Err(_) => {
                false
            }
        };
    }

    pub async fn https(mut proxy: ProxyV4) -> bool {
        let client = reqwest::ClientBuilder::new()
            .proxy(reqwest::Proxy::all(proxy.as_https()).unwrap())
            .user_agent(randua::new().desktop().to_string())
            .timeout(Duration::from_millis(2500))
            .build()
            .unwrap();

        return match client.get("https://api.ipify.org?format=json").send().await {
            Ok(response) => {
                if response.status().is_success() {
                    proxy.last_checked = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
                    true
                } else {
                    false
                }
            },
            Err(_) => {
                false
            }
        };
    }

    pub async fn google(mut proxy: ProxyV4) -> bool {
        let builder = reqwest::ClientBuilder::new()
            .proxy(reqwest::Proxy::all(proxy.as_https()).unwrap())
            .user_agent(randua::new().desktop().to_string())
            .timeout(Duration::from_millis(2500));

        let client = match proxy.proxy_type {
            ProxyType::HTTP => { 
                builder.proxy(reqwest::Proxy::all(proxy.as_http()).unwrap())
                    .build()
                    .unwrap()
            }
            ProxyType::HTTPS => {
                builder.proxy(reqwest::Proxy::all(proxy.as_https()).unwrap())
                    .build()
                    .unwrap()
            }
            _ => {
                panic!("Invalid proxy type passed to checker.");
            }
        };

        return match client.get("https://www.google.com/search?client=firefox-b-d&q=string").send().await {
            Ok(response) => {
                if response.status().is_success() && response.status().as_u16() != 409 {
                    proxy.last_checked = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
                    true
                } else {
                    false
                }
            },
            Err(_) => {
                false
            }
        };
    }
}
