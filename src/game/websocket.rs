#![cfg(target_arch = "wasm32")]

use wasm_bindgen::prelude::*;
use wasm_bindgen::{prelude::Closure, JsValue};
use web_sys::{console, ErrorEvent, MessageEvent, WebSocket};

// https://rustwasm.github.io/wasm-bindgen/examples/websockets.html
pub fn send_to_server() -> Result<(), JsValue> {
    let url = "http://127.0.0.1:3000";

    console::log_2(&JsValue::from_str("connecting "), &JsValue::from_str(url));

    let ws = WebSocket::new(url)?;
    let cloned_ws = ws.clone();

    console::log_1(&JsValue::from_str("socket created "));

    let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
        console::log_2(&JsValue::from_str("message event"), &e.data());
    }) as Box<dyn FnMut(MessageEvent)>);
    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    onmessage_callback.forget();

    let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
        console::log_2(
            &JsValue::from_str("error event"),
            &JsValue::from_str(e.message().as_str()),
        );
    }) as Box<dyn FnMut(ErrorEvent)>);
    ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
    onerror_callback.forget();

    let onopen_callback = Closure::wrap(Box::new(move |_| {
        console::log_1(&JsValue::from_str("socket opended"));

        cloned_ws.send_with_str("hello from rust");
    }) as Box<dyn FnMut(JsValue)>);
    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();

    console::log_1(&JsValue::from_str("callback initialized "));

    Ok(())
}
