mod time;
mod gib;
mod go;
mod sgf;
mod gib2sgf;
use wasm_bindgen::prelude::*;

pub use crate::gib2sgf::gib_to_sgf;

#[wasm_bindgen]
pub fn convert_gib_to_sgf(str: &str) -> Option<String> {
    gib_to_sgf(str).ok()
}
