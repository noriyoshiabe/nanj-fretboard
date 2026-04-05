use std::cell::RefCell;
use std::rc::{Rc, Weak};
use wasm_bindgen::prelude::*;
use crate::task_queue::TaskQueue;

#[derive(Clone, Copy)]
pub struct QuestionItem {
    pub string: u8,
    pub fret: u8,
    note: &'static str,
}

#[derive(Clone, Copy)]
pub enum QuestionEvent {
    New(QuestionItem),
    Answer { correct: bool },
}

pub trait QuestionObserver {
    fn on_notify_event(&mut self, event: QuestionEvent) -> Result<(), JsValue>;
}

pub struct Question {
    observers: Vec<Weak<RefCell<dyn QuestionObserver>>>,
    task_queue: Rc<RefCell<TaskQueue>>,
}

impl Question {
    pub fn new(task_queue: Rc<RefCell<TaskQueue>>) -> Self {
        let observers: Vec<Weak<RefCell<dyn QuestionObserver>>> = Vec::new();

        Self {
            observers,
            task_queue,
        }
    }

    pub fn add_observer(&mut self, observer: Rc<RefCell<dyn QuestionObserver>>) {
        self.observers.push(Rc::downgrade(&observer));
    }

    fn notify_event(&mut self, event: QuestionEvent) {
        self.observers.retain(|weak| weak.strong_count() > 0);

        for weak in self.observers.iter() {
            let weac_c = weak.clone();

            self.task_queue.borrow_mut().enqueue(move || -> Result<(), JsValue> {
                if let Some(rc) = weac_c.upgrade() {
                    rc.borrow_mut().on_notify_event(event)?;
                }
                Ok(())
            });
        }
    }

    pub fn start(&mut self) {
        self.notify_event(QuestionEvent::New(QuestionItem{
            string: 6,
            fret: 12,
            note: "C",
        }))
    }

    pub fn answer(&mut self, note: &str) {
        self.notify_event(QuestionEvent::Answer{ correct: true })
    }
}
