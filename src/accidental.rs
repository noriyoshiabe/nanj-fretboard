use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

use crate::view::{Rect, View, Point};

use crate::question::{Question, QuestionEvent, QuestionObserver};

pub struct Accidental {
    frame: Rect,
    question: Rc<RefCell<Question>>,
    active: bool,
}

impl View for Accidental {
    fn frame(&self) -> Rect {
        self.frame
    }

    fn set_frame(&mut self, frame: Rect) {
        self.frame = frame
    }

    fn draw(&mut self, ctx: &CanvasRenderingContext2d, dpr: f64, _: &mut bool) -> Result<(), JsValue> {
        ctx.set_stroke_style_str("gray");
        ctx.set_line_width(2.0 * dpr);

        if !self.active {
           ctx.set_global_alpha(0.3);
        }

        ctx.begin_path();

        let bounds = self.bounds();

        ctx.move_to(bounds.left(), bounds.top());

        ctx.line_to(bounds.right(), bounds.top());
        ctx.line_to(bounds.right(), bounds.bottom());
        ctx.line_to(bounds.left(), bounds.bottom());
        ctx.line_to(bounds.left(), bounds.top());

        ctx.stroke();

        ctx.set_font(&format!("{}px system-ui", dpr * 16.));

        let metrics = ctx.measure_text("#/♭")?;
        let width = metrics.width();
        let height = metrics.actual_bounding_box_ascent();
        let x = (self.frame.width - width) / 2.;
        let y = (self.frame.height - height) / 2. + height;

        ctx.fill_text("#/♭", x, y)?;

        Ok(())
    }

    fn pointer_down(&mut self, _: Point) -> Result<bool, JsValue> {
        self.question.borrow_mut().toggle_accidental();
        Ok(true)
    }
}

impl QuestionObserver for Accidental {
    fn on_notify_event(&mut self, event: QuestionEvent) -> Result<(), JsValue> {
        if let QuestionEvent::AccidentalChange { accidental } = event {
            self.active = accidental;
        }

        Ok(())
    }
}

impl Accidental {
    pub fn new(question: Rc<RefCell<Question>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            frame: Rect::default(),
            question,
            active: false,
        }))
    }
}
