use super::JsValue;

pub fn rust_to_js_boolean(b: bool) -> JsValue {
    if b == true {
        return JsValue::JsTrue
    } else {
        return JsValue::JsFalse
    }
}

pub fn flip_js_bool(b: JsValue) -> JsValue {
    return rust_to_js_boolean(b != JsValue::JsTrue)
}
