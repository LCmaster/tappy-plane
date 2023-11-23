use crate::{engine::{Game, Renderer, Spritesheet, Rect, self, FRAME_RATE, FRAMES_PER_SECOND}, browser};
use anyhow::Result;
use async_trait::async_trait;
use web_sys::HtmlImageElement;

pub struct TappyPlane{
    pub image: Option<HtmlImageElement>,
    pub sheet: Option<Spritesheet>,
    pub frame: u8,
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
                    frame: self.frame
                }
            )
        )
    }

    fn update(&mut self){
        self.frame = if self.frame < (12-1) { self.frame + 1 } else { 0 }; }

    fn draw(&self, renderer: &Renderer) {
        let frame = self.frame;
        if let Some(sheet) = self.sheet.as_ref() {
            let background: &Rect = sheet.tileset.get("background.png").as_ref().unwrap();
            let plane_tile: &Rect = sheet.tileset.get(format!("planeRed{}.png", frame/4 + 1 ).as_str()).as_ref().unwrap();

            let clear_area = Rect{
                x: 0,
                y: 0,
                width: 640,
                height: 480
            };

            renderer.clear(&clear_area);

            self.image.as_ref().map(|image| {
                renderer.draw_image(&image, background, &clear_area);
                renderer.draw_image(
                    &image,
                    plane_tile,
                    &Rect {
                        x: plane_tile.width*2,
                        y: (480/2 - plane_tile.height/2),
                        width: plane_tile.width,
                        height: plane_tile.height
                    }
                );
            });

        }
    }
}


