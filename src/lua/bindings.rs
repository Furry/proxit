use rlua::{self, Context, Result};
use reqwest;

pub struct Lua(rlua::Lua);

impl Lua {
    pub fn binded() -> rlua::Lua {
        let lua = rlua::Lua::new();
        lua.context(|ctx| {
            bind(ctx).unwrap();

            ctx.load("get('https://github.com')").exec().unwrap();
        });

        return lua;
    }
}

fn bind(ctx: Context) -> Result<()> {
    // Include core files as byte arrays.
    let libs = vec![
        include_bytes!("../lua/lua_libs/json.lua")
    ];

    let request = ctx.create_function(|ctx, url: String| {
        let to_return = ctx.create_table()?;
        
        let response = reqwest::blocking::get(url);
        if response.is_ok() {
            let r = response.unwrap();
            to_return.set("status", r.status().as_u16())?;
            match r.text() {
                Ok(t) => {
                    to_return.set("text", t)?;
                }
                Err(_) => {}
            };
        } else {

        }

        return Ok(to_return);
    }).unwrap();

    // Set each function in the global scope.
    ctx.globals().set("get", request).unwrap();

    // Load every library.
    for lib in libs {
        ctx.load(lib).exec()?;
    }

    Ok(())
}