use std::fs;

use rlua::Table;
use colored::*;
use crate::logger::LOGGER;

pub fn load(lua: rlua::Lua) -> Vec<String> {
    let mut addrs = Vec::new();

    // Iterate over every lua file in ./sources
    for entry in fs::read_dir("./sources").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let filename = path.file_name().unwrap().to_str().unwrap();
        if filename.ends_with(".lua") {
            let contents = fs::read_to_string(path).unwrap();
            lua.context(|ctx| {
                let r = ctx.load(&contents).eval::<Table>().unwrap();
                let x: Vec<String> = r.get("addresses").unwrap();
                let name: String = r.get("name").unwrap();

                LOGGER.log(format!(
                    "Loaded {} proxies from {}",
                    x.len().to_string().blue(),
                    name.blue()
                ));

                addrs.extend(x);
            });
        }
    }

    return addrs;
}