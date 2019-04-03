extern crate wasm_bindgen;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, CanvasRenderingContext2d, Element, HtmlCanvasElement};

use crate::state::{State, COLORS, PEN_SIZES};

pub fn init(
    toolbar: &Element,
    canvas: &HtmlCanvasElement,
    state: &Rc<RefCell<State>>,
) -> Result<(), JsValue> {
    let document = window().unwrap().document().unwrap();

    let generic_box_styles = "height: 50px; border-bottom: 1px solid #efefef; display: flex; align-items: center; justify-content: center;";

    for (_name, hex) in COLORS.iter() {
        let el = document.create_element("div")?;

        el.set_attribute(
            "style",
            &format!("{} background-color: {};", generic_box_styles, hex),
        );

        let state_copy = state.clone();

        let handle_click = Closure::wrap(Box::new(move || {
            state_copy.borrow_mut().update_color(hex.to_string());
        }) as Box<dyn FnMut()>);

        el.add_event_listener_with_callback("click", handle_click.as_ref().unchecked_ref())?;

        handle_click.forget();

        toolbar.append_child(&el)?;
    }

    for size in PEN_SIZES.iter() {
        let el = document.create_element("div")?;

        el.set_attribute("style", generic_box_styles);
        toolbar.append_child(&el)?;

        let inner_el = document.create_element("div")?;

        let style = format!(
            "border-radius: 50%; background-color: black; width: {}px; height: {}px;",
            size + 2.0,
            size + 2.0
        );
        inner_el.set_attribute("style", &style);
        el.append_child(&inner_el);

        let state_copy = state.clone();

        let handle_click = Closure::wrap(Box::new(move || {
            state_copy.borrow_mut().update_pen_size(*size);
        }) as Box<dyn FnMut()>);

        el.add_event_listener_with_callback("click", handle_click.as_ref().unchecked_ref())?;

        handle_click.forget();
    }

    {
        let el = document.create_element("div")?;

        el.set_attribute(
            "style",
            &format!("{} font-size: 11px; cursor: default;", generic_box_styles),
        );
        el.set_inner_html("clear");
        toolbar.append_child(&el)?;

        let state_copy = state.clone();

        let context = canvas
            .get_context("2d")
            .expect("Could not get context")
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        let handle_click = Closure::wrap(Box::new(move || {
            context.clear_rect(
                0.0,
                0.0,
                state_copy.borrow().get_width() as f64,
                state_copy.borrow().get_height() as f64,
            );
        }) as Box<dyn FnMut()>);

        el.add_event_listener_with_callback("click", handle_click.as_ref().unchecked_ref())?;

        handle_click.forget();
    }

    Ok(())
}
