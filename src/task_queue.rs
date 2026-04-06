use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

type Task = Box<dyn FnOnce() -> Result<(), JsValue>>;

pub struct TaskQueue {
    queue: RefCell<Vec<Task>>,
}

impl TaskQueue {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            queue: RefCell::new(Vec::new()),
        }))
    }

    pub fn enqueue<F>(&self, task: F)
    where
        F: FnOnce() -> Result<(), JsValue> + 'static,
    {
        self.queue.borrow_mut().push(Box::new(task));
    }

    pub fn dispatch_task(&mut self, next: &mut bool) -> Result<(), JsValue> {
        let tasks = self.queue.replace(Vec::new());
        
        for task in tasks {
            task()?;
            *next = true;
        }

        Ok(())
    }
}

