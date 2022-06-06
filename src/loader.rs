use std::fs;
use rlua::Table;
use colored::*;
use tokio::task::{JoinHandle, spawn_blocking};
use crate::{logger::LOGGER, lua::bindings};


pub async fn load() -> Vec<String> {
    let mut handles: Vec<JoinHandle<Vec<String>>> = Vec::new();

    for entry in fs::read_dir("./sources").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let filename = path.file_name().unwrap().to_str().unwrap();

        if filename.ends_with(".lua") {
            let h = spawn_blocking(move || {
                let lua = bindings::Lua::binded();
                let contents = fs::read_to_string(path).unwrap();
                
                let mut addresses: Vec<String> = Vec::new();
                let mut name: String = String::new();
                // Spawn the lua context and evaluate the script.
                lua.context(|ctx| {
                    let t = ctx.load(&contents).eval::<Table>().unwrap().to_owned();
                    addresses = t.get("addresses").unwrap();
                    name = t.get("name").unwrap();
                });

                LOGGER.log(format!(
                    "Loaded {} proxies from {}",
                    addresses.len().to_string().blue(),
                    name.blue()
                ));
                addresses
            });

            handles.push(h);
        }
    }

    let mut addrs: Vec<String> = Vec::new();
    for h in handles {
        let x = h.await.unwrap();
        addrs.extend(x);
    }

    return addrs;
}