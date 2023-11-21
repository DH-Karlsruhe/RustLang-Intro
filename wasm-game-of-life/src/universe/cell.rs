use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)] // representation of Cell as single byte in linear wasm mem.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Cell {
    pub fn toggle(&mut self) {
        *self = match *self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        };
    }
}

pub fn get_glider_bitmap() -> [(u8, u8, u8); 3] {
    return [
        (0, 0, 8),
        (8, 0, 8),
        (0, 8, 8),
    ]
}