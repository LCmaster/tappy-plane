use std::{rc::Rc, cell::RefCell, sync::Mutex, collections::HashMap};

use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};
use futures;

use crate::browser;

pub const FRAMES_PER_SECOND: f64 = 60.0;
pub const FRAME_RATE: f64 = 1.0 / FRAMES_PER_SECOND;
pub const FRAME_RATE_IN_MILLISECONDS: f64 = FRAME_RATE * 1000.0;

#[derive(Debug, Deserialize)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

#[derive(Debug, Deserialize)]
pub struct Spritesheet {
    pub image: String,
    pub tileset: HashMap<String, Rect>,
}

pub struct Renderer {
    context: CanvasRenderingContext2d,
}

impl Renderer {
    pub fn clear(&self, rect: &Rect) {
        self
            .context
            .clear_rect(rect.x.into(), rect.y.into(), rect.width.into(), rect.height.into());
    }

    pub fn draw_image(&self, image: &HtmlImageElement, orig: &Rect, dest: &Rect) {
        self
            .context
            .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                &image,

                orig.x.into(),
                orig.y.into(),
                orig.width.into(),
                orig.height.into(),

                dest.x.into(),
                dest.y.into(),
                dest.width.into(),
                dest.height.into()
            )
            .expect("Drawing is throwing exceptions! Unrecoverable error.");
    }
}

#[async_trait(?Send)]
pub trait Game {
    async fn initialize(&self) -> Result<Box<dyn Game>>;
    fn update(&mut self);
    fn draw(&self, renderer: &Renderer);
}

pub struct GameLoop{
    last_frame: f64,
    accumulated_delta: f64,
}
impl GameLoop {
    pub async fn start(game: impl Game + 'static) -> Result<()> {
        let mut game = game.initialize().await?;
        let mut game_loop = GameLoop {
            last_frame: browser::now()?,
            accumulated_delta: 0.0,
        };

        let renderer = Renderer {
            context: browser::context()?,
        };

        let f: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>> = Rc::new(RefCell::new(None));
        let g = f.clone();

        let animate = Some(browser::create_animation_closure(move |delta: f64| {
            game_loop.accumulated_delta += delta - game_loop.last_frame;

            while game_loop.accumulated_delta > FRAME_RATE_IN_MILLISECONDS {
                game.update();
                game_loop.accumulated_delta -= FRAME_RATE_IN_MILLISECONDS;
            }

            game_loop.last_frame = delta;
            game.draw(&renderer);

            browser::request_animation_frame(f.borrow().as_ref().unwrap());
        }));

        *g.borrow_mut() = animate;
        browser::request_animation_frame(g.borrow().as_ref().unwrap())?;

        Ok(())
    }
}

pub async fn load_image(source: &str) -> Result<HtmlImageElement> {
    let image = browser::new_image().unwrap();

        let (success_tx, success_rx) = futures::channel::oneshot::channel::<Result<(), JsValue>>();
        let success_tx = Rc::new(Mutex::new(Some(success_tx)));
        let error_tx = Rc::clone(&success_tx);

        let callback = Closure::once(Box::new(move || {
            if let Some(success_tx) = success_tx.lock().ok().and_then(|mut opt| opt.take()) {
                success_tx.send(Ok(()));
            }
        }));

        let error_callback = Closure::once(Box::new(move |err| {
            if let Some(error_tx) = error_tx.lock().ok().and_then(|mut opt| opt.take()) {
                error_tx.send(Err(err));
            }
        }));
                                 
        image.set_onerror(Some(error_callback.as_ref().unchecked_ref()));
        image.set_onload(Some(callback.as_ref().unchecked_ref()));
        image.set_src(source);

        success_rx.await;
    Ok(image)
}
