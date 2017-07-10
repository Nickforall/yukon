use super::JsValue;

pub fn rust_to_js_boolean(b: bool) -> JsValue {
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
