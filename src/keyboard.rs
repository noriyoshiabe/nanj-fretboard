use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

use crate::view::{Rect, View, Point};

pub struct Keyboard {
    frame: Rect,
    children: Vec<Rc<RefCell<dyn View>>>,
    normal_pads: Vec<Rc<RefCell<NotePad>>>,
    accidental_pads: Vec<Rc<RefCell<NotePad>>>,
}

impl View for Keyboard {
    fn frame(&self) -> Rect {
        self.frame
    }

    fn set_frame(&mut self, frame: Rect) {
        self.frame = frame
    }

    fn layout(&mut self) {
        let s = self.frame.width / 14. * 2.;

        for (i, note_pad) in self.normal_pads.iter().enumerate() {
            let x = (i as f64 * 2.) * self.frame.width / 14.;
            let y = s * 0.9;
            note_pad.borrow_mut().set_frame(Rect { x, y, width: s, height: s});
        }

        for (i, note_pad) in self.accidental_pads.iter().enumerate() {
            let x = if i < 2 {
                (i as f64 * 2. + 1.) * self.frame.width / 14.
            } else {
                (i as f64 * 2. + 3.) * self.frame.width / 14.
            };
            let y = 0.0;
            note_pad.borrow_mut().set_frame(Rect { x, y, width: s, height: s});
        }
    }

    fn draw(&mut self, _: &CanvasRenderingContext2d, _: f64, _: &mut bool) -> Result<(), JsValue> {
        Ok(())
    }

    fn children(&self) -> &[Rc<RefCell<dyn View>>] {
        self.children.as_slice()
    }
}

impl Keyboard {
    pub fn new() -> Self {
        let mut children: Vec<Rc<RefCell<dyn View>>> = Vec::new();
        let mut normal_pads: Vec<Rc<RefCell<NotePad>>> = Vec::new();
        let mut accidental_pads: Vec<Rc<RefCell<NotePad>>> = Vec::new();

        let pitches = [
            "C",
            "D",
            "E",
            "F",
            "G",
            "A",
            "B",
        ];

        for pitch in pitches {
            let note_pad = Rc::new(RefCell::new(NotePad::new(pitch.to_string(), false)));
            normal_pads.push(note_pad.clone());
            children.push(note_pad);
        }

        let pitches = [
            "C#",
            "D#",
            "F#",
            "G#",
            "A#",
        ];

        for pitch in pitches {
            let note_pad = Rc::new(RefCell::new(NotePad::new(pitch.to_string(), true)));
            accidental_pads.push(note_pad.clone());
            children.push(note_pad);
        }

        Self {
            frame: Rect::default(),
            children,
            normal_pads,
            accidental_pads,
        }
    }
}

pub struct NotePad {
    frame: Rect,
    #[allow(unused)]
    pitch: String,
    accidental: bool,
    active: bool, 
}

impl View for NotePad {
    fn frame(&self) -> Rect {
        self.frame
    }

    fn set_frame(&mut self, frame: Rect) {
        self.frame = frame
    }

    fn layout(&mut self) {
    }

    fn draw(&mut self, ctx: &CanvasRenderingContext2d, dpr: f64, _: &mut bool) -> Result<(), JsValue> {
        ctx.set_line_width(2.0 * dpr);

        if self.active {
           ctx.set_global_alpha(0.5);
        }

        if self.accidental {
            ctx.set_stroke_style_str("#444");
            ctx.set_fill_style_str("#444");
        } else {
            ctx.set_stroke_style_str("lightgray");
            ctx.set_fill_style_str("white");
        }

        let c = self.frame.width / 2.;
        let r = c * 0.9;

        ctx.begin_path();
        ctx.arc(c, c, r, 0., std::f64::consts::PI * 2.)?;
        ctx.fill();
        ctx.stroke();

        Ok(())
    }

    fn pointer_down(&mut self, _: Point, _: &mut bool) -> bool {
        self.active = true;
        true
    }

    fn pointer_up(&mut self, _: Point, _: &mut bool) {
        self.active = false
    }
}

impl NotePad {
    pub fn new(pitch: String, accidental: bool) -> Self {
        Self {
            frame: Rect::default(),
            pitch,
            accidental,
            active: false,
        }
    }
}
