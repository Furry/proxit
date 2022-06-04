pub mod collection;
pub mod logger;
pub mod loader;
pub mod lua;

use std::fs;
use rlua::prelude::LuaValue;
use rlua::{FromLuaMulti, Table};

fn main() {
    let lua = rlua::Lua::new();
    // read the file ./sources-glarketm-proxylist.lua
    // let contents = fs::read_to_string("./sources/github-clarketm-proxylist.lua").unwrap();
    lua.context(move |ctx| {
        lua::bindings::bind(ctx).unwrap();

        // let r = ctx.load(&contents).eval::<Table>().unwrap();
        // // let x: Vec<String> = r.get("addresses").unwrap();

    });

    let addrs = loader::load(lua);
    println!("{}", addrs.len())
}
