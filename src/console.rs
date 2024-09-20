use wasm_bindgen::JsValue;
use web_sys::console;

pub fn log<T: Into<JsValue>>(value: T) {
    console::log_1(&value.into());
}
