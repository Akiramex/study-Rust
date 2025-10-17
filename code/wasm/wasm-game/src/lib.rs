use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
}

#[wasm_bindgen]
struct World {
    width: u32,
    size: u32,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: u32) -> Self {
        World { width, size: width * width}
    }

    pub fn width(&self) -> u32 {
        self.width
    }
}