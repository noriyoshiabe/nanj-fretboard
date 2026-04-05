use image::DynamicImage;
use wasm_bindgen::Clamped;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

pub struct AssetImage {
    image: DynamicImage,
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
}

impl AssetImage {
    pub fn try_new(bytes: &[u8]) -> Result<Self, JsValue> {
        let image = image::load_from_memory(bytes).map_err(|e| e.to_string())?;
        let canvas = web_sys::window().ok_or("window not exists.")?.
            document().ok_or("document not exists.")?.
            create_element("canvas")?.dyn_into::<HtmlCanvasElement>()?;
        let ctx = canvas.get_context("2d")?.ok_or("could not get 2d context.")?.dyn_into::<CanvasRenderingContext2d>()?;

        Ok(Self {
            image,
            canvas,
            ctx,
        })
    }

    pub fn source_canvas(&self, width: f64, height: f64) -> Result<HtmlCanvasElement, JsValue> {
        let resized = self.image.resize(width as u32, height as u32, image::imageops::FilterType::Lanczos3).to_rgba8();
        let (image_width, image_height) = resized.dimensions();
        let image_data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&resized.into_raw()), image_width, image_height)?;

        self.canvas.set_width(image_width);
        self.canvas.set_height(image_height);
        self.ctx.put_image_data(&image_data, 0 as f64, 0 as f64)?;

        Ok(self.canvas.clone())
    }
}

pub struct Asset {
    pub nanj_normal: AssetImage,
}

impl Asset {
    pub fn try_new() -> Result<Self, JsValue> {
        let nanj_normal = AssetImage::try_new(include_bytes!("../assets/normal.png"))?;

        Ok(Self {
            nanj_normal,
        })
    }
}
