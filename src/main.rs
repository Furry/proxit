#![feature(async_closure)]

pub mod collection;
pub mod webserver;
pub mod proxies;
pub mod logger;
pub mod loader;
pub mod utils;
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
use actix_web::{web, App, HttpServer};

use crate::proxies::ProxyType;

#[tokio::main]
async fn main() {
    let checker = Checker::new(250).await;
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