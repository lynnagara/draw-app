extern crate wasm_bindgen;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    window, CanvasRenderingContext2d, Document, Element, HtmlCanvasElement, HtmlElement,
    HtmlImageElement,
};

use crate::state::{State, COLORS, PEN_SIZES};

const generic_box_styles: &str = "height: 50px; border-bottom: 1px solid #efefef; display: flex; align-items: center; justify-content: center;";

enum UndoRedo {
    Undo,
    Redo,
}

pub fn init(
    toolbar: &Element,
    canvas: &HtmlCanvasElement,
    state: &Rc<RefCell<State>>,
) -> Result<(), JsValue> {
    let document = window().unwrap().document().unwrap();

    for (_name, hex) in COLORS.iter() {
        let el = get_color_block_element(hex.to_string(), &document, state)?;
        toolbar.append_child(&el)?;
    }

    for size in PEN_SIZES.iter() {
        let el = get_pen_size_element(*size, &document, state)?;
        toolbar.append_child(&el)?;
    }

    let clear_el = get_clear_element(&document, state, canvas)?;
    toolbar.append_child(&clear_el);

    let undo_el = get_undo_redo_element(UndoRedo::Undo, &document, state, canvas)?;
    toolbar.append_child(&undo_el);
    let redo_el = get_undo_redo_element(UndoRedo::Redo, &document, state, canvas)?;
    toolbar.append_child(&redo_el);

    Ok(())
}

fn get_color_block_element(
    hex: String,
    document: &Document,
    state: &Rc<RefCell<State>>,
) -> Result<Element, JsValue> {
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

    Ok(el)
}

fn get_pen_size_element(
    size: f64,
    document: &Document,
    state: &Rc<RefCell<State>>,
) -> Result<Element, JsValue> {
    let el = document.create_element("div")?;

    el.set_attribute("style", generic_box_styles);

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
        state_copy.borrow_mut().update_pen_size(size as f64);
    }) as Box<dyn FnMut()>);

    el.add_event_listener_with_callback("click", handle_click.as_ref().unchecked_ref())?;

    handle_click.forget();

    Ok(el)
}

fn get_clear_element(
    document: &Document,
    state: &Rc<RefCell<State>>,
    canvas: &HtmlCanvasElement,
) -> Result<Element, JsValue> {
    let el = document.create_element("div")?;

    el.set_attribute(
        "style",
        &format!("{} font-size: 11px; cursor: default;", generic_box_styles),
    );
    el.set_inner_html("clear");

    let state_copy = state.clone();

    let canvas_copy = canvas.clone();

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

        state_copy
            .borrow_mut()
            .add_undo_state(canvas_copy.to_data_url().unwrap());
    }) as Box<dyn FnMut()>);

    el.add_event_listener_with_callback("click", handle_click.as_ref().unchecked_ref())?;

    handle_click.forget();

    Ok(el)
}

fn get_undo_redo_element(
    undo_or_redo: UndoRedo,
    document: &Document,
    state: &Rc<RefCell<State>>,
    canvas: &HtmlCanvasElement,
) -> Result<Element, JsValue> {
    let el = document.create_element("div")?;

    el.set_attribute(
        "style",
        &format!("{} font-size: 11px; cursor: default;", generic_box_styles),
    );
    let text = match undo_or_redo {
        UndoRedo::Undo => "undo",
        UndoRedo::Redo => "redo",
    };

    el.set_inner_html(text);

    let state_copy = state.clone();

    let canvas_copy = canvas.clone();

    let context = canvas
        .get_context("2d")
        .expect("Could not get context")
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    let handle_click = Closure::wrap(Box::new(move || {
        let target = match undo_or_redo {
            UndoRedo::Undo => {
                state_copy
                    .borrow_mut()
                    .add_redo_state(canvas_copy.to_data_url().unwrap());
                state_copy.borrow_mut().undo()
            }
            UndoRedo::Redo => {
                state_copy
                    .borrow_mut()
                    .add_undo_state(canvas_copy.to_data_url().unwrap());
                state_copy.borrow_mut().redo()
            },
        };

        let state_copy_2 = state_copy.clone();
        let context_copy = context.clone();

        match target {
            Some(p) => {
                let image_el = HtmlImageElement::new().unwrap();
                image_el.set_src(&p);

                let html_image_el = image_el.clone().dyn_into::<HtmlElement>().unwrap();

                let handle_onload = Closure::wrap(Box::new(move || {
                    context_copy.clear_rect(
                        0.0,
                        0.0,
                        state_copy_2.borrow().get_width() as f64,
                        state_copy_2.borrow().get_height() as f64,
                    );
                    context_copy.draw_image_with_html_image_element(&image_el, 0.0, 0.0);
                }) as Box<dyn FnMut()>);

                html_image_el.set_onload(Some(handle_onload.as_ref().unchecked_ref()));

                handle_onload.forget();
            }
            None => {}
        }
    }) as Box<dyn FnMut()>);

    el.add_event_listener_with_callback("click", handle_click.as_ref().unchecked_ref())?;

    handle_click.forget();

    Ok(el)
}
