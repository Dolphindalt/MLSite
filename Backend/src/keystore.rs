use std::collections::HashSet;

pub struct KeyStore {
    active_keys: HashSet<String>,
}

impl KeyStore {
    pub fn new() -> KeyStore {
        KeyStore { active_keys: HashSet::new(), }
    }

    pub fn add_key(&mut self, hex: &String) {
        self.active_keys.insert(hex.clone());
    }

    pub fn remove_key(&mut self, hex: &String) {
        self.active_keys.remove(hex);
    }

    pub fn contains_key(&mut self, hex: &String) {
        self.active_keys.contains(hex);
    }
}