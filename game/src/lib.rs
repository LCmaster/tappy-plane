use anyhow::Result;
use engine::Engine;
use game::{TappyPlane, Waiting};
use wasm_bindgen::prelude::*;

#[macro_use]
mod browser;
mod physics;
mod engine;
mod utils;
mod game;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    browser::spawn_local(async move {
        let game = TappyPlane::default();
        Engine::start(game).await.expect("Could not start game loop");
    });

    Ok(())
}
