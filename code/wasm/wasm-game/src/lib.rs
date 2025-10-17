use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn add(left: i32, right: i32) -> i32 {
    left + right
}

#[wasm_bindgen]
pub fn sub(left: i32, right: i32) -> i32 {
    left - right
}