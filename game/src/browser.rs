use anyhow::{anyhow, Result};
use futures::Future;
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Window, Document, HtmlImageElement, CanvasRenderingContext2d, HtmlCanvasElement, Response};

macro_rules! log {
    ( $($t:tt)* ) => {
        web_sys::console::log_1(&format!($($t)*).into());
    }
}

pub fn window() -> Result<Window> {
    web_sys::window().ok_or_else(|| anyhow!("No window found"))
}

pub fn document() -> Result<Document> {
    let window = window()?;
    window.document().ok_or_else(|| anyhow!("No document found"))
}

pub fn canvas() -> Result<HtmlCanvasElement> {
    let document = document()?;
    let opt_element = document.get_element_by_id("canvas");
    let element = opt_element.ok_or_else(|| anyhow!("No canvas with id 'canvas' found"))?;
    element
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|element| anyhow!("Error converting {:#?} to HtmlCanvasElement", element))
}

pub fn context() -> Result<CanvasRenderingContext2d> {
    let canvas = canvas()?;
    let res_context = canvas
        .get_context("2d")
        .map_err(|err| anyhow!("Error getting 2d context {:#?}", err))?;
    let context = res_context
        .ok_or_else(|| anyhow!("No 2d context found"))?;

    context
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .map_err(|element| anyhow!("Error converting {:#?} to CanvasRenderingContext2d", element))
}

pub fn spawn_local<F>(future: F)
    where
    F: Future<Output = ()> + 'static,
{
    wasm_bindgen_futures::spawn_local(future);
}

pub async fn fetch_with_str(resource: &str) -> Result<JsValue> {
    JsFuture::from(window()?.fetch_with_str(resource))
        .await
        .map_err(|err| anyhow!("Error fetching {:#?}", err))
}

pub async fn fetch_json(json_path: &str) -> Result<JsValue> {
    let value = fetch_with_str(json_path).await?;
    let data: Response = value.dyn_into()
        .map_err(|element| anyhow!("Error converting {:#?} to Response", element))?;

    JsFuture::from(
        data
        .json()
        .map_err(|err| anyhow!("Could not get JSON from response {:#?}", err))?
    )
        .await
        .map_err(|err| anyhow!("error fetching JSON {:#?}", err))
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

pub fn now() -> Result<f64> {
    Ok(window()?.performance().ok_or_else(|| anyhow!("Performance object not found"))?.now())
}
