mod utils;

use wasm_bindgen::prelude::*;
use fixedbitset::FixedBitSet;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Universe {
    height: u32,
    width: u32,
    cells: FixedBitSet,
}

impl Universe {
    pub fn get_index(&self, row: u32, column: u32) -> usize {
            (row * self.width + column) as usize
    }
        
    pub fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.height - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }
                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                count += self.cells[self.get_index(neighbor_row, neighbor_col)] as u8;
            }
        }
        count
    }

    pub fn get_cells(&self) -> &FixedBitSet {
        &self.cells
    }

    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, column) in cells.iter().cloned() {
            self.cells.set(self.get_index(row, column), true);
        }
    }
}

#[wasm_bindgen]
impl Universe {
    
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let index = self.get_index(row, col);
                let cell = self.cells[index];
                let live_neighbors = self.live_neighbor_count(row, col);

                next.set(index, match (cell, live_neighbors) {
                    (true, x) if x < 2 => false,
                    (true, 2) | (true, 3) => true,
                    (true, x) if x > 3 => false,
                    (false, 3) => true,
                    (otherwise, _) => otherwise,
                });
            }
        }
        self.cells = next;
    }

    pub fn new() -> Universe {
        let width = 256;
        let height = 128;

        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);

        for i in 0..size {
            cells.set(i, rand::random::<bool>());
        }

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        self.cells.toggle(self.get_index(row, column));
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        for i in 0..width * self.height {
            self.cells.set(i as usize, false);
        }
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        for i in 0..self.width * height {
            self.cells.set(i as usize, false);
        }
    }
}
