use bot_lang::gen_ast;
use wasm_bindgen::prelude::*;

mod bot_lang;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello {name}")
}

#[wasm_bindgen]
pub fn ast(src: &str) -> String {
    let (_tree, err) = gen_ast(src);
    err
}
