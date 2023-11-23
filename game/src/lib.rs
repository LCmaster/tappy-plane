use anyhow::Result;
use engine::GameLoop;
use game::TappyPlane;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[macro_use]
mod browser;
mod engine;
mod game;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    console::log_1(&JsValue::from_str("Hello world!"));

    browser::spawn_local(async move {
        let game = TappyPlane{
            image: None,
            sheet: None,
            frame: 0
        };
        GameLoop::start(game).await.expect("Could not start game loop");
    });

    Ok(())
}
