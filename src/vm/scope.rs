use super::JsValue;
use std::collections::HashMap;
use std;

#[derive(Clone)]
pub struct Scope {
    id: i32,
    is_global: bool,
    parent: i32,
    variables: HashMap<String, JsValue>,
}

impl Scope {
    pub fn new(id: i32, parent: i32) {
        Scope {
            id: id,
            is_global: false,
            variables: HashMap::new(),
            parent: id
        };
    }

    pub fn new_global() -> Scope {
        Scope { id: 0, is_global: true, variables: HashMap::new(), parent: 0 }
    }

    pub fn get_var(&self, string: String) -> JsValue {
        match self.variables.get(&string) {
            Some(a) => return a.clone(),
            None => return JsValue::JsUndefined
        }
    }

    pub fn set_var(&mut self, string: String, js_value: JsValue) {
        self.variables.insert(string, js_value);
    }
}
