use bevy::utils::HashMap;

pub trait StateContext {
    fn get_value(&self, key: &str) -> Option<&bool>;
    fn set_value(&mut self, key: &str, value: &bool);
}

impl StateContext for HashMap<String, bool> {
    fn get_value(&self, key: &str) -> Option<&bool> {
        self.get(key)
    }

    fn set_value(&mut self, key: &str, value: &bool) {
        self.insert(key.to_string(), *value);
    }
}
