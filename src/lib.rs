extern crate wasm_bindgen;

use std::cell::RefCell;
use std::cmp::{max, min};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, Element, HtmlCanvasElement, HtmlElement};

mod canvas;
mod state;
mod toolbar;

static TOOLBAR_WIDTH: u32 = 50;

#[wasm_bindgen]
pub fn init_app() -> Result<(), JsValue> {
    let window = window().expect("Could not find `window`");
    let document = window.document().expect("Could not find `document`");
    let body = document.body().expect("Could not find `body` element");

    let (w, h) = get_dimensions(&body);

    let state: Rc<RefCell<state::State>> = Rc::new(RefCell::new(state::State::new(w, h)));

    let root = document.create_element("div")?;
    root.set_attribute("style", "min-height: 100%;");

    body.append_child(&root);

    let canvas_el = document
        .create_element("canvas")?
        .dyn_into::<HtmlCanvasElement>()?;
    canvas_el.set_width(w - TOOLBAR_WIDTH);
    canvas_el.set_height(h);

    root.append_child(&canvas_el)?;
    canvas::init_canvas(&canvas_el, &state);

    let toolbar_el = document.create_element("div")?.dyn_into::<Element>()?;
    toolbar_el.set_attribute("style", "width:100%; border-left: 1px solid #efefef;");
    body.append_child(&toolbar_el)?;
    toolbar::init_toolbar(&toolbar_el, &canvas_el, &state);

    Ok(())
}

fn get_dimensions(body: &HtmlElement) -> (u32, u32) {
    let client_width = body.client_width() as u32;
    let client_height = body.client_height() as u32;

    let width = min(max(client_width, 600), 3000);
    let height = min(max(client_height, 400), 2000);

    (width, height)
}
