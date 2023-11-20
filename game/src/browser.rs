use anyhow::{anyhow, Result};
use js_sys::Function;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{Window, Document, HtmlImageElement, CanvasRenderingContext2d, HtmlCanvasElement};

macro_rules! log {
    ( $($t:tt)* ) => {
        web_sys::console::log_1(&format!($($t)*).into());
    }
}

pub fn window() -> Result<Window> {
    web_sys::window().ok_or_else(|| anyhow!("No window found"))
}

pub fn document() -> Result<Document> {
    window()?.document().ok_or_else(|| anyhow!("No document found"))
}

pub fn canvas(id: &str) -> Result<HtmlCanvasElement> {
    document()?
        .get_element_by_id(id)
        .ok_or_else(|err| anyhow!("No Canvas Element found with ID '{:#?}'", id))?
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|element| anyhow!("Error converting {:#?} to HtmlCanvasElement", element))
}

pub fn context(canvas_id: &str) -> Result<CanvasRenderingContext2d> {
    canvas(canvas_id)?
        .get_context("2d")
        .map_err(|err| anyhow!("Error getting 2d context {:#?}", err))?
        .ok_or_else(|| anyhow!("No 2d context found"))?
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .map_err(|element| anyhow!("Error converting {:#?} to CanvasRenderingContext2d", element))
}

pub fn new_image() -> Result<HtmlImageElement> {
    HtmlImageElement::new().map_err(|err| anyhow!("Could not create HtmlImageElement: {:#?}", err))
}

pub fn request_animation_frame(callback: &Closure<dyn FnMut(f64)>) -> Result<i32> {
    window()?
        .request_animation_frame(callback.as_ref().unchecked_ref())
        .map_err(|err| anyhow!("Could not request animation frame {:#?}", err))
}

pub fn create_animation_closure(f: impl FnMut(f64) + 'static) -> Closure<dyn FnMut(f64)> {
    Closure::wrap(Box::new(f))
}

pub fn now() -> Result<f32> {
    Ok(js_sys::Date::now() as f32)
}
