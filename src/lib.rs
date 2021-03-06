mod time;
mod gib;
mod go;
mod sgf;
mod gib2sgf;
mod lexer;
use wasm_bindgen::prelude::*;

pub use crate::gib2sgf::gib_to_sgf;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn convert_gib_to_sgf(str: &str) -> Option<String> {
    gib_to_sgf(str).ok()
}
