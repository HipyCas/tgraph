use console_engine::pixel;
use console_engine::screen::Screen;
use std::iter::successors;

mod function;
mod graph;
mod multi_graph;
mod traits;
mod types;

pub use crate::function::Function;
pub use crate::graph::*;
pub use crate::multi_graph::*;
pub use crate::traits::AsF64;
pub use crate::types::{Character, ColorWrapper};

pub use console_engine::Color;

#[doc(hidden)]
pub fn draw<F: Fn(u32) -> u32>(f: F, character: Character) {
  // Set width
  let width = 80;

  // Generate values
  let y: Vec<u32> = (0..=width).map(|x| f(x)).collect();

  // Get maximum and minimum value
  let max = *y.iter().max().unwrap_or(&0);
  let min = *y.iter().max().unwrap_or(&0);

  // Set graph height
  let height = max + 1;

  let max_height_digits = successors(Some(height), |&n| (n >= 10).then(|| n / 10)).count() as u32;

  // println!("H: {}; W: {}, y {:?}", height, width, y);

  let mut scr = Screen::new(width + 1 + max_height_digits, height + 1);
  // Draw axis
  scr.h_line(
    (max_height_digits + 1) as i32,
    height as i32,
    width as i32,
    pixel::pxl('_'),
  );
  scr.v_line(max_height_digits as i32, 0, height as i32, pixel::pxl('|')); // looks like top boundary is not included

  // Draw height numbers
  for h in 0..=height {
    for (index, digit) in h.to_string().chars().enumerate() {
      scr.set_pxl(index as i32, (height - h) as i32, pixel::pxl(digit));
    }
  }
  scr.draw();
}
