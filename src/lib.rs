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

struct Shape {
    width: u32,
    height: u32,
    cells: FixedBitSet
}

enum AvailableShapes {
    Square,
    Beehive,
    Loaf,
    Boat,
    Tub,
    Blinker,
    Toad,
    Beacon,
    Pulsar,
    PentaDeca,
    Glider,
    Lwss,
    Mwss,
    Hwss,
    GliderGen
}


fn get_index(row: usize, col: usize, width: usize) -> usize {
    ((row*width) + col) as usize
}

fn set_cells(height: usize, width: usize, shape: Vec<Vec<u32>>, cells: &mut FixedBitSet ) {
    for row in 0..height{
        for col in 0..width {
            cells.set(get_index(row,col, width), shape[row][col] != 0);
        }
    }
}
impl Shape {
    pub fn new(shape_name: AvailableShapes) -> Shape {
        let width: usize;
        let height: usize;
        let mut cells: FixedBitSet;
        let shape: Vec<Vec<u32>>;
        match shape_name  {
            AvailableShapes::Square => {
                shape = vec![
                    vec![1,1],
                    vec![1,1]
                ];
            },
            AvailableShapes::Beehive => {
                shape = vec![
                    vec![0,0,1,1,0,0],
                    vec![0,1,0,0,1,0],
                    vec![0,0,1,1,0,0],
                ];
            },
            AvailableShapes::Loaf => {
                shape = vec![
                    vec![0,0,1,1,0,0],
                    vec![0,1,0,0,1,0],
                    vec![0,0,1,0,1,0],
                    vec![0,0,0,1,0,0]
                ];
            },
            AvailableShapes::Boat => {
                shape = vec![
                    vec![0,1,1,0,0],
                    vec![0,1,0,1,0],
                    vec![0,0,1,0,0]
                ];
            },
            AvailableShapes::Tub => {
                shape = vec![
                    vec![0,0,1,0,0],
                    vec![0,1,0,1,0],
                    vec![0,0,1,0,0]
                ];
            }
            AvailableShapes::Blinker => {
                shape = vec![
                    vec![1],
                    vec![1],
                    vec![1]
                ]
            }
            AvailableShapes::Toad => {
                shape = vec![
                    vec![0,0,1,0],
                    vec![1,0,0,1],
                    vec![1,0,0,1],
                    vec![0,1,0,0]
                ];
            }
            AvailableShapes::Beacon => {
                shape = vec![
                    vec![1,1,0,0],
                    vec![1,1,0,0],
                    vec![0,0,1,1],
                    vec![0,0,1,1]
                ];
            }
            AvailableShapes::Pulsar => {
                shape = vec![
                    vec![0,0,1,1,1,0,0,0,1,1,1,0,0],
                    vec![0,0,0,0,0,0,0,0,0,0,0,0,0],
                    vec![1,0,0,0,0,1,0,1,0,0,0,0,1],
                    vec![1,0,0,0,0,1,0,1,0,0,0,0,1],
                    vec![1,0,0,0,0,1,0,1,0,0,0,0,1],
                    vec![0,0,1,1,1,0,0,0,1,1,1,0,0],
                    vec![0,0,0,0,0,0,0,0,0,0,0,0,0],
                    vec![0,0,1,1,1,0,0,0,1,1,1,0,0],
                    vec![1,0,0,0,0,1,0,1,0,0,0,0,1],
                    vec![1,0,0,0,0,1,0,1,0,0,0,0,1],
                    vec![1,0,0,0,0,1,0,1,0,0,0,0,1],
                    vec![0,0,0,0,0,0,0,0,0,0,0,0,0],
                    vec![0,0,1,1,1,0,0,0,1,1,1,0,0]
                ]
            }
            AvailableShapes::PentaDeca => {
                shape = vec![
                    vec![0,0,0,1,1,1,0,0,0],
                    vec![0,0,1,0,0,0,1,0,0],
                    vec![0,1,0,0,0,0,0,1,0],
                    vec![0,0,0,0,0,0,0,0,0],
                    vec![1,0,0,0,0,0,0,0,1],
                    vec![1,0,0,0,0,0,0,0,1],
                    vec![0,0,0,0,0,0,0,0,0],
                    vec![0,1,0,0,0,0,0,1,0],
                    vec![0,0,1,0,0,0,1,0,0],
                    vec![0,0,0,1,1,1,0,0,0]
                ];
            }
            AvailableShapes::Glider => {
                shape = vec![
                    vec![0,0,1],
                    vec![1,0,1],
                    vec![0,1,1]
                ]
            }
            AvailableShapes::Lwss => {
                shape = vec![
                    vec![0,1,1,1,1],
                    vec![1,0,0,0,1],
                    vec![0,0,0,0,1],
                    vec![1,0,0,1,0]
                ]
            }
            AvailableShapes::Mwss => {
                shape = vec![
                    vec![0,0,1,0,0,0],
                    vec![1,0,0,0,1,0],
                    vec![0,0,0,0,0,1],
                    vec![1,0,0,0,0,1],
                    vec![0,1,1,1,1,1]
                ]
            }
            AvailableShapes::Hwss => {
                shape = vec![
                    vec![0,0,1,1,0,0,0],
                    vec![1,0,0,0,0,1,0],
                    vec![0,0,0,0,0,0,1],
                    vec![1,0,0,0,0,0,1],
                    vec![0,1,1,1,1,1,1],

                ]
            }
            AvailableShapes::GliderGen => {
                shape = vec![
                    vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0],
                    vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,1,0,0,0,0,0,0,0,0,0,0,0],
                    vec![0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,1,1],
                    vec![0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,1,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,1,1],
                    vec![1,1,0,0,0,0,0,0,0,0,1,0,0,0,0,0,1,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
                    vec![1,1,0,0,0,0,0,0,0,0,1,0,0,0,1,0,1,1,0,0,0,0,1,0,1,0,0,0,0,0,0,0,0,0,0,0],
                    vec![0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,1,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0],
                    vec![0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
                    vec![0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1],
                ]
            }
        }
        width = shape[0].len();
        height = shape.len();
        let size = width*height;
        cells = FixedBitSet::with_capacity(size);
        set_cells(height, width, shape, &mut cells);
        Shape {
            width: width as u32,
            height: height as u32,
            cells
        }
    }
}



#[wasm_bindgen]
impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
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

    pub fn draw_shape(&mut self, row: u32, col: u32, shape: &str) {
        let shape_to_draw = match shape {
            "box" => Shape::new(AvailableShapes::Square),
            "beehive" => Shape::new(AvailableShapes::Beehive),
            "loaf" => Shape::new(AvailableShapes::Loaf),
            "boat" => Shape::new(AvailableShapes::Boat),
            "tub" => Shape::new(AvailableShapes::Tub),
            "blinker" => Shape::new(AvailableShapes::Blinker),
            "toad" => Shape::new(AvailableShapes::Toad),
            "beacon" => Shape::new(AvailableShapes::Beacon),
            "pulsar" => Shape::new(AvailableShapes::Pulsar),
            "pentadeca" => Shape::new(AvailableShapes::PentaDeca),
            "glider" => Shape::new(AvailableShapes::Glider),
            "lwss" => Shape::new(AvailableShapes::Lwss),
            "mwss" => Shape::new(AvailableShapes::Mwss),
            "hwss" => Shape::new(AvailableShapes::Hwss),
            "glidergen" => Shape::new(AvailableShapes::GliderGen),
            _ => todo!()
        };
        let offset = self.get_index(row, col);
        let mut row_offset: usize;
        let shape_size = shape_to_draw.width * shape_to_draw.height;
        log!("Drawing: {}", shape);
        for i in 0..shape_size {
            row_offset = usize::try_from((self.width-shape_to_draw.width) * (i / shape_to_draw.width)).unwrap();
            let shape_idx = usize::try_from(i).unwrap();
            let univ_idx =  offset + shape_idx + row_offset;
            self.cells.set(univ_idx, shape_to_draw.cells[shape_idx]);
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

