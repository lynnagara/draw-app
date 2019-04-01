extern crate wasm_bindgen;

#[macro_use]
extern crate lazy_static;

use wasm_bindgen::prelude::*;
use std::sync::Mutex;

lazy_static! {
    // Image is 100 x 100
    static ref ARRAY: Mutex<Vec<Vec<u8>>> = Mutex::new(vec![vec![0; 100]; 100]);
}

// #[wasm_bindgen]
// pub fn adder() {
//     ARRAY.lock().unwrap().push(1);
// }


#[wasm_bindgen]
pub fn draw(x: usize, y: usize) -> String {
    ARRAY.lock().unwrap()[x][y] = 1;
    let l = ARRAY.lock().unwrap()[x][y];
    String::from(format!("{}{}{:?}", x, y, l))
}

#[wasm_bindgen]
pub fn render() -> String {
    let elements = ARRAY.lock().unwrap().to_owned();

    serde_json::to_string(&elements).unwrap()
}