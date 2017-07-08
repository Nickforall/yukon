use super::JS_value;

use ansi_term::Colour::RGB;

pub fn ret_value_fmt(val: &JS_value) -> String {
    match val {
        &JS_value::JS_NULL => return format!("{}", RGB(130, 130, 130).paint("null".to_owned())),
        &JS_value::JS_UNDEFINED => return format!("{}", RGB(130, 130, 130).paint("undefined".to_owned())),
        &JS_value::JS_NAN => return format!("{}", RGB(209, 154, 102).paint("NaN".to_owned())),
        &JS_value::JS_NUMBER(num) => return format!("{}", RGB(209, 154, 102).paint(format!("{}", num))),
        &JS_value::JS_STRING(ref s) => return format!("{}", RGB(152, 195, 121).paint(format!("\"{}\"", s.clone()))),
    }
}
