extern crate js_sys;
extern crate fixedbitset;

use wasm_bindgen::prelude::*;
use fixedbitset::FixedBitSet;


#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbour_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;

        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_column in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_column == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_column = (column + delta_column) % self.width;
                let index = self.get_index(neighbor_row, neighbor_column);
                count += self.cells[index] as u8
            }
        }

        count
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for column in 0..self.width {
                let index = self.get_index(row, column);
                let cell = self.cells[index];
                let live_neighbours = self.live_neighbour_count(row, column);

                next.set(index, match (cell, live_neighbours) {
                    // 1: Any live cell with fewer than two live neighbours dies.
                    (true, x) if x < 2 => false,

                    // 2: Any live cell with two or three live neighbours lives on to the next generation.
                    (true, 2) | (true, 3) => true,

                    // 3: Any live cell with more than three live neighbours dies.
                    (true, x) if x > 3 => false,

                    // 4: Any dead cell with exactly three live neighbours becomes a live cell.
                    (false, 3) => true,
                    
                    (otherwise, _) => otherwise
                });
            }
        }

        self.cells = next;
    }

    pub fn new() -> Universe {
        let width = 64;
        let height = 64;

        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);
    
        for i in 0..size {
            cells.set(i, js_sys::Math::random() < 0.5);
        }

        Universe {
            width,
            height,
            cells,
        }
    }
}
