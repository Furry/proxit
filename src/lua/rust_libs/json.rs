use std::iter::Map;

use rlua::{ Context, Result, Value, ToLuaMulti, MultiValue, UserData };

pub struct JSONObject(serde_json::Value);
impl UserData for JSONObject {}

// pub fn bind(ctx: Context) -> Result<()> {
//     let json = ctx.create_table()?;

//     let load = ctx.create_function(|ctx, content: String| {
//         let o = serde_json::from_str(content);
//         if o.is_ok() {
//             let object = o.unwrap();
//         }
//     })?;

//     json.set("load", load)?;
//     return Ok(());
// }