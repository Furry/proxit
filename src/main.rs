#![feature(async_closure)]

pub mod collection;
pub mod webserver;
pub mod proxies;
pub mod logger;
pub mod loader;
pub mod utils;
pub mod cache;
pub mod lua;

use std::sync::{Arc, Mutex};
use cache::Cache;
use proxies::ProxyV4;
use proxies::checker::Checker;
use lazy_static::lazy_static;
use crate::proxies::ProxyType;

lazy_static! {
    pub static ref CACHE: Arc<Mutex<Cache>> = Arc::new(Mutex::new(Cache::new()));
}

#[tokio::main]
async fn main() {
    let checker = Checker::new(250);

    load(&checker).await;

    tokio::join!(
        cache_loop(checker),
        webserver::webserver()
    );
}

async fn load(checker: &Checker) {
    let addrs = loader::load().await
        .iter()
        .map(|p| proxies::ProxyV4::parse(p))
        .filter(|p| p.is_some())
        .map(|p| p.unwrap())
        .collect::<Vec<ProxyV4>>();

    checker.add(addrs);
}

async fn cache_loop(checker: Checker) {
    let cache = CACHE.clone();
    let receiver_container = checker.get_reciever();
    let mut receiver = receiver_container.lock().unwrap();
    let mut c = 0;
    loop {
        while let Some(proxy) = receiver.recv().await {
            if proxy.proxy_type != ProxyType::INVALID && proxy.proxy_type != ProxyType::UNKNOWN {
                c += 1;
                println!("{}", c);
                cache.lock().unwrap().add(proxy);
            }
        }
    }
}