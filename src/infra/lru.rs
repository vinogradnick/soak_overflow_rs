use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

#[derive(Debug, Default)]
pub struct LruCache<K, V> {
    capacity: usize,
    map: HashMap<K, V>,
    order: VecDeque<K>,
}

impl<K: Eq + Hash + Clone, V> LruCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            map: HashMap::new(),
            order: VecDeque::new(),
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.map.contains_key(key) {
            // обновляем порядок
            self.order.retain(|k| k != key);
            self.order.push_back(key.clone());
            self.map.get(key)
        } else {
            None
        }
    }

    pub fn put(&mut self, key: K, value: V) {
        if self.map.contains_key(&key) {
            self.order.retain(|k| k != &key);
        } else if self.map.len() == self.capacity {
            // удаляем старый
            if let Some(old) = self.order.pop_front() {
                self.map.remove(&old);
            }
        }
        self.order.push_back(key.clone());
        self.map.insert(key, value);
    }
}

// fn main() {
//     let mut cache = LruCache::new(2);
//     cache.put(1, "one");
//     cache.put(2, "two");
//     println!("{:?}", cache.get(&1)); // Some("one")
//     cache.put(3, "three"); // удалится 2
//     println!("{:?}", cache.get(&2)); // None
// }
