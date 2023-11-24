use crate::{engine::{Game, Renderer, Spritesheet, Rect, self}, browser};
use anyhow::Result;
use async_trait::async_trait;
use wasm_bindgen::JsValue;
use web_sys::{HtmlImageElement, console};

pub trait GameState {
    fn update(&mut self, input: &bool) -> Option<Box<dyn GameState>>;
    fn draw(&self, renderer: &Renderer, image: &Option<HtmlImageElement>, sheet: &Option<Spritesheet>);
}

pub struct Waitting;

pub struct GetReady {
    frame: u16,
}

pub struct Playing;
pub struct GameOver;

impl GameState for Waitting {
    fn update(&mut self, input: &bool) -> Option<Box<dyn GameState>>{
        if *input {
            Some(Box::new(GetReady{ frame: 0}))
        } else {
            None
        }
    }
    
    fn draw(&self, renderer: &Renderer, image: &Option<HtmlImageElement>, sheet: &Option<Spritesheet>){
        if let Some(sheet) = sheet {
            let plane_sprite = sheet.tileset.get("planeRed1.png").unwrap();
            
            let tap_left_sprite = sheet.tileset
                .get("tapLeft.png")
                .unwrap();
            let tap_right_sprite = sheet.tileset
                .get("tapRight.png")
                .unwrap();

            let offset = 4;

            image.as_ref().map(|image| {
                renderer.draw_image(
                    image, 
                    tap_right_sprite, 
                    &Rect { 
                        x: plane_sprite.width*2 - tap_right_sprite.width - offset, 
                        y: 480/2 - tap_right_sprite.height/2, 
                        width: tap_right_sprite.width, 
                        height: tap_right_sprite.height 
                    }
                );
            });

            image.as_ref().map(|image| {
                renderer.draw_image(
                    image, 
                    tap_left_sprite, 
                    &Rect { 
                        x: plane_sprite.width*2 + tap_left_sprite.width + offset, 
                        y: 480/2 - tap_left_sprite.height/2, 
                        width: tap_left_sprite.width, 
                        height: tap_left_sprite.height 
                    }
                );
            });


        }
    }
}

impl GameState for GetReady {
    fn update(&mut self, input: &bool) -> Option<Box<dyn GameState>>{
        self.frame += 1;

        if self.frame >= 60*4 {
            Some(Box::new(Playing))
        } else {
            None
        }
    }
    
    fn draw(&self, renderer: &Renderer, image: &Option<HtmlImageElement>, sheet: &Option<Spritesheet>){
        if let Some(sheet) = sheet {
            let index: usize = usize::from(self.frame / 60);
            let sprite_name: Vec<String> = vec![
                String::from("textGetReady.png"), 
                String::from("number3.png"), 
                String::from("number2.png"), 
                String::from("number1.png")
            ];

            let sprite = sheet.tileset
                .get(sprite_name.get(index).unwrap())
                .unwrap();

            image.as_ref().map(|image| {
                renderer.draw_image(
                    image, 
                    sprite, 
                    &Rect { 
                        x: 800/2 - sprite.width/2, 
                        y: 480/2 - sprite.height/2, 
                        width: sprite.width, 
                        height: sprite.height 
                    }
                );
            });
        }
    }
}

impl GameState for Playing {
    fn update(&mut self, input: &bool) -> Option<Box<dyn GameState>>{
        None
    }
    
    fn draw(&self, renderer: &Renderer, image: &Option<HtmlImageElement>, sheet: &Option<Spritesheet>){
    }
}

impl GameState for GameOver {
    fn update(&mut self, input: &bool) -> Option<Box<dyn GameState>>{
        None
    }

    fn draw(&self, renderer: &Renderer, image: &Option<HtmlImageElement>, sheet: &Option<Spritesheet>){
    }
}

pub struct TappyPlane{
    pub image: Option<HtmlImageElement>,
    pub sheet: Option<Spritesheet>,
    pub state: Box<dyn GameState>,
    pub frame: u32,
}

#[async_trait(?Send)]
impl Game for TappyPlane {
    async fn initialize(&self) -> Result<Box<dyn Game>> {
        let sheet: Spritesheet = serde_wasm_bindgen::from_value(
            browser::fetch_json("/assets/sheet.json")
            .await?
        ).unwrap();

        let image = engine::load_image(&sheet.image).await?;

        Ok(
            Box::new(
                TappyPlane{
                    image: Some(image),
                    sheet: Some(sheet),
                    state: Box::new(Waitting),
                    frame: self.frame
                }
            )
        )
    }

    fn update(&mut self, input: &bool){
        match self.state.update(input) {
            Some(new_state) => self.state = new_state,
            None => ()
        };

        self.frame = if self.frame < (12-1) { self.frame + 1 } else { 0 }; 
    }

    fn draw(&self, renderer: &Renderer) {

        let frame = self.frame;
        if let Some(sheet) = self.sheet.as_ref() {
            let background: &Rect = sheet.tileset.get("background.png").as_ref().unwrap();
            let floor: &Rect = sheet.tileset.get("groundGrass.png").as_ref().unwrap();
            let ceilling: &Rect = sheet.tileset.get("groundDirt.png").as_ref().unwrap();
            let obstacle: &Rect = sheet.tileset.get("rock.png").as_ref().unwrap();
            let plane_tile: &Rect = sheet.tileset.get(format!("planeRed{}.png", frame/4 + 1 ).as_str()).as_ref().unwrap();

            let clear_area = Rect{
                x: 0,
                y: 0,
                width: 800,
                height: 480
            };

            renderer.clear(&clear_area);

            self.image.as_ref().map(|image| {
                renderer.draw_image(&image, background, &clear_area);
                renderer.draw_image(
                    &image, 
                    floor, 
                    &Rect{
                        x: 0, 
                        y: 480-floor.height,
                        width: floor.width,
                        height: floor.height
                    }
                );
                
                renderer.context.save();
                renderer.context.translate(ceilling.width as f64, 0.0);
                renderer.context.rotate(std::f64::consts::PI);

                renderer.draw_image(
                    &image,
                    ceilling, 
                    &Rect{
                        x: 0,
                        y: -ceilling.height,
                        width: ceilling.width,
                        height: ceilling.height,
                    }
                );

                renderer.context.restore();

                
                renderer.draw_image(
                    &image,
                    plane_tile,
                    &Rect{
                        x: plane_tile.width*2,
                        y: 480/2 - plane_tile.height/2,
                        width: plane_tile.width,
                        height: plane_tile.height
                    }
                );
            });

            self.state.draw(renderer, &self.image, &self.sheet);
        }
    }
}


