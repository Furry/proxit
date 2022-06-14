use std::collections::VecDeque;
use std::sync::{ Arc, Mutex };
use crate::proxies::ProxyV4;

pub struct Cache {
    pub inner: Arc<Mutex<VecDeque<ProxyV4>>>
}

impl Cache {
    pub fn new() -> Self {
        return Self {
            inner: Arc::new(Mutex::new(VecDeque::new()))
        };
    }

    pub fn add(&self, proxy: ProxyV4) {
        let mut inner = self.inner.lock().unwrap();
        if !inner.contains(&proxy) {
            inner.push_front(proxy);
        }
    }

    pub fn get(&self) -> Option<ProxyV4> {
        let mut inner = self.inner.lock().unwrap();
        return inner.pop_back();
    }

    pub fn first(&self) -> Option<ProxyV4> {
        let inner = self.inner.lock().unwrap();
        return inner.front().cloned();
    }

    pub fn random(&self) -> Option<ProxyV4> {
        let inner = self.inner.lock().unwrap();
        // Get a random index.
        let index = rand::random::<usize>() % inner.len();
        // Get the proxy at the index.
        return inner.get(index).cloned();
    }

    pub fn request(&self, amount: usize) -> Vec<ProxyV4> {
        let inner = self.inner.lock().unwrap();
        let mut proxies: Vec<ProxyV4> = Vec::new();

        for i in 0..amount {
            if let Some(proxy) = inner.get(i).cloned() {
                proxies.push(proxy);
            }
        }

        return proxies;
    }

    pub fn request_random(&self, amount: usize) -> Vec<ProxyV4> {
        let mut proxies: Vec<ProxyV4> = Vec::new();
        for _ in 0..amount {
            if let Some(proxy) = self.random() {
                proxies.push(proxy);
            }
        }

        return proxies;
    }
}