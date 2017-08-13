use super::JsValue;

pub fn js_value_to_string(val: &JsValue) -> String {
    match val {
        &JsValue::JsNull => return "null".to_owned(),
        &JsValue::JsUndefined => return "undefined".to_owned(),
        &JsValue::JsNan => return "NaN".to_owned(),
        &JsValue::JsNumber(num) => return format!("{}", num),
        &JsValue::JsString(ref s) => return s.clone(),
        &JsValue::JsTrue => return "true".to_owned(),
        &JsValue::JsFalse => return "false".to_owned(),
        &JsValue::JsFalse => return "object".to_owned(),

    }
}
