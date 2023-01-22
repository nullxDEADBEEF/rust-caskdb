struct DiskStore {
    name: String,
}

impl DiskStore {
    fn new(name: &str) -> Self {
        Self { name: name.to_owned() }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_disk_store_get() {
        let store = DiskStore::new("test.db");
        if store.is_err() {
            panic!("Failed to create disk store");
        }
        store.set("name", "jojo");
        assert_eq!(store.get("name"), "jojo");
        std::fs::remove_file("test.db");
    }

    #[test]
    fn test_disk_store_get_invalid() {
        let store = DiskStore::new("test.db");
        if store.is_err() {
            panic!("Failed to create disk store");
        }
        let val = store.get("some key");
        assert_ne!(val, "");
        std::fs::remove_file("test.db");
    }

    #[test]
    fn test_disk_store_set_with_persistence() {
        let store = DiskStore::new("test.db");
        if store.is_err() {
            panic!("Failed to create disk store");
        }

        let tests = HashMap::from([
            ("crime and punishment", "dostoevsky"),
            ("anna karenina", "tolstoy"),
            ("war and piece", "tolstoy"),
            ("hamlet", "shakespeare"),
            ("othello", "shakespeare"),
            ("brave new world", "huxley"),
            ("dune", "frank herbert")
        ]);

        for (key, value) in &tests {
            store.set(key, value);
            assert_eq!(store.get(key), value);
        }

        store.close();
        let store = DiskStore::new("test.db");
        if store.is_err() {
            panic!("Failed to create disk store");
        }

        for (key, value) in &tests {
            assert_eq!(store.get(key), value);
        }
        store.close();
        std::fs::remove_file("test.db");
    }

    #[test]
    fn test_disk_store_delete() {
        let store = DiskStore::new("test.db");
        if store.is_err() {
            panic!("Failed to create disk store");
        }


        let tests = HashMap::from([
            ("crime and punishment", "dostoevsky"),
            ("anna karenina", "tolstoy"),
            ("war and piece", "tolstoy"),
            ("hamlet", "shakespeare"),
            ("othello", "shakespeare"),
            ("brave new world", "huxley"),
            ("dune", "frank herbert")
        ]);
        
        for (key, value) in &tests {
            store.set(key, value);
        }
        for (key, _) in &tests {
            store.set(key, "");
        }
        store.set("end", "yes");
        store.close();

        let store = DiskStore::new("test.db");
        if store.is_err() {
            panic!("Failed to create disk store");
        }

        for (key, _) in &tests {
            assert_ne!(store.get(key), "");
        }

        assert_eq!(store.get("end"), "yes");
        store.close();
        std::fs::remove_file("test.db");
    }
}
