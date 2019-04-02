extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, window, MouseEvent, CanvasRenderingContext2d, HtmlCanvasElement};


pub struct State {
    is_drawing: bool
}


#[wasm_bindgen]
pub fn init(w: u32, h: u32) -> Result<(), JsValue> {
    let window = window().expect("Could not find `window`");
    let document = window.document().expect("Could not find `document`");
    let body = document.body().expect("Could not find `body` element");
    let canvas = document.create_element("canvas")?
        .dyn_into::<HtmlCanvasElement>()?;
    let context = canvas
        .get_context("2d")
        .expect("Could not get context")
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    canvas.set_width(w);
    canvas.set_height(h);
    body.append_child(&canvas)?;

    context.begin_path();
    let mut is_drawing = false;

    {
        let context_copy = context.clone();
        let handle_mouse_down = Closure::wrap(Box::new(move |event: MouseEvent| {
            is_drawing = true;
            let new_x = event.x() as f64;
            let new_y = event.y() as f64;
            context_copy.move_to(new_x, new_y);
        }) as Box<dyn FnMut(_)>);

        canvas.add_event_listener_with_callback(
            "mousedown",
            handle_mouse_down.as_ref().unchecked_ref()
        )?;

        handle_mouse_down.forget();
    }

    {
        let context_copy = context.clone();

        let handle_mouse_up = Closure::wrap(Box::new(move |event: MouseEvent| {
            let new_x = event.x() as f64;
            let new_y = event.y() as f64;
            context_copy.fill_rect(new_x, new_y, 1.0, 1.0);
            context_copy.line_to(new_x, new_y);
            context_copy.stroke();

        }) as Box<dyn FnMut(_)>);

        canvas.add_event_listener_with_callback(
            "mouseup",
            handle_mouse_up.as_ref().unchecked_ref()
        )?;

        handle_mouse_up.forget();
    }

    {
        let handle_mouse_move = Closure::wrap(Box::new(move |event: MouseEvent| {
            console::log_1(&is_drawing.into());

            if is_drawing {
                console::log_1(&"called move".into());
            }
        }) as Box<dyn FnMut(_)>);

        canvas.add_event_listener_with_callback(
            "mousemove",
            handle_mouse_move.as_ref().unchecked_ref()
        )?;

        handle_mouse_move.forget();

    }

    Ok(())
}
