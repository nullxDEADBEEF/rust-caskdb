struct MemoryStorage {

}

impl MemoryStorage {
    fn new() -> Self {
        Self {}
    }

    fn set(&self, key: &str, value: &str) {} 
    fn get(&self, key: &str) -> String { "".to_owned() }
    fn close(&self) -> bool { true }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_store_get() {
        let store = MemoryStorage::new();
        store.set("name", "jojo");
        assert_eq!(store.get("name"), "jojo");
    }

    #[test]
    fn test_memory_store_invalid_get() {
        let store = MemoryStorage::new();
        assert_eq!(store.get("some random key"), "")
    }

    #[test]
    fn test_memory_store_close() {
        let store = MemoryStorage::new();
        assert!(store.close())
    }
}
