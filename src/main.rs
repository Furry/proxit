pub mod collection;
pub mod proxies;
pub mod logger;
pub mod loader;
pub mod cache;
pub mod lua;

use std::fs;
use std::io::Write;
use cache::Cache;
use lua::bindings;
use proxies::ProxyV4;
use proxies::checker::Checker;
use rlua::prelude::LuaValue;
use rlua::{FromLuaMulti, Table};
use tokio::task::spawn_blocking;

use crate::proxies::ProxyType;

#[tokio::main]
async fn main() {
    let addrs = loader::load().await
        .iter()
        .map(|p| proxies::ProxyV4::parse(p))
        .filter(|p| p.is_some())
        .map(|p| p.unwrap())
        .collect::<Vec<ProxyV4>>();

    let checker = Checker::new(250);
    // let cache: Cache::new();
    let proxy_count = addrs.len();
    checker.add(addrs);

    // println!("{}", addrs.len());
    checker.start().await;

    let receiver_container = checker.get_reciever();
    let mut receiver = receiver_container.lock().unwrap();
    // create a file
    let mut file = fs::File::create("./proxies.json").unwrap();
    let mut count = 0;
    let mut cache: Vec<ProxyV4> = Vec::new();

    loop {
        match receiver.recv().await {
            Some(p) => {
                count += 1;
                // if p.proxy_type == ProxyType::HTTPS || p.proxy_type == ProxyType::HTTP {
                //     file.write(format!("{}\n", p.to_string()).as_bytes()).unwrap();
                // };
                println!("{}", count);
                if p.proxy_type != ProxyType::INVALID {
                    cache.push(p);
                }
            }
            None => {
                break;
            }
        }
        // if count == 5000 {
        if count == proxy_count {
            println!("DONE!");
            // parse proxies into a json object and write it to file
            let json = serde_json::to_string(&cache).unwrap();
            file.write(json.as_bytes()).unwrap();
            break;
        }
    }
}