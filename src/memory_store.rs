

#[cfg(test)]
mod tests {
    #[test]
    fn test_memory_store_get() {
        let store = NewMemoryStorage();
        store.set("name", "jojo");
        assert_eq!(store.get("name"), "jojo");
    }

    #[test]
    fn test_memory_store_invalid_get() {
        let store = NewMemoryStorage();
        assert_eq!(store.get("some random key"), "")
    }

    #[test]
    fn test_memory_store_close() {
        let store = NewMemoryStorage();
        assert!(store.close())
    }
}
