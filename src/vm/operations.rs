use super::JS_value;
use super::temp::js_value_to_string;

pub fn add(a: &JS_value, b: &JS_value) -> JS_value {
    match (a, b) {
        (&JS_value::JS_NUMBER(x), &JS_value::JS_NUMBER(y)) => {
            return JS_value::JS_NUMBER(x + y)
        },
        _ => return JS_value::JS_STRING(format!("{}{}", js_value_to_string(a), js_value_to_string(b)))
    }
}

pub fn mlp(a: &JS_value, b: &JS_value) -> JS_value {
    match (a, b) {
        (&JS_value::JS_NUMBER(x), &JS_value::JS_NUMBER(y)) => {
            return JS_value::JS_NUMBER(x * y)
        },
        _ => return JS_value::JS_NAN
    }
}

pub fn div(a: &JS_value, b: &JS_value) -> JS_value {
    match (a, b) {
        (&JS_value::JS_NUMBER(x), &JS_value::JS_NUMBER(y)) => {
            return JS_value::JS_NUMBER(x / y)
        },
        _ => return JS_value::JS_NAN
    }
}

pub fn sub(a: &JS_value, b: &JS_value) -> JS_value {
    match (a, b) {
        (&JS_value::JS_NUMBER(x), &JS_value::JS_NUMBER(y)) => {
            return JS_value::JS_NUMBER(x - y)
        },
        _ => return JS_value::JS_NAN
    }
}
