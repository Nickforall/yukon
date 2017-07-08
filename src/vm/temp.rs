use super::JS_value;

pub fn js_value_to_string(val: &JS_value) -> String {
    match val {
        &JS_value::JS_NULL => return "null".to_owned(),
        &JS_value::JS_UNDEFINED => return "undefined".to_owned(),
        &JS_value::JS_NAN => return "NaN".to_owned(),
        &JS_value::JS_NUMBER(num) => return format!("{}", num),
        &JS_value::JS_STRING(ref s) => return s.clone(),
    }
}
