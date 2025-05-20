use super::abstract_storage::*;
use super::fixed_sized_cache::FixedSizedCache;
pub enum StorageType {
    FixedCache,
}
#[allow(unused)]
pub struct StorageInstance {
    cache: Box<dyn AbstractStorage>,
    current_storage_type: StorageType,
}
impl StorageInstance {
    pub fn from_type(current_storage_type: StorageType) -> Self {
        let cache = match current_storage_type {
            StorageType::FixedCache => Box::new(FixedSizedCache::new(1024)),
        };
        StorageInstance {
            cache,
            current_storage_type,
        }
    }
    pub fn get(&self, key: &str) -> Option<Record> {
        self.cache.get(key)
    }
    pub fn put(&mut self, value: Entry) -> String {
        self.cache.put(value)
    }
    pub fn remove(&mut self, key: &str) {
        self.cache.remove(key)
    }
    pub fn size(&self) -> usize {
        self.cache.size()
    }
    pub fn clear(&mut self) {
        self.cache.clear()
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.cache.contains_key(key)
    }
    pub fn list(&self, limit: usize) -> Vec<Record> {
        self.cache.list(limit)
    }
    pub fn list_all(&self) -> Vec<Record> {
        self.cache.list_all()
    }
}
