use tokio::sync::RwLock;
use std::collections::HashMap;
struct KeyValueStore {
    // rwlock for co-current read , mutex will lock in case of read as well
    store: RwLock<HashMap<String, String>>,
}


impl KeyValueStore {
    fn new() -> Self {
        KeyValueStore {
            store: RwLock::new(HashMap::new()),
        }
    }

    async fn set(&self, key: String, value: String) {
        let mut store = self.store.write().await;
        store.insert(key, value);
    }

    async fn get(&self, key: &str) -> Option<String> {
        let store = self.store.read().await;
        store.get(key).cloned()
    }
}