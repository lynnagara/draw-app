extern crate wasm_bindgen;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, Element};

use crate::state::{State, COLORS, PEN_SIZES};

pub fn init(toolbar: Element, state: &Rc<RefCell<State>>) -> Result<(), JsValue> {
    let document = window().unwrap().document().unwrap();

    for (_name, hex) in COLORS.iter() {
        let el = document.create_element("div")?;

        el.set_attribute(
            "style",
            &format!("height: 50px; background-color: {};", hex),
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

        el.set_attribute("style", "height: 50px;");
        toolbar.append_child(&el)?;

        let inner_el = document.create_element("div")?;
        inner_el.set_attribute("style", "border-radius: 50%;");
        inner_el.set_inner_html(&format!("{:?}", size));
        el.append_child(&inner_el);

        let state_copy = state.clone();

        let handle_click = Closure::wrap(Box::new(move || {
            state_copy.borrow_mut().update_pen_size(*size);
        }) as Box<dyn FnMut()>);

        el.add_event_listener_with_callback("click", handle_click.as_ref().unchecked_ref())?;

        handle_click.forget();
    }

    {
        
    }

    Ok(())
}
