use std::{rc::Rc, cell::RefCell};

use anyhow::{anyhow, Result, Ok};
use web_sys::{HtmlImageElement, CanvasRenderingContext2d};

use crate::browser;

const FRAMES_PER_SECOND: f32 = 60.0;
const FRAME_RATE: f32 = 1.0 / FRAMES_PER_SECOND;
const FRAME_RATE_IN_MILLISECONDS: f32 = FRAME_RATE * 1000.0;

pub trait Game {
    fn update(&mut self);
    fn draw(&self, context: &CanvasRenderingContext2d);
}

pub struct GameLoop{
    delta: f32,
    elapsed_time: f32,
}
impl GameLoop {
    pub async fn start(mut game: impl Game + 'static) -> Result<()> {
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        let animate = Some(browser::create_animation_closure(move |delta| {
            game.update();
            game.draw(context)
            browser::request_animation_frame(f.borrow().as_ref().unwrap());
        }));

        *g.borrow_mut() = animate
        browser::request_animation_frame(g.borrow().as_ref().unwrap())?;

        Ok(())
    }
}

pub async fn load_image(source: &str) -> Result<HtmlImageElement> {
    let image = browser::new_image()?;
    Ok(image)
}
