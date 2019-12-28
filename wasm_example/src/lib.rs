#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    type HTMLDocument;
    type Element;

    static document: HTMLDocument;

    #[wasm_bindgen(method)]
    fn createElement(this: &HTMLDocument, tagName: &str) -> Element;

}


#[wasm_bindgen]
pub fn run_alert(item: &str) {
    alert(&format!("This is WASM and {}"), item);
}


pub fn create_stuff() {
    let div = document.createElement("div")
    
}