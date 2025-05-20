#[derive(Clone, Debug)]
#[allow(unused)]
pub enum DataType {
    String,
    Integer,
    Float,
    Boolean,
}
#[derive(Clone, Debug)]
#[allow(unused)]
pub struct Record {
    pub data: String,
    pub type_suggestion: DataType,
    pub uid: String,
}
pub struct Entry {
    pub data: String,
    pub type_suggestion: DataType,
}
impl Entry {
    pub fn new(data: String, type_suggestion: DataType) -> Self {
        Entry {
            data,
            type_suggestion,
        }
    }
}
#[allow(unused)]

pub trait AbstractStorage {
    fn get(&self, key: &str) -> Option<Record>;
    fn put(&mut self, value: Entry) -> String;
    fn remove(&mut self, key: &str);
    fn size(&self) -> usize;
    fn clear(&mut self);
    fn is_empty(&self) -> bool;
    fn contains_key(&self, key: &str) -> bool;
    fn list(&self, limit: usize) -> Vec<Record>;
    fn list_all(&self) -> Vec<Record>;
}
