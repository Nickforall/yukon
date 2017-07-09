use super::JsValue;
use super::temp::js_value_to_string;
use super::types::rust_to_js_boolean;
use super::types::flip_js_bool;

pub fn add(a: &JsValue, b: &JsValue) -> JsValue {
    match (a, b) {
        (&JsValue::JsNumber(x), &JsValue::JsNumber(y)) => {
            return JsValue::JsNumber(x + y)
        },
        _ => return JsValue::JsString(format!("{}{}", js_value_to_string(a), js_value_to_string(b)))
    }
}

pub fn mlp(a: &JsValue, b: &JsValue) -> JsValue {
    match (a, b) {
        (&JsValue::JsNumber(x), &JsValue::JsNumber(y)) => {
            return JsValue::JsNumber(x * y)
        },
        (&JsValue::JsString(ref x), &JsValue::JsNumber(y)) => {
            match x.clone().parse::<f64>() {
                Ok(parsed_x) => return JsValue::JsNumber(parsed_x * y),
                Err(_) => return JsValue::JsNan,
            }
        }
        (&JsValue::JsNumber(x), &JsValue::JsString(ref y)) => {
            match y.clone().parse::<f64>() {
                Ok(parsed_y) => return JsValue::JsNumber(x * parsed_y),
                Err(_) => return JsValue::JsNan,
            }
        }
        _ => return JsValue::JsNan
    }
}

pub fn div(a: &JsValue, b: &JsValue) -> JsValue {
    match (a, b) {
        (&JsValue::JsNumber(x), &JsValue::JsNumber(y)) => {
            return JsValue::JsNumber(x / y)
        },
        (&JsValue::JsString(ref x), &JsValue::JsNumber(y)) => {
            match x.clone().parse::<f64>() {
                Ok(parsed_x) => return JsValue::JsNumber(parsed_x / y),
                Err(_) => return JsValue::JsNan,
            }
        }
        (&JsValue::JsNumber(x), &JsValue::JsString(ref y)) => {
            match y.clone().parse::<f64>() {
                Ok(parsed_y) => return JsValue::JsNumber(x / parsed_y),
                Err(_) => return JsValue::JsNan,
            }
        }
        _ => return JsValue::JsNan
    }
}

pub fn sub(a: &JsValue, b: &JsValue) -> JsValue {
    match (a, b) {
        (&JsValue::JsNumber(x), &JsValue::JsNumber(y)) => {
            return JsValue::JsNumber(x - y)
        },
        (&JsValue::JsString(ref x), &JsValue::JsNumber(y)) => {
            match x.clone().parse::<f64>() {
                Ok(parsed_x) => return JsValue::JsNumber(parsed_x - y),
                Err(_) => return JsValue::JsNan,
            }
        }
        (&JsValue::JsNumber(x), &JsValue::JsString(ref y)) => {
            match y.clone().parse::<f64>() {
                Ok(parsed_y) => return JsValue::JsNumber(x - parsed_y),
                Err(_) => return JsValue::JsNan,
            }
        }
        _ => return JsValue::JsNan
    }
}

fn handle_eq_with_bool(boolValue: &JsValue, comparedValue: &JsValue) -> JsValue {
    match comparedValue {
        &JsValue::JsString(ref x) => {
            if !x.is_empty() {
                match x.clone().parse::<f64>() {
                    Ok(parsed) => return rust_to_js_boolean(parsed == 0 as f64),
                    Err(_) => return rust_to_js_boolean(boolValue != (&JsValue::JsFalse)),
                }
            } else {
                return rust_to_js_boolean(boolValue == (&JsValue::JsFalse))
            }
        },
        &JsValue::JsNumber(x) => {
            if x != 0 as f64 {
                return rust_to_js_boolean(boolValue != (&JsValue::JsFalse))
            } else {
                return rust_to_js_boolean(boolValue == (&JsValue::JsFalse))
            }
        },
        _ => {
            return rust_to_js_boolean(boolValue != &JsValue::JsFalse)
        }
    }
}

pub fn eq(a: &JsValue, b: &JsValue) -> JsValue {
    if a == b {
        return rust_to_js_boolean(true)
    }

    if a == &JsValue::JsTrue || a == &JsValue::JsFalse {
        return handle_eq_with_bool(a, b)
    }

    if b == &JsValue::JsTrue || b == &JsValue::JsFalse {
        return handle_eq_with_bool(b, a)
    }

    match (a, b) {
        (&JsValue::JsString(ref x), &JsValue::JsString(ref y)) => {
            return rust_to_js_boolean(x == y)
        },
        (&JsValue::JsNumber(x), &JsValue::JsNumber(y)) => {
            return rust_to_js_boolean(x == y)
        },
        (&JsValue::JsString(ref x), &JsValue::JsNumber(y)) => {
            match x.clone().parse::<f64>() {
                Ok(parsed_x) => return rust_to_js_boolean(parsed_x == y),
                Err(_) => return JsValue::JsFalse,
            }
        }
        (&JsValue::JsNumber(x), &JsValue::JsString(ref y)) => {
            match y.clone().parse::<f64>() {
                Ok(parsed_y) => return rust_to_js_boolean(x == parsed_y),
                Err(_) => return JsValue::JsFalse,
            }
        }
        _ => return JsValue::JsFalse
    }
}

pub fn neq(a: &JsValue, b: &JsValue) -> JsValue {
    return flip_js_bool(eq(a, b))
}

pub fn strict_eq(a: &JsValue, b: &JsValue) -> JsValue {
    match (a, b) {
        (&JsValue::JsString(ref x), &JsValue::JsString(ref y)) => {
            return rust_to_js_boolean(x == y)
        },
        (&JsValue::JsNumber(x), &JsValue::JsNumber(y)) => {
            return rust_to_js_boolean(x == y)
        },
        (&JsValue::JsTrue, &JsValue::JsTrue) => {
            return rust_to_js_boolean(true)
        },
        (&JsValue::JsFalse, &JsValue::JsFalse) => {
            return rust_to_js_boolean(true)
        }
        _ => return JsValue::JsFalse
    }
}

pub fn strict_neq(a: &JsValue, b: &JsValue) -> JsValue {
    return flip_js_bool(strict_eq(a, b))
}
