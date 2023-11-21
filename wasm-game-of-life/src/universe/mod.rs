//! # Conveys Universe
//!
//! `ConveysUniverse` contains a simple evolutionary algorithm that simulates the life of cells in a grid.

use wasm_bindgen::prelude::*;
mod cell;
use cell::Cell;

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    // matrix is embedded as row segments.
    cells: Vec<Cell>,
}

// Internal Implementations
impl Universe {
    ///! The function `get_index` calculates the index of a 2D array given the row and column numbers.
    ///
    /// Arguments:
    ///
    /// * `row`: The `row` parameter represents the row number.
    /// It is of type `u32`, which means it can hold non-negative integer values.
    /// * `column`: The `column` parameter represents the column
    /// It is also of type `u32`.
    ///
    /// Returns: Cell Index as `usize`
    ///
    /// The function `get_index` returns an index value of type `usize`.
    pub fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    /// The function calculates the number of live neighbors for a given cell in a grid.
    ///
    /// Arguments:
    ///
    /// * `row`: The `row` parameter represents the row index of a cell in a grid.
    /// * `column`: The `column` parameter represents the column index of a cell in a grid. It is used
    /// to calculate the column index of the neighboring cells.
    ///
    /// Returns: `count` as `u8`
    ///
    /// The function `live_neighbor_count` returns a value of type `u8`, which represents the count of
    /// live neighbors for a given cell.
    pub fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
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

    /// Get the dead and alive values of the entire universe.
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
        }
    }
}

/// Public implementations, exported to JavaScript.
#[wasm_bindgen]
impl Universe {
    pub fn place_heart(&mut self, row: u32, col: u32) {
        /*
        o o o o o
        o 1 o 1 o
        1 1 x 1 1
        o 1 1 1 o
        o o 1 o o
         */

        if row < 3 || col < 3 || row >= self.height || col >= self.width {
            return;
        }

        let heart = [
            (row+1, col+1),
            (row+1, col-1),
            (row, col),
            (row, col-1),
            (row, col+1),
            (row, col-2),
            (row, col+2),
            (row-1, col),
            (row-1, col-1),
            (row-1, col+1),
            (row-2, col),
        ];

        heart.iter().for_each(|(row, col)|{
            let idx = self.get_index(*row, *col);
            self.cells[idx] = Cell::Alive;
        });

    }

    pub fn place_church(&mut self, row: u32, col: u32) {
        /*
        o o 1 o o
        o 1 1 1 o
        o 1 x 1 o
        1 1 1 1 1
        1 1 1 1 1
         */

        if row < 3 || col < 3 || row >= self.height || col >= self.width {
            return;
        }

        let church = [
            (row +2, col),
            (row+1, col),
            (row+1, col+1),
            (row+1, col-1),
            (row, col),
            (row, col-1),
            (row, col+1),
            (row-1, col),
            (row-1, col-1),
            (row-1, col+1),
            (row-2, col),
            (row-2, col-1),
            (row-2, col+1),
        ];

        church.iter().for_each(|(row, col)|{
            let idx = self.get_index(*row, *col);
            self.cells[idx] = Cell::Alive;
        });

    }
    
    pub fn place_glider(&mut self, row: u32, col: u32) {
        /*
        o 1 o
        o x 1
        1 1 1
         */

        if (row == 0 || col == 0 || row == self.height || col == self.width) {
            return;
        }

        let glider = [
            (row - 1, col),
            (row, col + 1),
            (row + 1, col),
            (row + 1, col + 1),
            (row + 1, col - 1),
        ];

        glider.iter().for_each(|(row, col)| {
            let idx = self.get_index(*row, *col);
            self.cells[idx] = Cell::Alive;
        })

        /*

        let glider = [
            (row -1, col),
            (row, col +1),
            (row +1, col +1),
            (row +1, col),
            (row +1, col -1),
        ];

        glider.iter().for_each(|(row, column)|{
            let idx = self.get_index(*row, *column);
            self.cells[idx] = Cell::Alive;
        }) */
    }

    /// The tick function updates the state of each cell in a grid. (e.g. clock-tick)
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }

    pub fn new(width: Option<u32>, height: Option<u32>) -> Universe {
        let width = width.unwrap_or(64);
        let height = height.unwrap_or(64);

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        // panic!("PANIC 🔥 (test)");

        Universe {
            width,
            height,
            cells,
        }
    }

    // Actually renders the matrix as a (multiline) string.
    pub fn render(&self) -> String {
        self.to_string()
    }

    // for canvas rendering..
    pub fn width(&self) -> u32 {
        self.width
    }

    // for canvas rendering..
    pub fn height(&self) -> u32 {
        self.height
    }

    // for canvas rendering..
    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    /// Set the width of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width * self.height).map(|_i| Cell::Dead).collect();
    }

    /// Set the height of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..self.width * height).map(|_i| Cell::Dead).collect();
    }

    ///
    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells[idx].toggle();
    }
}

// Implementing Display trait (shared characteristics/properties) for Universe.
use std::fmt;

impl fmt::Display for Universe {
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
