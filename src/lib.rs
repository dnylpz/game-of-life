#[macro_use]
mod utils;

use wasm_bindgen::prelude::*;

use std::convert::TryFrom;
extern crate js_sys;
extern crate fixedbitset;
use fixedbitset::FixedBitSet;


#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet
}

#[wasm_bindgen]
impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column ) as usize
    }

    fn live_neighbot_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col  == 0 {
                    continue;
                }
                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column+delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();
        
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbot_count(row, col);

                
                next.set(idx,match(cell, live_neighbors) {
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
        utils::set_panic_hook();
        let width = 64;
        let height = 64;
        
        let size = (width * height ) as usize;
        let mut cells = FixedBitSet::with_capacity(size);
        
        for i in 0..size {
            cells.set(i, i % 2 == 0 || i % 7 == 0);
        }

        Universe {
            width,
            height,
            cells
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

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        for i in 0..width * self.height {
            self.cells.set(usize::try_from(i).unwrap(), false);
        }
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        for i in  0..self.width * height {
            self.cells.set(usize::try_from(i).unwrap(), false);
        }
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells.set(idx, !self.cells[idx]);
    }

    pub fn reset(&mut self)  {
        let size = (self.height * self.width) as usize;
        self.cells = FixedBitSet::with_capacity(size);
    }

    pub fn randomize(&mut self) {
        self.reset();
        for i in 0..(self.width * self.height) {
            self.cells.set(usize::try_from(i).unwrap(), js_sys::Math::random() < 0.5 );
        }
    }

}



impl Universe {
    pub fn get_cells(&self) -> &FixedBitSet {
        &self.cells
    }

    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells.set(idx,true);
        }
    }
}

