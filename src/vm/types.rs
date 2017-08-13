use super::JsValue;
use std::collections::HashMap;

#[derive(Clone)]
pub enum JsObjectType {
    JsObject,
}

#[derive(Clone)]
pub struct JsObject<'a> {
    t: JsObjectType,
    prototype: &'a JsObject<'a>,
    properties: HashMap<String, JsValue>
}

impl<'a> JsObject<'a> {
    pub fn new(t: JsObjectType, prototype: &'a JsObject<'a>) -> Self {
        JsObject {t: t, prototype: prototype, properties: HashMap::new()}
    }
}

pub fn rust_to_js_boolean<'a>(b: bool) -> JsValue {
    if b == true {
        return JsValue::JsTrue
    } else {
        return JsValue::JsFalse
    }
}

pub fn try_js_value_to_js_number(v: &JsValue) -> JsValue {
    match v {
        &JsValue::JsString(ref string) => {
            match string.clone().parse::<f64>() {
                Ok(parsed_x) => return JsValue::JsNumber(parsed_x),
                Err(_) => return JsValue::JsString(string.clone()),
            }
        },
        _ => return v.clone()
    }
}

pub fn flip_js_bool(b: JsValue) -> JsValue {
    return rust_to_js_boolean(b != JsValue::JsTrue)
}
