use wasm_bindgen::prelude::*;

mod app;
mod dispatcher;
mod runtime;
mod util;
mod view;

mod asset;
mod accidental;
mod fretboard;
mod keyboard;
mod question;
mod nanj;
mod root_view;
mod task_queue;

use crate::app::App;
use crate::runtime::Runtime;

#[wasm_bindgen]
pub fn run(canvas_id: &str) -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let canvas = Runtime::init_canvas(canvas_id)?;
    let app = App::try_new(canvas.clone())?;

    Runtime::run(canvas, app)?;

    Ok(())
}
