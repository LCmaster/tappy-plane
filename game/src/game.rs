use crate::{engine::{Game, Renderer, Sheet, Rect, self, FRAME_RATE, FRAMES_PER_SECOND}, browser};
use anyhow::Result;
use async_trait::async_trait;
use web_sys::HtmlImageElement;

pub struct TappyPlane{
    pub image: Option<HtmlImageElement>,
    pub sheet: Option<Sheet>,
    pub frame: u8,
}

#[async_trait(?Send)]
impl Game for TappyPlane {
    async fn initialize(&self) -> Result<Box<dyn Game>> {
        let sheet: Sheet = serde_wasm_bindgen::from_value(
            browser::fetch_json("/assets/Spritesheet/planes.json")
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
        self.frame = if self.frame < (60-1) { self.frame + 1 } else { 0 }; }

    fn draw(&self, renderer: &Renderer) {
        let frame = self.frame;
        if let Some(sheet) = self.sheet.as_ref() {
            if let Some(image) = self.image.as_ref() {
                if let Some(tile) = sheet.tileset.get(usize::from(6 + (frame/20))).as_ref() {
                    let sprite = &tile.rect;
                    let clear_area = Rect{
                        x: 0,
                        y: 0,
                        width: 640,
                        height: 480
                    };
                    renderer.clear(&clear_area);

                    renderer.draw_image(
                        &image,
                        &sprite,
                        &Rect {
                            x: 0,
                            y: 0,
                            width: sprite.width,
                            height: sprite.height
                        }
                    );
                }
            }
        }

        
    }
}


