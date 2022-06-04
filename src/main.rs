pub mod collection;
pub mod proxies;
pub mod logger;
pub mod loader;
pub mod lua;
use std::fs;
use lua::bindings;
use proxies::ProxyV4;
use proxies::checker::Checker;
use rlua::prelude::LuaValue;
use rlua::{FromLuaMulti, Table};
use tokio::task::spawn_blocking;

#[tokio::main]
async fn main() {
    // let lua = bindings::Lua::binded();

    // read the file ./sources-glarketm-proxylist.lua
    // let contents = fs::read_to_string("./sources/github-clarketm-proxylist.lua").unwrap();

    // let addrs = loader::load(&lua)
    //     .iter()
    //     .map(|p| proxies::ProxyV4::parse(p))
    //     .collect::<Vec<ProxyV4>>();

    // let count = addrs.len();
    // let checker = Checker::new(10);
    // checker.add(addrs);
    // checker.start();

    // println!("{}", count)

    let addrs = loader::load().await;
    println!("{}", addrs.len());
}