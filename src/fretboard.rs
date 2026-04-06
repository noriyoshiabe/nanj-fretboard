use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

use crate::view::{Rect, View};

use crate::asset::Asset;
use crate::nanj::NanJ;
use crate::question::{Question, QuestionEvent, QuestionObserver};

pub struct Fretboard {
    frame: Rect,
    children: Vec<Rc<RefCell<dyn View>>>,
    nanjs: Vec<Rc<RefCell<NanJ>>>,
    asset: Rc<Asset>,
    question: Rc<RefCell<Question>>,
}

impl View for Fretboard {
    fn frame(&self) -> Rect {
        self.frame
    }

    fn set_frame(&mut self, frame: Rect) {
        self.frame = frame
    }

    fn layout(&mut self) -> Result<(), JsValue> {
        let s = self.frame.width / 13. * 0.7;
        let x_offset = self.frame.width / 13. * 0.15;
        let y_offset = -s * 0.5;

        for rc in self.nanjs.iter() {
            let mut nanj = rc.borrow_mut();
            let x = nanj.question_item.fret as f64 * self.frame.width / 13. + x_offset;
            let y = (nanj.question_item.string - 1) as f64 * self.frame.height / 5. + y_offset;
            nanj.set_frame(Rect { x, y, width: s, height: s});
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &CanvasRenderingContext2d, dpr: f64, _: &mut bool) -> Result<(), JsValue> {
        ctx.set_stroke_style_str("lightgray");
        ctx.set_line_width(1.0 * dpr);

        ctx.begin_path();

        for i in 0..6 {
            let y = i as f64 * self.frame.height / 5.;
            ctx.move_to(0., y);
            ctx.line_to(self.frame.width, y);
        }

        for i in 2..14 {
            let x = i as f64 * self.frame.width / 13.;
            ctx.move_to(x, 0.);
            ctx.line_to(x, self.frame.height);
        }

        ctx.stroke();

        ctx.set_stroke_style_str("gray");
        ctx.set_line_width(2.0 * dpr);

        ctx.begin_path();

        let x = 1.0 * self.frame.width / 13.;
        ctx.move_to(x, 0.);
        ctx.line_to(x, self.frame.height);

        ctx.stroke();

        ctx.set_fill_style_str("lightgray");

        let y = self.frame.height / 2.;
        let r = self.frame.width * 0.01;

        for i in [3, 5, 7, 9, 12] {
            let x = (i as f64 * 2. + 1.) * self.frame.width / 26.;

            ctx.begin_path();

            if i == 12 {
                ctx.arc(x, self.frame.height / 10. * 3., r, 0., std::f64::consts::PI * 2.)?;
                ctx.fill();

                ctx.begin_path();
                ctx.arc(x, self.frame.height / 10. * 7., r, 0., std::f64::consts::PI * 2.)?;
            } else {
                ctx.arc(x, y, r, 0., std::f64::consts::PI * 2.)?;
            }

            ctx.fill();
        }

        Ok(())
    }

    fn children(&self) -> &[Rc<RefCell<dyn View>>] {
        self.children.as_slice()
    }
}

impl QuestionObserver for Fretboard {
    fn on_notify_event(&mut self, event: QuestionEvent) -> Result<(), JsValue> {
        if let QuestionEvent::New(question_item) = event {
            let nanj = NanJ::try_new(self.asset.clone(), question_item)?;
            self.children.push(nanj.clone());
            self.nanjs.push(nanj.clone());

            self.question.borrow_mut().add_observer(nanj.clone());

            self.layout()?;
        }

        let gone_nanjs: Vec<Rc<RefCell<NanJ>>> = self.nanjs.iter().filter(|n| n.borrow().is_gone()).cloned().collect();
        for nanj in gone_nanjs.iter() {
            if let Some(index) = self.nanjs.iter().position(|n| Rc::ptr_eq(n, nanj)) {
                self.children.remove(index);
                self.nanjs.remove(index);
            };
        }

        Ok(())
    }
}

impl Fretboard {
    pub fn new(asset: Rc<Asset>, question: Rc<RefCell<Question>>) -> Rc<RefCell<Self>> {
        let children: Vec<Rc<RefCell<dyn View>>> = Vec::new();
        let nanjs: Vec<Rc<RefCell<NanJ>>> = Vec::new();

        Rc::new(RefCell::new(Self {
            frame: Rect::default(),
            children,
            nanjs,
            asset,
            question,
        }))
    }
}
