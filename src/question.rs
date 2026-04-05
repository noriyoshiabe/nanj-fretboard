use std::cell::RefCell;
use std::rc::{Rc, Weak};
//use wasm_bindgen::prelude::*;

#[derive(Clone, Copy)]
pub struct QuestionItem {
    string: u8,
    fret: u8,
    note: &'static str,
}

#[derive(Clone, Copy)]
pub enum QuestionEvent {
    New(QuestionItem),
    Answer { correct: bool },
}

pub trait QuestionObserver {
    fn on_notify_event(&mut self, event: QuestionEvent);
}

pub struct Question {
    observers: Vec<Weak<RefCell<dyn QuestionObserver>>>,
}

impl Question {
    pub fn new() -> Self {
        let observers: Vec<Weak<RefCell<dyn QuestionObserver>>> = Vec::new();

        Self {
            observers,
        }
    }

    pub fn add_observer(&mut self, observer: Rc<RefCell<dyn QuestionObserver>>) {
        self.observers.push(Rc::downgrade(&observer));
    }

    fn notify_event(&mut self, event: QuestionEvent) {
        self.observers.retain(|weak| {
            if let Some(rc) = weak.upgrade() {
                rc.borrow_mut().on_notify_event(event);
                true
            } else {
                false
            }
        });
    }

    pub fn start(&mut self) {
        self.notify_event(QuestionEvent::New(QuestionItem{
            string: 1,
            fret: 0,
            note: "C",
        }))
    }

    pub fn answer(&mut self) {
        self.notify_event(QuestionEvent::Answer{ correct: true });
    }
}
