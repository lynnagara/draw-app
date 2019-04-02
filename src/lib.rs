extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;


#[wasm_bindgen]
pub fn init(w: u32, h: u32) -> Result<(), JsValue> {
    let window = web_sys::window().expect("Could not find `window`");
    let document = window.document().expect("Could not find `document`");
    let body = document.body().expect("Could not find `body` element");
    let canvas = document.create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    let context = canvas
        .get_context("2d")
        .expect("Could not get context")
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    canvas.set_width(w);
    canvas.set_height(h);
    body.append_child(&canvas)?;


    let handle_mouse_down = Closure::wrap(Box::new(move || {
        console::log_1(&"called mousedown".into());
    }) as Box<dyn FnMut()>);

    let handle_mouse_up = Closure::wrap(Box::new(move || {
        console::log_1(&"called mouseup".into());
    }) as Box<dyn FnMut()>);

    let handle_mouse_move = Closure::wrap(Box::new(move || {
        console::log_1(&"called move".into());
    }) as Box<dyn FnMut()>);

    canvas.add_event_listener_with_callback(
        "mousedown",
        handle_mouse_down.as_ref().unchecked_ref()
    )?;

    canvas.add_event_listener_with_callback(
        "mouseup",
        handle_mouse_up.as_ref().unchecked_ref()
    )?;

    canvas.add_event_listener_with_callback(
        "mousemove",
        handle_mouse_move.as_ref().unchecked_ref()
    )?;

    // Leaking memory :)
    handle_mouse_down.forget();
    handle_mouse_up.forget();
    handle_mouse_move.forget();

    Ok(())
}
