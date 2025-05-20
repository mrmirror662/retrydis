use retrydis::storage::abstract_storage::{DataType, Entry};
use retrydis::storage::storage_instance::{StorageInstance, StorageType};

fn main() {
    // Create a storage instance of type FixedCache
    let mut storage = StorageInstance::from_type(StorageType::FixedCache);

    // Put a key-value pair
    let key = "user1";
    let value = Entry {
        data: "hello world".to_string(),
        type_suggestion: DataType::String,
    };
    storage.put(key, value);

    // Get the value back
    if let Some(record) = storage.get(key) {
        println!("Retrieved: {:?}", record);
    } else {
        println!("Key not found!");
    }
}
