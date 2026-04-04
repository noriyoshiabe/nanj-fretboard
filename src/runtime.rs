use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlElement, PointerEvent};

pub trait AppDelegate {
    fn start(&mut self);
    fn layout(&self);
    fn render(&self, ctx: &CanvasRenderingContext2d, next: &mut bool) -> Result<(), JsValue>;
    fn pointer_down(&mut self, x: f64, y: f64);
    fn pointer_up(&mut self, x: f64, y: f64);
}

pub struct Runtime {}

struct RuntimeContext<T: AppDelegate> {
    app: T,
    canvas: HtmlCanvasElement,
    is_running: bool,
    _listeners: Vec<Closure<dyn FnMut(JsValue)>>,
    animation_frame_callback: Closure<dyn FnMut()>,
}

impl Runtime {
    pub fn run(canvas: HtmlCanvasElement, app: impl AppDelegate + 'static) -> Result<(), JsValue> {
        let ctx = canvas.get_context("2d")?.ok_or("could not get 2d context.")?.
            dyn_into::<web_sys::CanvasRenderingContext2d>()?;

        let rt_ctx = Rc::new(RefCell::new(RuntimeContext {
            app,
            canvas: canvas.clone(),
            is_running: false,
            _listeners: Vec::new(),
            animation_frame_callback: Closure::wrap(Box::new(|| {})),
        }));

        let weak_rc = Rc::downgrade(&rt_ctx);
        rt_ctx.borrow_mut().animation_frame_callback = Closure::wrap(Box::new(move || {
            if let Some(rc) = weak_rc.upgrade() {
                let mut next = false;

                rc.borrow_mut().app.render(&ctx, &mut next).unwrap_or_else(|e| {
                    web_sys::console::error_1(&e);
                });

                if next {
                    Self::request_animation_frame(&rc.borrow().animation_frame_callback).unwrap_or_else(|e| {
                        web_sys::console::error_1(&e);
                    });
                } else {
                    rc.borrow_mut().is_running = false;
                }
            }
        }) as Box<dyn FnMut()>);

        let window = web_sys::window().ok_or("window not exists.")?;
        let dpr = window.device_pixel_ratio();

        let weak_rc = Rc::downgrade(&rt_ctx);
        Self::add_event_listener(Rc::clone(&rt_ctx), canvas.clone(), "pointerdown", move |e: JsValue| -> Result<(), JsValue> {
            if let Some(rc) = weak_rc.upgrade() {
                let e = e.dyn_into::<PointerEvent>()?;
                rc.borrow_mut().app.pointer_down(e.offset_x() as f64 * dpr, e.offset_y() as f64 * dpr);
            }
            Ok(())
        })?;

        let weak_rc = Rc::downgrade(&rt_ctx);
        Self::add_event_listener(Rc::clone(&rt_ctx), canvas.clone(), "pointerup", move |e: JsValue| -> Result<(), JsValue> {
            if let Some(rc) = weak_rc.upgrade() {
                let e = e.dyn_into::<PointerEvent>()?;
                rc.borrow_mut().app.pointer_up(e.offset_x() as f64 * dpr, e.offset_y() as f64 * dpr);
            }
            Ok(())
        })?;

        let weak_rc = Rc::downgrade(&rt_ctx);
        Self::add_event_listener(Rc::clone(&rt_ctx), window.clone(), "resize", move |_: JsValue| -> Result<(), JsValue> {
            if let Some(rc) = weak_rc.upgrade() {
                Self::resize_canvas(&rc.borrow().canvas)?;
                rc.borrow_mut().app.layout();
            }
            Ok(())
        })?;

        rt_ctx.borrow_mut().app.start();
        rt_ctx.borrow_mut().is_running = true;

        Self::request_animation_frame(&rt_ctx.borrow().animation_frame_callback)?;

        std::mem::forget(rt_ctx);

        Ok(())
    }

    pub fn init_canvas(canvas_id: &str) -> Result<HtmlCanvasElement, JsValue> {
        let canvas = web_sys::window().ok_or("window not exists.")?.
            document().ok_or("document not exists.")?.
            get_element_by_id(canvas_id).ok_or("canvas not exists.")?.
            dyn_into::<HtmlCanvasElement>()?;

        Self::resize_canvas(&canvas)?;

        Ok(canvas)
    }

    fn request_animation_frame(f: &Closure<dyn FnMut()>) -> Result<(), JsValue> {
        web_sys::window().ok_or("window not exists.")?.request_animation_frame(f.as_ref().unchecked_ref())?;
        Ok(())
    }

    fn add_event_listener<T, F>(rt_ctx: Rc<RefCell<RuntimeContext<impl AppDelegate + 'static>>>, target: T, event: &str, listener: F) -> Result<(), JsValue>
    where
        T: JsCast + AsRef<web_sys::EventTarget>,
        F: Fn(JsValue) -> Result<(), JsValue> + 'static,
    {
        let weak_rc = Rc::downgrade(&rt_ctx);
        let wrapped_listener = Closure::wrap(Box::new(move |e: JsValue| {
            if let Some(rc) = weak_rc.upgrade() {
                listener(e).unwrap_or_else(|e| {
                    web_sys::console::error_1(&e);
                });

                if !rc.borrow().is_running {
                    rc.borrow_mut().is_running = true;

                    Self::request_animation_frame(&rc.borrow().animation_frame_callback).unwrap_or_else(|e| {
                        web_sys::console::error_1(&e);
                    });
                }
            }
        }) as Box<dyn FnMut(JsValue)>);

        target.dyn_ref::<web_sys::EventTarget>().ok_or("cloud not cat target element to event target.")?.
            add_event_listener_with_callback(event, wrapped_listener.as_ref().unchecked_ref())?;
        rt_ctx.borrow_mut()._listeners.push(wrapped_listener);

        Ok(())
    }

    pub fn resize_canvas(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
        let window = web_sys::window().ok_or("window not exists.")?;
        let dpr = window.device_pixel_ratio();

        let width = window.inner_width()?.as_f64().unwrap_or(0.);
        let height = window.inner_height()?.as_f64().unwrap_or(0.);

        canvas.set_width((width * dpr) as u32);
        canvas.set_height((height * dpr) as u32);

        let html_el = canvas.dyn_ref::<HtmlElement>().ok_or("could not cast canvas to html element.")?;
        let style = html_el.style();
        style.set_property("width", &format!("{}px", width))?;
        style.set_property("height", &format!("{}px", height))?;

        Ok(())
    }
}
