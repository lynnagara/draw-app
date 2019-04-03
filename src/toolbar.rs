extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use web_sys::{window, Element};
use std::cell::RefCell;
use std::rc::Rc;

use crate::state::{COLORS, State};

pub fn init(toolbar: Element, state: &Rc<RefCell<State>>) -> Result<(), JsValue> {
    for (_name, hex) in COLORS.iter() {

        let document = window().unwrap().document().unwrap();

        let el = document.create_element("div")?;

        el.set_attribute(
            "style",
            &format!("height: 50px; background-color: {};", hex),
        );

        toolbar.append_child(&el)?;
    }
    Ok(())
}
