extern crate wasm_bindgen;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, Element, HtmlCanvasElement};

mod canvas;
mod state;
mod toolbar;

static TOOLBAR_WIDTH: u32 = 50;

#[wasm_bindgen]
pub fn init(w: u32, h: u32) -> Result<(), JsValue> {
    let state: Rc<RefCell<state::State>> = Rc::new(RefCell::new(state::State::new()));

    let window = window().expect("Could not find `window`");
    let document = window.document().expect("Could not find `document`");
    let body = document.body().expect("Could not find `body` element");

    let canvas_el = document
        .create_element("canvas")?
        .dyn_into::<HtmlCanvasElement>()?;
    canvas_el.set_width(w - TOOLBAR_WIDTH);
    canvas_el.set_height(h);
    body.append_child(&canvas_el)?;
    canvas::init(canvas_el, &state);

    let toolbar_el = document.create_element("div")?.dyn_into::<Element>()?;
    toolbar_el.set_attribute("style", "width:100%;");
    body.append_child(&toolbar_el)?;
    toolbar::init(toolbar_el, &state);

    Ok(())
}
