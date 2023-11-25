mod utils;

use js_sys::Math;
use fixedbitset::FixedBitSet;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};

#[wasm_bindgen]
pub struct Universe {
  width: u32,
  height: u32,
  cells: FixedBitSet,
  context: CanvasRenderingContext2d
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
    let mut next = FixedBitSet::with_capacity(self.cells.len());

    for row in 0..self.height {
      for col in 0..self.width {
        let idx = self.get_index(row, col);
        let live_neighbors = self.live_neighbor_count(row, col);

        if match (self.cells[idx], live_neighbors) {
          (true, 2) => true,
          (true, 3) => true,
          (true, _) => false,
          (false, 3) => true,
          (otherwise, _) => otherwise
        } {
          next.put(idx);
        }
      }
    }

    self.cells = next;
    self.draw();
  }

  fn draw(&self) {
    let ctx = &self.context;
    ctx.clear_rect(
      0f64,
      0f64,
      (self.width * 2).into(),
      (self.height * 2).into()
    );
    ctx.set_fill_style(&JsValue::from_str("#000000"));
    ctx.fill_rect(
      0f64,
      0f64,
      (self.width * 2).into(),
      (self.height * 2).into()
    );
    ctx.set_fill_style(&JsValue::from_str("#ffffff"));
    let size = self.width * self.height;
    for i in 0..size {
      let row = i / self.width;
      let col = i % self.width;
      let bit = self.cells[i as usize] as bool;
      if bit {
        ctx.fill_rect(
          (col * 2).into(),
          (row * 2).into(),
          2f64,
          2f64
        );
      }
    }
  }

  pub fn new(id: &str) -> Universe {
    let width = 512u32;
    let height = 256u32;

    let canvas = get_canvas(id);

    canvas.set_width(width * 2);
    canvas.set_height(height * 2);

    let context = canvas
      .get_context("2d")
      .expect("coudn't get context")
      .expect("no context")
      .dyn_into::<CanvasRenderingContext2d>()
      .unwrap();

    let size = (width * height) as usize;
    let mut cells = FixedBitSet::with_capacity(size);
    for i in 0..size {
      cells.set(i, Math::random() < 0.5);
    }

    Universe {
      width,
      height,
      cells,
      context
    }
  }
}

fn get_canvas(id: &str) -> HtmlCanvasElement {
  let document = web_sys::window().unwrap().document().unwrap();
  let canvas = document.get_element_by_id(id).unwrap();
  let canvas = canvas
    .dyn_into::<HtmlCanvasElement>()
    .map_err(|_| ())
    .unwrap();
  canvas
}
