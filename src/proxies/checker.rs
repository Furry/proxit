use std::{collections::VecDeque, sync::{Arc, Mutex}};

use tokio::sync::mpsc::{channel, Sender, Receiver};
use crate::logger::LOGGER;

use super::ProxyV4;
use std::time::SystemTime;
use colored::*;

pub struct Checker {
    pub queue: Arc<deadqueue::unlimited::Queue<ProxyV4>>,
    pub tx: Sender<ProxyV4>,
    pub rx: Receiver<ProxyV4>,

    worker_count: usize,
}

impl Checker {
    pub fn new(worker_count: usize) -> Self {
        let (tx, rx) = channel(1024);
        let queue = Arc::new(deadqueue::unlimited::Queue::new());
        return Self {
            queue,
            tx,
            rx,
            worker_count,
        };
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
                    let proxy = queue.pop().await;
                    match Checker::check(proxy.clone()).await {
                        Some(p) => {
                            tx.send(p).await.ok();
                        },
                        None => {
                            LOGGER.warn(format!(
                                "Proxy {} dead.",
                                proxy.to_string()
                            ))
                        }
                    }
                }
            }); 
        }
    }

    pub async fn check(mut proxy: ProxyV4) -> Option<ProxyV4> {
        let r = reqwest::get("https://google.com/").await.unwrap();
        if r.status().is_success() {
            proxy.google = true;
            proxy.last_checked = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
            return Some(proxy);
        }
        return None;
    }
}
