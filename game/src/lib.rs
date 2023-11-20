use wasm_bindgen::prelude::*;
use web_sys::console;

#[macro_use]
mod browser;
mod engine;


#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    console::log_1(&JsValue::from_str("Hello world!"));

    Ok(())
}
