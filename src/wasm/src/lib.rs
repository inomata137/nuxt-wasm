mod utils;

use js_sys::Math;
use fixedbitset::FixedBitSet;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Universe {
  width: u32,
  height: u32,
  cells: FixedBitSet,
}

#[wasm_bindgen]
impl Universe {
  fn get_index(&self, row: u32, column: u32) -> usize {
    (row * self.width + column) as usize
  }

  fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
    let mut count: u8 = 0;
    for dr in [self.height - 1, 0, 1] {
      for dc in [self.width - 1, 0, 1] {
        if dr == 0 && dc == 0 {
          continue;
        }
        let row_idx = (row + dr) % self.height;
        let col_idx = (column + dc) % self.width;
        let idx = self.get_index(row_idx, col_idx);
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
        let live_neighbors = self.live_neighbor_count(row, col);

        let next_cell = match (self.cells[idx], live_neighbors) {
          (true, 2) => true, // keep
          (true, 3) => true, // keep
          (true, _) => false, // toggle
          (false, 3) => true, // toggle
          (otherwise, _) => otherwise, // keep
        };

        next.set(idx, next_cell);
      }
    }

    self.cells = next;
  }

  pub fn new() -> Universe {
    let width = 512u32;
    let height = 256u32;
    let size = (width * height) as usize;
    let mut cells = FixedBitSet::with_capacity(size);
    for i in 0..size {
      cells.set(i, Math::random() < 0.5);
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
}
