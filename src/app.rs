use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::asset::Asset;
use crate::dispatcher::Dispatcher;
use crate::question::Question;
use crate::root_view::RootView;
use crate::runtime::AppDelegate;
use crate::view::{View, Rect};
use crate::task_queue::TaskQueue;

pub struct App {
    canvas: HtmlCanvasElement,
    root_view: Rc<RefCell<RootView>>,
    dispathcer: Dispatcher,
    question: Rc<RefCell<Question>>,
    task_queue: Rc<RefCell<TaskQueue>>,
}

impl AppDelegate for App {
    fn start(&mut self) -> Result<(), JsValue> {
        self.layout()?;
        self.question.borrow_mut().start();
        Ok(())
    }

    fn layout(&self) -> Result<(), JsValue> {
        let width = self.canvas.width() as f64;
        let height = self.canvas.height() as f64;
        self.root_view.borrow_mut().set_frame(Rect { x: 0., y: 0., width, height });
        self.dispathcer.dispatch_layout()
    }

    fn render(&self, ctx: &CanvasRenderingContext2d, dpr: f64, next: &mut bool) -> Result<(), JsValue> {
        self.task_queue.borrow_mut().dispatch_task(next)?;
        self.dispathcer.dispatch_render(ctx, dpr, next)
    }

    fn pointer_down(&mut self, id: i32, x: f64, y: f64) -> Result<(), JsValue> {
        self.dispathcer.dispatch_pointer_down(id, x, y)
    }

    fn pointer_up(&mut self, id: i32, x: f64, y: f64) -> Result<(), JsValue> {
        self.dispathcer.dispatch_pointer_up(id, x, y)
    }
}

impl App {
    pub fn try_new(canvas: HtmlCanvasElement) -> Result<Self, JsValue> {
        let asset = Rc::new(Asset::try_new()?);
        let task_queue = Rc::new(RefCell::new(TaskQueue::new()));
        let question = Rc::new(RefCell::new(Question::new(task_queue.clone())));
        let root_view = Rc::new(RefCell::new(RootView::new(asset, question.clone())));
        let dispathcer = Dispatcher::new(root_view.clone());

        Ok(Self {
            canvas,
            root_view,
            dispathcer,
            question,
            task_queue,
        })
    }
}
