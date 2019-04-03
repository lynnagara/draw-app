extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element};

static COLORS: [(&str, &str); 2] = [("Black", "#000000"), ("Green", "#3DC06C")];

pub fn init(toolbar: Element, document: Document) -> Result<(), JsValue> {
    for (_name, hex) in COLORS.iter() {
        let el = document.create_element("div")?;

        el.set_attribute(
            "style",
            &format!("height: 50px; background-color: {};", hex),
        );

        toolbar.append_child(&el)?;
    }
    Ok(())
}
