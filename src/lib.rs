use wasm_bindgen::prelude::*;

mod app;
mod dispatcher;
mod root_view;
mod runtime;
mod util;
mod view;

use crate::app::App;
use crate::runtime::Runtime;

#[wasm_bindgen]
pub fn run(canvas_id: &str) -> Result<(), JsValue> {
    let canvas = Runtime::init_canvas(canvas_id)?;
    let app = App::new(canvas.clone())?;

    Runtime::run(canvas, app)?;

    Ok(())
}
