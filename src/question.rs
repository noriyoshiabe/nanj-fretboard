use std::cell::RefCell;
use std::rc::{Rc, Weak};
use rand::seq::SliceRandom;
use rand::rng;
use wasm_bindgen::prelude::*;
use crate::task_queue::TaskQueue;

#[derive(Clone, Copy, PartialEq)]
pub struct QuestionItem {
    pub string: u8,
    pub fret: u8,
    note: &'static str,
}

#[derive(Clone, Copy)]
pub enum QuestionEvent {
    New(QuestionItem),
    Answer { correct: bool },
    AccidentalChange { accidental: bool },
}

pub trait QuestionObserver {
    fn on_notify_event(&mut self, event: QuestionEvent) -> Result<(), JsValue>;
}

pub struct Question {
    observers: Vec<Weak<RefCell<dyn QuestionObserver>>>,
    task_queue: Rc<RefCell<TaskQueue>>,
    current_item: QuestionItem,
    items: Vec<QuestionItem>,
    lot_items: Vec<QuestionItem>,
    accidental: bool,
}

impl Question {
    pub fn new(task_queue: Rc<RefCell<TaskQueue>>) -> Rc<RefCell<Self>> {
        let observers: Vec<Weak<RefCell<dyn QuestionObserver>>> = Vec::new();
        let items = build_question_items();
        let mut lot_items: Vec<QuestionItem> = items.iter().filter(|&i| !i.note.contains("#")).cloned().collect();

        lot_items.shuffle(&mut rng());
        let current_item = lot_items.pop().expect("definitely exists");

        Rc::new(RefCell::new(Self {
            observers,
            task_queue,
            items,
            lot_items,
            current_item,
            accidental: false,
        }))
    }

    pub fn add_observer(&mut self, observer: Rc<RefCell<dyn QuestionObserver>>) {
        self.observers.push(Rc::downgrade(&observer));
    }

    fn notify_event(&mut self, event: QuestionEvent) {
        self.observers.retain(|weak| weak.strong_count() > 0);

        for weak in self.observers.iter() {
            let weac_c = Weak::clone(weak);

            self.task_queue.borrow_mut().enqueue(move || -> Result<(), JsValue> {
                if let Some(rc) = weac_c.upgrade() {
                    rc.borrow_mut().on_notify_event(event)?;
                }
                Ok(())
            });
        }
    }

    pub fn start(&mut self) {
        self.notify_event(QuestionEvent::New(self.current_item));
    }

    pub fn answer(&mut self, note: &str) {
        let correct = self.current_item.note == note;

        self.notify_event(QuestionEvent::Answer{ correct });

        if correct {
            if self.lot_items.is_empty() {
                self.reset_lot_items();
            }

            self.current_item = self.lot_items.pop().expect("definitely exists");
            self.notify_event(QuestionEvent::New(self.current_item));
        }
    }

    pub fn toggle_accidental(&mut self) {
        self.accidental = !self.accidental;
        self.reset_lot_items();
        self.notify_event(QuestionEvent::AccidentalChange{ accidental: self.accidental });
    }

    fn reset_lot_items(&mut self) {
        self.lot_items = self.items.iter().filter(|i| {
            if self.accidental {
                true
            } else {
                !i.note.contains("#")
            }
        }).cloned().collect();
        self.lot_items = self.lot_items.iter().filter(|&i| *i != self.current_item ).cloned().collect();
        self.lot_items.shuffle(&mut rng());
    }
}

const NOTE_LIST: [&str; 12] = [
  "C",
  "C#",
  "D",
  "D#",
  "E",
  "F",
  "F#",
  "G",
  "G#",
  "A",
  "A#",
  "B",
];

fn build_question_items() -> Vec<QuestionItem> {
    let mut items: Vec<QuestionItem> = Vec::new();

    for i in 1..=6 {
        let mut note_index: usize = match i {
            1 => NOTE_LIST.iter().position(|&p| p == "E" ).expect("definitely exists"),
            2 => NOTE_LIST.iter().position(|&p| p == "B" ).expect("definitely exists"),
            3 => NOTE_LIST.iter().position(|&p| p == "G" ).expect("definitely exists"),
            4 => NOTE_LIST.iter().position(|&p| p == "D" ).expect("definitely exists"),
            5 => NOTE_LIST.iter().position(|&p| p == "A" ).expect("definitely exists"),
            6 => NOTE_LIST.iter().position(|&p| p == "E" ).expect("definitely exists"),
            _ => 0,
        };

        for j in 0..=12 {
            items.push(QuestionItem{
                string: i,
                fret: j,
                note: NOTE_LIST[note_index],
            });

            note_index += 1;
            if NOTE_LIST.len() <= note_index {
                note_index = 0;
            }
        }
    }

    items
}
