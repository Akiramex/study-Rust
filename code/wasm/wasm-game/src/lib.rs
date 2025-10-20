use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
}

#[wasm_bindgen]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct SnackCell (u32);

struct Snake {
    body: Vec<SnackCell>,
    direction: Direction,
}

impl Snake {
    fn new(spawn_index: u32) -> Self {
        Self {
            body: vec![SnackCell(spawn_index)],
            direction: Direction::Right,
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
        let (head_row, head_col) = self.index_to_row_col(head_index);
        let (row, col) = match self.snake.direction {
            Direction::Up => {
                ((head_row - 1) % self.width, head_col)
            }
            Direction::Down => {
                ((head_row + 1) % self.width, head_col)
            }
            Direction::Left => {
                (head_row, (head_col - 1) % self.width)
            }
            Direction::Right => {
                (head_row, (head_col + 1) % self.width)
            }
        };
        let new_head_index = self.row_col_to_index(row, col);
        self.set_snake_head_index(new_head_index);
    }

    pub fn change_snake_direction(&mut self, direction: Direction) {
        self.snake.direction = direction;
    }
    
    fn set_snake_head_index(&mut self, index: u32) {
        self.snake.body[0].0 = index;
    }

    fn index_to_row_col(&self, index: u32) -> (u32, u32) {
        (index / self.width, index % self.width)
    }

    fn row_col_to_index(&self, row: u32, col: u32) -> u32 {
        row * self.width + col
    }
}