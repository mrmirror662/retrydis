use retrydis::storage::abstract_storage::{DataType, Entry};
use retrydis::storage::storage_instance::{StorageInstance, StorageType};

fn run_storage_tests(storage_type: StorageType) {
    let mut cache = StorageInstance::from_type(storage_type);
    println!("--- Starting Tests for StorageInstance ---");

    // Test put and size
    let key1 = cache.put(Entry::new("hello".to_string(), DataType::String));
    let key2 = cache.put(Entry::new("world".to_string(), DataType::String));
    cache.put(Entry::new("foo".to_string(), DataType::String));
    cache.put(Entry::new("bar".to_string(), DataType::String));

    // Test get existing
    println!(
        "[Get Existing] {}",
        if cache.get(&key1).is_some() {
            "✅ PASS"
        } else {
            "❌ FAIL"
        }
    );

    // Test get non-existent
    println!(
        "[Get Non-existent] {}",
        if cache.get("nonexistent_key").is_none() {
            "✅ PASS"
        } else {
            "❌ FAIL"
        }
    );

    // Test contains_key
    println!(
        "[Contains Existing Key] {}",
        if cache.contains_key(&key2) {
            "✅ PASS"
        } else {
            "❌ FAIL"
        }
    );
    println!(
        "[Contains Non-existent Key] {}",
        if !cache.contains_key("invalid_key") {
            "✅ PASS"
        } else {
            "❌ FAIL"
        }
    );

    // Test remove
    cache.remove(&key2);
    println!(
        "[Remove Key2 + Check Contains] {}",
        if !cache.contains_key(&key2) {
            "✅ PASS"
        } else {
            "❌ FAIL"
        }
    );

    // Test list (limit)
    println!(
        "[List Limit = 2] {}",
        if cache.list(2).len() == 2 {
            "✅ PASS"
        } else {
            "❌ FAIL"
        }
    );

    // Test list_all
    let all_entries = cache.list_all();
    println!(
        "[List All] {}",
        if all_entries.len() == 3 {
            "✅ PASS".to_string()
        } else {
            format!("❌ FAIL (expected 3, got {})", all_entries.len())
        }
    );

    // Test clear
    cache.clear();
    println!(
        "[Clear Cache + Size == 0] {}",
        if cache.size() == 0 {
            "✅ PASS"
        } else {
            "❌ FAIL"
        }
    );

    println!("--- Tests Completed ---");
}

fn main() {
    run_storage_tests(StorageType::FixedCache);
}
