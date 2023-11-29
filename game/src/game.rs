use crate::{engine::{Game, Renderer, Spritesheet, Rect, self, Position}, browser, physics::World};

use anyhow::Result;
use async_trait::async_trait;
use rapier2d::{geometry::ColliderHandle, dynamics::RigidBodyHandle};
use web_sys::HtmlImageElement;

const CANVAS_WIDTH: f64 = 800.0;
const CANVAS_HEIGHT: f64 = 480.0;

pub trait GameState {
    fn update(&mut self, delta: &f64, input: &bool) -> Option<Box<dyn GameState>>;
    fn draw(&self, renderer: &Renderer, image: &HtmlImageElement, sheet: &Spritesheet);
}

pub struct Waiting;

pub struct GetReady {
    scroll_speed: f64,
    time_elapsed: f64,
}

pub struct Playing {
    plane_frame: f64,
    scroll_speed: f64,
    terrain_offset: f64,
    obstacles: Vec<Position>,
    distance_between_obstacles: f64,

    world: World,
    plane_collider: Option<RigidBodyHandle>
}
pub struct GameOver{
    frames: u8,
}

impl GameState for Waiting {
    fn update(&mut self, delta: &f64, input: &bool) -> Option<Box<dyn GameState>>{
        if *input {
            Some(Box::new(GetReady{ scroll_speed: 1.0, time_elapsed: 0.0}))
        } else {
            None
        }
    }
    
    fn draw(&self, renderer: &Renderer, image: &HtmlImageElement, sheet: &Spritesheet){
        clear_canvas(renderer);

        let plane_sprite = sheet.tileset.get("planeRed1.png").unwrap();

        let h_pos = CANVAS_WIDTH/2.0 - (plane_sprite.width as f64)/2.0;
        let v_pos = CANVAS_HEIGHT/2.0 - (plane_sprite.height as f64)/2.0;

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
            Some(
                Box::new(
                    Playing{
                        plane_frame: 1.0,
                        scroll_speed: self.scroll_speed, 
                        terrain_offset: 0.0,
                        obstacles: Vec::new(), 
                        distance_between_obstacles: 400.0,
                        world: World::default(),
                        plane_collider: None,
                    }
                )
            )
            // Some(Box::new(GameOver))
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

        let start_pos = CANVAS_WIDTH/2.0 - (plane_sprite.width as f64)/2.0;
        let end_pos = plane_sprite.width as f64;

        let h_pos = start_pos - ((start_pos - end_pos) * self.time_elapsed) / 4.0;
        let v_pos = CANVAS_HEIGHT/2.0 - (plane_sprite.height as f64)/2.0;

        draw_background(sheet, image, renderer);
        draw_limits(0, sheet, image, renderer);
        
        draw_plane("Red", &1_u16, &Position { x: h_pos, y: v_pos }, sheet, image, renderer);

        renderer.draw_image(
            image, 
            sprite, 
            &Rect { 
                x: CANVAS_WIDTH as i32/2 - sprite.width/2, 
                y: CANVAS_HEIGHT as i32/2 - sprite.height/2, 
                width: sprite.width, 
                height: sprite.height 
            }
        );
    }
}

impl GameState for Playing {
    fn update(&mut self, delta: &f64, input: &bool) -> Option<Box<dyn GameState>>{
        self.world.update();

        if self.plane_collider.is_none() {
            self.world.add_collider(&Rect { 
                x: 0, 
                y: 480 - 71, 
                width: 808,
                height: 71 
            });
            self.world.add_collider(&Rect{
                x: 0,
                y: 0,
                width: 808,
                height: 71,
            });

            let handle = self.world.add_plane(
                &Rect{ 
                    x: 88, 
                    y: (CANVAS_HEIGHT/2.0 - 73.0/2.0) as i32, 
                    width: 88, 
                    height: 73
                }
            );

            self.plane_collider = Some(handle);
        }

        if *input {
            self.world.add_impulse(self.plane_collider.as_ref().unwrap(), -50_000.0);
        }

        let plane_rotation_speed = 60.0 / 3.0;
        self.plane_frame += delta * plane_rotation_speed;
        self.plane_frame = self.plane_frame % 3.0;

        self.terrain_offset -= delta * 100.0 * self.scroll_speed ;
        self.terrain_offset = self.terrain_offset % 808.0;

        for pos in self.obstacles.iter_mut() {
            pos.x -= delta * 100.0 * self.scroll_speed;  
        }

        self.obstacles.retain_mut(|pos| pos.x > -200.0);
        
        if self.obstacles.len() == 0 {
            self.obstacles.push(create_obstacle(CANVAS_WIDTH, 0.0));
        } else {
            let last_obstacle = self.obstacles.last().unwrap();
            if last_obstacle.x <= CANVAS_WIDTH - self.distance_between_obstacles {
                self.obstacles.push(create_obstacle(last_obstacle.x + self.distance_between_obstacles, 200.0));
            }
        }

        if let Some(handle) = self.plane_collider.as_ref() {
            let pos = self.world.get_body_position(handle);
            if pos.y - 71.0/2.0 < 71.0 || pos.y + 71.0/2.0 > CANVAS_HEIGHT - 71.0 {
                Some(Box::new(GameOver{frames: 0}))
            } else {
                None
            }
        } else {
            None
        }
    }
    
    fn draw(&self, renderer: &Renderer, image: &HtmlImageElement, sheet: &Spritesheet){
        clear_canvas(renderer);
        
        let plane_sprite = sheet.tileset.get("planeRed1.png").unwrap();

        let h_pos = plane_sprite.width as f64;
        let v_pos = CANVAS_HEIGHT/2.0 - (plane_sprite.height as f64)/2.0;

        draw_background(sheet, image, renderer);

        if let Some(handle) = self.plane_collider.as_ref() {
            let pos = self.world.get_body_position(handle);
            draw_plane(
                "Red", 
                &(self.plane_frame as u16 + 1), 
                &Position{x: pos.x - plane_sprite.width as f64/2.0, y: pos.y - plane_sprite.height as f64/2.0}, 
                sheet, 
                image, 
                renderer
            );
        }
        draw_obstacles(
            &self.obstacles, 
            sheet, 
            image, 
            renderer
        );
        draw_limits(self.terrain_offset as i32, sheet, image, renderer);
    }
}

impl GameState for GameOver {
    fn update(&mut self, delta: &f64, input: &bool) -> Option<Box<dyn GameState>>{
        self.frames += if self.frames > 1 { 0 } else { 1 };

        if *input {
            Some(
                Box::new(
                    GetReady{
                        scroll_speed: 1.0, 
                        time_elapsed: 0.0
                    }
                )
            )
        } else {
            None
        }
    }

    fn draw(&self, renderer: &Renderer, image: &HtmlImageElement, sheet: &Spritesheet){
        if self.frames < 1 {
            let sprite = sheet.tileset.get("textGameOver.png").unwrap();

            renderer.draw_image(
                image, 
                sprite, 
                &Rect { 
                    x: CANVAS_WIDTH as i32/2 - sprite.width/2, 
                    y: CANVAS_HEIGHT as i32/2 - sprite.height/2, 
                    width: sprite.width, 
                    height: sprite.height 
                }
            );
        }
    }
}

pub struct TappyPlane{
    pub image: Option<HtmlImageElement>,
    pub sheet: Option<Spritesheet>,
    pub state: Box<dyn GameState>,
}

impl Default for TappyPlane {
    fn default() -> Self {
        TappyPlane { 
            image: None, 
            sheet: None, 
            state: Box::new(Waiting),
        }
    }
}

#[async_trait(?Send)]
impl Game for TappyPlane {
    async fn init(&self) -> Result<Box<dyn Game>> {
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
        width: CANVAS_WIDTH as i32,
        height: CANVAS_HEIGHT as i32
    };
    renderer.clear(&clear_area);
}

fn draw_background(sheet: &Spritesheet, image: &HtmlImageElement, renderer: &Renderer) {
    let background: &Rect = sheet.tileset.get("background.png").as_ref().unwrap();
    renderer.draw_image(
        &image, 
        background, 
        &Rect { 
            x: 0, 
            y: 0, 
            width: CANVAS_WIDTH as i32, 
            height: CANVAS_HEIGHT as i32 
        }
    );
}

fn draw_limits(offset: i32, sheet: &Spritesheet, image: &HtmlImageElement, renderer: &Renderer) {
    let terrain_above: &Rect = sheet.tileset.get("groundDirt.png").as_ref().unwrap();
    let terrain_below: &Rect = sheet.tileset.get("groundGrass.png").as_ref().unwrap();
    
    renderer.draw_image(
        &image, 
        terrain_below,
        &Rect{
            x: offset, 
            y: CANVAS_HEIGHT as i32 - terrain_below.height,
            width: terrain_below.width,
            height: terrain_below.height
        }
    );

    renderer.draw_image(
        &image, 
        terrain_below, 
        &Rect{
            x: offset + terrain_below.width, 
            y: CANVAS_HEIGHT as i32 -terrain_below.height,
            width: terrain_below.width,
            height: terrain_below.height
        }
    );

    renderer.context.save();
    renderer.context.translate(terrain_above.width as f64, terrain_above.height as f64);
    renderer.context.rotate(std::f64::consts::PI);

    renderer.draw_image(
        &image,
        terrain_above, 
        &Rect{
            x: -offset, 
            y: 0,
            width:terrain_above.width,
            height: terrain_above.height,
        }
    );
    renderer.draw_image(
        &image,
        terrain_above, 
        &Rect{
            x: -offset - terrain_above.width as i32, 
            y: 0,
            width: terrain_above.width,
            height: terrain_above.height,
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

fn create_obstacle(min_x: f64, max_offset: f64)-> Position {
    Position{
        x: min_x + (js_sys::Math::random() * max_offset).floor(), 
        y: if js_sys::Math::random() > 0.5 { CANVAS_HEIGHT } else { 0.0 },
    }
}

fn draw_obstacles(obstacles: &Vec<Position>, sheet: &Spritesheet, image: &HtmlImageElement, renderer: &Renderer) {
    for pos in obstacles.iter() {
        let sprite = sheet
            .tileset
            .get(
                if pos.y > 0.0 {
                    "rock.png"
                } else {
                    "rockDown.png"
                }
            ).unwrap();
        
        renderer.draw_image(
            image, 
            sprite, 
            &Rect{ 
                x: pos.x.floor() as i32, 
                y: (if pos.y > 0.0 { pos.y } else { pos.y + sprite.height as f64 } - sprite.height as f64).floor() as i32, 
                width: sprite.width, 
                height: sprite.height 
            }
        );
    }
}

