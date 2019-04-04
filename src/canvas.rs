extern crate wasm_bindgen;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent};

use crate::state::State;

pub fn init(canvas: &HtmlCanvasElement, state: &Rc<RefCell<State>>) -> Result<(), JsValue> {
    let context = canvas
        .get_context("2d")
        .expect("Could not get context")
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    {
        let context_copy = context.clone();
        let state_copy = state.clone();
        let canvas_copy = canvas.clone();
        let handle_mouse_down = Closure::wrap(Box::new(move |event: MouseEvent| {
            state_copy.borrow_mut().start_drawing();
            state_copy
                .borrow_mut()
                .add_undo_state(canvas_copy.to_data_url().unwrap());
            let new_x = event.offset_x() as f64;
            let new_y = event.offset_y() as f64;
            context_copy.begin_path();
            context_copy.set_stroke_style(&JsValue::from(state_copy.borrow().get_color()));
            context_copy.set_line_width(state_copy.borrow().get_pen_size());
            context_copy.move_to(new_x, new_y);
        }) as Box<dyn FnMut(_)>);

        canvas.add_event_listener_with_callback(
            "mousedown",
            handle_mouse_down.as_ref().unchecked_ref(),
        )?;

        handle_mouse_down.forget();
    }

    {
        let context_copy = context.clone();
        let state_copy = state.clone();

        let handle_mouse_up = Closure::wrap(Box::new(move |event: MouseEvent| {
            state_copy.borrow_mut().stop_drawing();
            let new_x = event.offset_x() as f64;
            let new_y = event.offset_y() as f64;
            context_copy.fill_rect(new_x, new_y, 1.0, 1.0);
            context_copy.line_to(new_x, new_y);
            context_copy.stroke();
        }) as Box<dyn FnMut(_)>);

        canvas.add_event_listener_with_callback(
            "mouseup",
            handle_mouse_up.as_ref().unchecked_ref(),
        )?;

        handle_mouse_up.forget();
    }

    {
        let context_copy = context.clone();
        let state_copy = state.clone();
        let handle_mouse_move = Closure::wrap(Box::new(move |event: MouseEvent| {
            if state_copy.borrow().is_drawing() {
                let new_x = event.offset_x() as f64;
                let new_y = event.offset_y() as f64;
                context_copy.line_to(new_x, new_y);
                context_copy.stroke();
            }
        }) as Box<dyn FnMut(_)>);

        canvas.add_event_listener_with_callback(
            "mousemove",
            handle_mouse_move.as_ref().unchecked_ref(),
        )?;

        handle_mouse_move.forget();
    }

    Ok(())
}
