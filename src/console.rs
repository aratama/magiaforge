use wasm_bindgen::JsValue;
use web_sys::console;

pub fn log<T: Into<JsValue>>(value: T) {
    console::log_1(&value.into());
}

pub fn log2<T: Into<JsValue>, S: Into<JsValue>>(value: T, s: S) {
    console::log_2(&value.into(), &s.into());
}
