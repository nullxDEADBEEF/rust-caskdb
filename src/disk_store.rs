
#[cfg(test)]
mod tests {
    #[test]
    fn test_disk_store_get() {
        let store = NewDiskStore("test.db");
        if store.is_err() {
            panic!("Failed to create disk store");
        }
    }

    #[test]
    fn test_disk_store_get_invalid() {

    }

    #[test]
    fn test_disk_store_set_with_persistence() {

    }

    #[test]
    fn test_disk_store_delete() {

    }
}