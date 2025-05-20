use super::abstract_storage::*;

#[derive(Debug)]
pub struct FixedSizedCache {
    cache: std::collections::HashMap<String, Record>,
    size: usize,
    current_size: usize,
}
impl FixedSizedCache {
    pub fn new(size: usize) -> Self {
        FixedSizedCache {
            cache: std::collections::HashMap::new(),
            size: size,
            current_size: 0,
        }
    }
}
impl AbstractStorage for FixedSizedCache {
    fn size(&self) -> usize {
        self.current_size
    }
    fn get(&self, key: &str) -> Option<Record> {
        self.cache.get(key).cloned()
    }

    fn put(&mut self, key: &str, value: Entry) {
        let value_size = key.len() + value.data.len();
        if self.current_size + value_size > self.size {
            panic!("Value size exceeds cache size");
        }
        let record = Record {
            data: value.data,
            type_suggestion: value.type_suggestion,
            key: key.to_string().clone(),
        };
        self.current_size += value_size;
        self.cache.insert(key.to_string(), record);
    }

    fn remove(&mut self, key: &str) {
        if let Some(value) = self.cache.get(key) {
            self.current_size -= key.len() + value.data.len();
        }
        self.cache.remove(key);
    }
    fn list(&self, limit: usize) -> Vec<Record> {
        let mut records = Vec::new();
        let mut count = 0;
        for (_key, value) in self.cache.iter() {
            if count >= limit {
                break;
            }
            records.push(value.clone());
            count += 1;
        }
        records
    }
    fn list_all(&self) -> Vec<Record> {
        let mut records = Vec::new();
        for (_key, value) in self.cache.iter() {
            records.push(value.clone());
        }
        records
    }
    fn clear(&mut self) {
        self.cache.clear();
        self.current_size = 0;
    }
    fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }
    fn contains_key(&self, key: &str) -> bool {
        self.cache.contains_key(key)
    }
}
