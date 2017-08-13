use super::JsValue;

use ansi_term::Colour::RGB;

pub fn ret_value_fmt(val: &JsValue) -> String {
    match val {
        &JsValue::JsNull => return format!("{}", RGB(130, 130, 130).paint("null".to_owned())),
        &JsValue::JsUndefined => return format!("{}", RGB(130, 130, 130).paint("undefined".to_owned())),
        &JsValue::JsNan => return format!("{}", RGB(209, 154, 102).paint("NaN".to_owned())),
        &JsValue::JsNumber(num) => return format!("{}", RGB(209, 154, 102).paint(format!("{}", num))),
        &JsValue::JsString(ref s) => return format!("{}", RGB(152, 195, 121).paint(format!("\"{}\"", s.clone()))),
        &JsValue::JsTrue => return format!("{}", RGB(209, 154, 102).paint("true".to_owned())),
        &JsValue::JsFalse => return format!("{}", RGB(209, 154, 102).paint("false".to_owned())),
        //TODO: format objects
        // &JsValue::JsObject(_) => return format!("{}", RGB(209, 154, 102).paint("OBJECT".to_owned())),
    }
}
