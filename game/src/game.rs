use crate::{engine::{Game, Renderer, Spritesheet, Rect, self, Position}, browser};
use anyhow::Result;
use async_trait::async_trait;
use wasm_bindgen::JsValue;
use web_sys::{HtmlImageElement, console};

pub trait GameState {
    fn update(&mut self, delta: &f64, input: &bool) -> Option<Box<dyn GameState>>;
    fn draw(&self, renderer: &Renderer, image: &HtmlImageElement, sheet: &Spritesheet);
}

pub struct Waiting;

pub struct GetReady {
    time_elapsed: f64,
}

pub struct Playing {
    frame: u16,
    obstacles: Vec<Position>
}
pub struct GameOver;

impl GameState for Waiting {
    fn update(&mut self, delta: &f64, input: &bool) -> Option<Box<dyn GameState>>{
        if *input {
            Some(Box::new(GetReady{ time_elapsed: 0.0}))
        } else {
            None
        }
    }
    
    fn draw(&self, renderer: &Renderer, image: &HtmlImageElement, sheet: &Spritesheet){
        clear_canvas(renderer);

        let plane_sprite = sheet.tileset.get("planeRed1.png").unwrap();

        let h_pos = 800.0/2.0 - (plane_sprite.width as f64)/2.0;
        let v_pos = 480.0/2.0 - (plane_sprite.height as f64)/2.0;

        draw_background(sheet, image, renderer);
        draw_limits(0, sheet, image, renderer);
        draw_plane("Red", &1_u16, &Position { x: h_pos, y: v_pos }, sheet, image, renderer);
            
        let tap_left_sprite = sheet.tileset
            .get("tapLeft.png")
            .unwrap();
        let tap_right_sprite = sheet.tileset
            .get("tapRight.png")
            .unwrap();
        let offset = 8;

        renderer.draw_image(
            image, 
            tap_right_sprite, 
            &Rect { 
                x: h_pos as i32 - tap_right_sprite.width - offset, 
                y: 480/2 - tap_right_sprite.height/2, 
                width: tap_right_sprite.width, 
                height: tap_right_sprite.height 
            }
        );

        renderer.draw_image(
            image, 
            tap_left_sprite, 
            &Rect { 
                x: h_pos as i32 + tap_left_sprite.width + offset, 
                y: 480/2 - tap_left_sprite.height/2, 
                width: tap_left_sprite.width, 
                height: tap_left_sprite.height 
            }
        );
    }
}

impl GameState for GetReady {
    fn update(&mut self, delta: &f64, input: &bool) -> Option<Box<dyn GameState>>{
        self.time_elapsed += delta;

        if self.time_elapsed as u16 >= 4 {
            // Some(Box::new(Playing{frame: 0, obstacles: Vec::new()}))
            Some(Box::new(GameOver))
        } else {
            None
        }
    }
    
    fn draw(&self, renderer: &Renderer, image: &HtmlImageElement, sheet: &Spritesheet){
        clear_canvas(renderer);
        
        let index: usize = usize::from(self.time_elapsed as u16);
        let sprite_name: Vec<String> = vec![
            String::from("textGetReady.png"), 
            String::from("number3.png"), 
            String::from("number2.png"), 
            String::from("number1.png")
        ];

        let sprite = sheet.tileset
            .get(sprite_name.get(index).unwrap())
            .unwrap();

        let plane_sprite = sheet.tileset.get("planeRed1.png").unwrap();

        let start_pos = 800.0/2.0 - (plane_sprite.width as f64)/2.0;
        let end_pos = plane_sprite.width as f64;

        let h_pos = start_pos - ((start_pos - end_pos) * self.time_elapsed) / 4.0;
        let v_pos = 480.0/2.0 - (plane_sprite.height as f64)/2.0;

        draw_background(sheet, image, renderer);
        draw_limits(0, sheet, image, renderer);
        
        draw_plane("Red", &1_u16, &Position { x: h_pos, y: v_pos }, sheet, image, renderer);

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
    }
}

impl GameState for Playing {
    fn update(&mut self, delta: &f64, input: &bool) -> Option<Box<dyn GameState>>{
        None
    }
    
    fn draw(&self, renderer: &Renderer, image: &HtmlImageElement, sheet: &Spritesheet){
        clear_canvas(renderer);
        
        let plane_sprite = sheet.tileset.get("planeRed1.png").unwrap();

        let h_pos = plane_sprite.width as f64;
        let v_pos = 480.0/2.0 - (plane_sprite.height as f64)/2.0;

        draw_background(sheet, image, renderer);
        draw_limits(0, sheet, image, renderer);
        draw_plane("Red", &1_u16, &Position{x: h_pos, y: v_pos}, sheet, image, renderer);
    }
}

impl GameState for GameOver {
    fn update(&mut self, delta: &f64, input: &bool) -> Option<Box<dyn GameState>>{
        if *input {
            Some(Box::new(GetReady{ time_elapsed: 0.0}))
        } else {
            None
        }
    }

    fn draw(&self, renderer: &Renderer, image: &HtmlImageElement, sheet: &Spritesheet){
        let sprite = sheet.tileset.get("textGameOver.png").unwrap();

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
    }
}

pub struct TappyPlane{
    pub image: Option<HtmlImageElement>,
    pub sheet: Option<Spritesheet>,
    pub state: Box<dyn GameState>,
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
                    state: Box::new(Waiting),
                }
            )
        )
    }

    fn update(&mut self, delta: &f64, input: &bool){
        match self.state.update(delta, input) {
            Some(new_state) => self.state = new_state,
            None => ()
        };
    }

    fn draw(&self, renderer: &Renderer) {
        if let Some(sheet) = self.sheet.as_ref() {
            self.image.as_ref().map(|image| {
                self.state.draw(renderer, image, sheet);
            });
        }
    }
}

fn clear_canvas(renderer: &Renderer) {
    let clear_area = Rect{
        x: 0,
        y: 0,
        width: 800,
        height: 480
    };
    renderer.clear(&clear_area);
}

fn draw_background(sheet: &Spritesheet, image: &HtmlImageElement, renderer: &Renderer) {
    let background: &Rect = sheet.tileset.get("background.png").as_ref().unwrap();
    renderer.draw_image(&image, background, &Rect { x: 0, y: 0, width: 800, height: 480 });
}

fn draw_limits(offset: i32, sheet: &Spritesheet, image: &HtmlImageElement, renderer: &Renderer) {
    let floor: &Rect = sheet.tileset.get("groundGrass.png").as_ref().unwrap();
    let ceilling: &Rect = sheet.tileset.get("groundDirt.png").as_ref().unwrap();
    
    renderer.draw_image(
        &image, 
        floor,
        &Rect{
            x: 0 - offset, 
            y: 480-floor.height,
            width: floor.width,
            height: floor.height
        }
    );

    renderer.draw_image(
        &image, 
        floor, 
        &Rect{
            x: 0 - offset + floor.width, 
            y: 480-floor.height,
            width: floor.width,
            height: floor.height
        }
    );

    renderer.context.save();
    renderer.context.translate(ceilling.width as f64, ceilling.height as f64);
    renderer.context.rotate(std::f64::consts::PI);

    renderer.draw_image(
        &image,
        ceilling, 
        &Rect{
            x: 0 + offset, 
            y: 0,
            width: ceilling.width,
            height: ceilling.height,
        }
    );
    renderer.draw_image(
        &image,
        ceilling, 
        &Rect{
            x: 0 + offset - ceilling.width as i32, 
            y: 0,
            width: ceilling.width,
            height: ceilling.height,
        }
    );

    renderer.context.restore();
}

fn draw_plane(color: &str, sprite_number: &u16, position: &Position, sheet: &Spritesheet, image: &HtmlImageElement, renderer: &Renderer) {
    let plane_tile: &Rect = sheet.tileset.get(format!("plane{}{}.png", color, sprite_number ).as_str()).as_ref().unwrap();

    renderer
        .draw_image(
            &image,
            plane_tile,
            &Rect{
                x: position.x as i32,
                y: position.y as i32,
                width: plane_tile.width,
                height: plane_tile.height
            }
        );
}

fn draw_obstacle() {}

