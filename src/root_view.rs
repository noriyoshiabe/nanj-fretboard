use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

use crate::view::{Rect, View};
use crate::asset::Asset;
use crate::accidental::Accidental;
use crate::fretboard::Fretboard;
use crate::keyboard::Keyboard;

pub struct RootView {
    frame: Rect,
    children: Vec<Rc<RefCell<dyn View>>>,
    accidental: Rc<RefCell<Accidental>>,
    fretboard: Rc<RefCell<Fretboard>>,
    keyboard: Rc<RefCell<Keyboard>>,
}

impl View for RootView {
    fn frame(&self) -> Rect {
        self.frame
    }

    fn set_frame(&mut self, frame: Rect) {
        self.frame = frame
    }

    fn layout(&mut self) -> Result<(), JsValue> {
        let width = (self.frame.width * 0.25).min(self.frame.height * 0.15);
        let height = width / (10. / 3.);
        let x = (self.frame.width - width) / 2.;

        let mut f_acccidental = Rect { x, y: 0., width, height };

        let h_margin = self.frame.width * 0.025;
        let width = self.frame.width - h_margin * 2.;
        let height = width / (24. / 4.);
        let x = h_margin;

        let mut f_fretboard = Rect { x, y: 0., width, height };

        let h_margin = self.frame.width * 0.05;
        let width = self.frame.width - h_margin * 2.;
        let height = width / (14. / (4. * 0.95));
        let x = h_margin;

        let mut f_keyboard = Rect { x, y: 0., width, height };

        let empty_height = self.frame.height - f_acccidental.height - f_fretboard.height - f_keyboard.height;
        let v_space = empty_height / 4.;

        f_acccidental.y = v_space;
        f_fretboard.y = f_acccidental.bottom() + v_space;
        f_keyboard.y = f_fretboard.bottom() + v_space;
                          
        self.accidental.borrow_mut().set_frame(f_acccidental);
        self.fretboard.borrow_mut().set_frame(f_fretboard);
        self.keyboard.borrow_mut().set_frame(f_keyboard);

        Ok(())
    }

    fn draw(&mut self, ctx: &CanvasRenderingContext2d, _: f64, _: &mut bool) -> Result<(), JsValue> {
        ctx.clear_rect(0.0, 0.0, self.frame.width, self.frame.height);
        Ok(())
    }

    fn children(&self) -> &[Rc<RefCell<dyn View>>] {
        self.children.as_slice()
    }
}

impl RootView {
    pub fn new(asset: Rc<Asset>) -> Self {
        let mut children: Vec<Rc<RefCell<dyn View>>> = Vec::new();

        let accidental = Rc::new(RefCell::new(Accidental::new()));
        children.push(accidental.clone());

        let fretboard = Rc::new(RefCell::new(Fretboard::new(asset)));
        children.push(fretboard.clone());

        let keyboard = Rc::new(RefCell::new(Keyboard::new()));
        children.push(keyboard.clone());

        Self {
            frame: Rect::default(),
            children,
            accidental,
            fretboard,
            keyboard,
        }
    }
}
