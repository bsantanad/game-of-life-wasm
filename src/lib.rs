mod utils;

use wasm_bindgen::prelude::*;
use std::fmt;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/*
 * the universe will be respresented as linear array
 * to translate i and j to the index of the array we
 * can do (row * width + column)
 */

#[wasm_bindgen]
extern {
}

/* define cell */
#[wasm_bindgen]
#[repr(u8)] /* each cell is represented as a single byte */
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

/* define universe */
#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {
    // given row and column, return cell index in Vec 
    fn get_index(&self, row: u32, column: u32) -> usize{
        (row * self.width + column) as usize
    }

    // how many neighbors are alive
    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
    count
    }
}

/// public methods, exported to js.
#[wasm_bindgen]
impl Universe{
    // calculate each iteration in the universe 
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    // underpopulation
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // stay alive
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // overpopulation 
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // reproduce 
                    (Cell::Dead, 3) => Cell::Alive,
                    // stay in the same state
                    (otherwise, _) => otherwise,
                };
            next[idx] = next_cell;
            }
        }
        self.cells = next;
    }

    // init universe
    pub fn new() -> Universe {
        let width = 64;
        let height = 32;

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                }else{
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for Universe {
    // print it in a human readable way
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
         for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
