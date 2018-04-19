#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(msg: &str);
}

#[wasm_bindgen]
pub struct Person {
    pub name: String,
    pub age: f64
}

#[wasm_bindgen]
impl Person {
    pub fn new(name: &str, age: f64) -> Person {
        Person {
            name: name.to_string(),
            age: age,
        }
    }
    pub fn say_hi() {
        log("hi");
    }
}

#[wasm_bindgen]
pub enum State {
    Open,
    Closed,
}

#[wasm_bindgen]
pub fn say_hi(person: Person) {
    log(&format!("hi {}, who is {} years old!", person.name, person.age));
}

#[wasm_bindgen]
pub fn multiply(lhs: f64, rhs: f64) -> f64 {
    lhs * rhs
}

#[wasm_bindgen]
pub fn hello_world() {
    log("Hello world");
}