use std::collections::HashMap;

use crate::proxies::ProxyV4;

pub struct Cache {
    pub inner: HashMap<String, ProxyV4>
}

impl Cache {
    pub fn new() -> Self {
        return Self {
            inner: HashMap::new()
        };
    }
}