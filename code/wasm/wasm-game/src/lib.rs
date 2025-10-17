use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
}

struct SnackCell (u32);

struct Snake {
    body: Vec<SnackCell>,
}

impl Snake {
    fn new(spawn_index: u32) -> Self {
        Self {
            body: vec![SnackCell(spawn_index)],
        }
    }
}

#[wasm_bindgen]
pub struct World {
    width: u32,
    size: u32,
    snake: Snake,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: u32, snake_head_index: u32) -> Self {
        World { 
            width, 
            size: width * width,
            snake: Snake::new(snake_head_index),
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn snake_head_index(&self) -> u32 {
        self.snake.body[0].0
    }

    pub fn update(&mut self) {
        let head_index = self.snake_head_index();
        self.snake.body[0].0 = (head_index + 1) % self.size;
    }
}