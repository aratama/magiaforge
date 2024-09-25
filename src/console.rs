#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

#[cfg(target_arch = "wasm32")]
use web_sys::console;

#[cfg(target_arch = "wasm32")]
#[allow(dead_code)]
pub fn log<T: Into<JsValue>>(value: T) {
    #[cfg(target_arch = "wasm32")]
    console::log_1(&value.into());

    #[cfg(not(target_arch = "wasm32"))]
    println!("{}", value.into().as_string().unwrap());
}

#[cfg(target_arch = "wasm32")]
#[allow(dead_code)]
pub fn log2<T: Into<JsValue>, S: Into<JsValue>>(value: T, s: S) {
    #[cfg(target_arch = "wasm32")]
    console::log_2(&value.into(), &s.into());

    #[cfg(not(target_arch = "wasm32"))]
    println!(
        "{} {}",
        value.into().as_string().unwrap(),
        s.into().as_string().unwrap()
    );
}
