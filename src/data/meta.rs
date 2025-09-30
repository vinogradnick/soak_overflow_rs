use std::any::Any;
use std::collections::HashMap;

pub struct MetaContext {
    map: HashMap<String, Box<dyn Any>>,
}

impl MetaContext {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn add<T: 'static>(&mut self, key: String, value: T) {
        self.map.insert(key, Box::new(value));
    }

    pub fn get<T: 'static>(&self, key: &str) -> Option<&T> {
        self.map.get(key)?.downcast_ref::<T>()
    }

    pub fn remove(&mut self, key: &str) {
        self.map.remove(key);
    }
}
