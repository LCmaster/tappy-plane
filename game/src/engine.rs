use std::{rc::Rc, cell::{RefCell, Cell}, sync::Mutex, collections::HashMap, io::BufRead};

use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlImageElement, MouseEvent};
use futures;

use crate::browser;

pub const FRAMES_PER_SECOND: f64 = 60.0;
pub const FRAME_RATE: f64 = 1.0 / FRAMES_PER_SECOND;
pub const FRAME_RATE_IN_MILLISECONDS: f64 = FRAME_RATE * 1000.0;

#[derive(Debug, Deserialize)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Deserialize)]
pub struct Spritesheet {
    pub image: String,
    pub tileset: HashMap<String, Rect>,
}

pub struct Renderer {
    pub context: CanvasRenderingContext2d,
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
    fn update(&mut self, delta: &f64, input: &bool);
    fn draw(&self, renderer: &Renderer);
}

pub struct Position {
    pub x: f64,
    pub y: f64,
}

pub struct GameState;

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

        let input= Rc::new(Cell::new(false));
        {
            let pressed = input.clone();
            let listener = Closure::<dyn FnMut(_)>::new(move |event: MouseEvent| pressed.set(true) );
            browser::canvas()?.add_event_listener_with_callback("mousedown", listener.as_ref().unchecked_ref()).expect("Could not add mousedown listener to canvas");
            listener.forget();
        }
        {
            let pressed = input.clone();
            let listener = Closure::<dyn FnMut(_)>::new(move |event: MouseEvent| pressed.set(false) );
            browser::canvas()?.add_event_listener_with_callback("mouseup", listener.as_ref().unchecked_ref()).expect("Could not add mouseup listener to canvas");
            listener.forget();

        }
        {
            let pressed = input.clone();
            let listener = Closure::<dyn FnMut(_)>::new(move |event: MouseEvent| pressed.set(false) );
            browser::canvas()?.add_event_listener_with_callback("mouseleave", listener.as_ref().unchecked_ref()).expect("Could not add mouseleave listener to canvas");
            listener.forget();

        }



        let f: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>> = Rc::new(RefCell::new(None));
        let g = f.clone();

        let mut delta: f64 = 0.0;
        let mut previous_time: f64 = browser::now()?; 

        let animate = Some(browser::create_animation_closure(move |js_delta: f64| {
            let current_time: f64 = browser::now().unwrap();
            delta = (current_time - previous_time) / 1000.0;

            game.update(&delta, &input.get());
            game.draw(&renderer);

            previous_time = current_time;

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
