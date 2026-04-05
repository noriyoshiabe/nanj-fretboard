use image::GenericImageView;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

pub struct Asset {
    pub nanj_normal: HtmlCanvasElement,
    pub nanj_happy: HtmlCanvasElement,
    pub nanj_angry: HtmlCanvasElement,
}

impl Asset {
    pub fn try_new() -> Result<Self, JsValue> {
        let nanj_normal = source_canvas(include_bytes!("../assets/normal.png"))?;
        let nanj_happy = source_canvas(include_bytes!("../assets/happy.png"))?;
        let nanj_angry = source_canvas(include_bytes!("../assets/angry.png"))?;

        Ok(Self {
            nanj_normal,
            nanj_happy,
            nanj_angry,
        })
    }
}

fn source_canvas(bytes: &[u8]) -> Result<HtmlCanvasElement, JsValue> {
    let image = image::load_from_memory(bytes).map_err(|e| e.to_string())?;
    let (width, height) = image.dimensions();
    
    let canvas = web_sys::window().ok_or("window not exists.")?.
        document().ok_or("document not exists.")?.
        create_element("canvas")?.dyn_into::<HtmlCanvasElement>()?;
    let ctx = canvas.get_context("2d")?.ok_or("could not get 2d context.")?.dyn_into::<CanvasRenderingContext2d>()?;

    canvas.set_width(width);
    canvas.set_height(height);

    let data = ImageData::new_with_u8_clamped_array_and_sh(
        wasm_bindgen::Clamped(&image.to_rgba8().into_raw()),
        width,
        height,
    )?;

    ctx.put_image_data(&data, 0., 0.)?;

    Ok(canvas)
}
