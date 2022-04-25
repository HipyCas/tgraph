use console_engine::pixel;
use console_engine::screen::Screen;
use derivative::Derivative;
use std::fmt;
use std::iter::successors;
use typed_builder::TypedBuilder;

use crate::types::{Character, ColorWrapper};

#[derive(Derivative, TypedBuilder, Debug)]
#[derivative(Default)]
pub struct GraphOptions {
  #[builder(default)]
  pub color: ColorWrapper,
  #[builder(default)]
  pub character: Character,
  #[derivative(Default(value = "true"))]
  #[builder(default = true)]
  pub height_legend: bool,
}

pub struct Graph<F: Fn(f64) -> f64> {
  f: F,
  widths: GraphWidths,
  height: u32,
  graph_height: u32,
  pts: Vec<(f64, f64)>,
  options: GraphOptions,
}

pub struct GraphWidths {
  pub total: u32,
  pub graph: u32,
  pub height_legend: u32,
}

impl<F> Graph<F>
where
  F: Fn(f64) -> f64,
{
  /// `width` refers to the total width of the graph, meaning that the function will be printed from 0 to `width - 1 - max_height_number_digits`
  pub fn new(f: F, width: u32, set_height: Option<u32>) -> Graph<F> {
    Graph::with_options(f, width, set_height, GraphOptions::default())
  }

  pub fn new_screen(f: F) -> Graph<F> {
    Graph::with_options_screen(f, GraphOptions::default())
  }

  pub fn with_options(
    f: F,
    width: u32,
    set_height: Option<u32>,
    options: GraphOptions,
  ) -> Graph<F> {
    // Generate function (x, y) pairs
    let mut pts = (0..width)
      .map(|x| (x as f64, f(x as f64)))
      .collect::<Vec<(f64, f64)>>();
    // Get max y
    let max = pts
      .iter()
      .map(|(_, y)| *y)
      .fold(f64::NEG_INFINITY, f64::max)
      .ceil() as u32;
    // Get digits of maximum number
    let max_height_digits = successors(Some(max), |&n| (n >= 10).then(|| n / 10)).count() as u32;
    // Remove elements that shouldn't be printed because of legend
    for _ in 0..max_height_digits {
      pts.pop();
    }
    let height = match set_height {
      Some(h) => h,
      None => max + 1,
    };
    Graph {
      f,
      widths: GraphWidths {
        total: width,
        graph: width - max_height_digits,
        height_legend: max_height_digits,
      },
      height,
      graph_height: height - 1,
      pts,
      options,
    }
  }

  pub fn with_options_screen(f: F, options: GraphOptions) -> Graph<F> {
    let w_screen = console_engine::crossterm::terminal::size().unwrap().0;
    Graph::with_options(f, w_screen as u32, None, options)
  }

  pub fn draw(&self) {
    // draw(
    //   |x| (self.f)(x as f64).round() as u32,
    //   self.options.character,
    // )
    let mut scr = Screen::new(self.widths.total, self.height);

    self.draw_axis(&mut scr);
    if self.options.height_legend {
      self.draw_height_legend(&mut scr);
    }
    self.draw_function(&mut scr);

    scr.draw();
  }

  fn draw_axis(&self, scr: &mut Screen) {
    // Draw axis
    scr.h_line(
      (self.widths.height_legend + 1) as i32,
      self.graph_height as i32,
      self.widths.graph as i32,
      pixel::pxl('_'),
    );
    scr.v_line(
      self.widths.height_legend as i32,
      0,
      self.height as i32,
      pixel::pxl('|'),
    ); // looks like top boundary is not included
  }

  fn draw_height_legend(&self, scr: &mut Screen) {
    for h in 0..=self.graph_height {
      for (index, digit) in h.to_string().chars().enumerate() {
        scr.set_pxl(
          index as i32,
          (self.graph_height - h) as i32,
          pixel::pxl(digit),
        );
      }
    }
  }

  fn draw_function(&self, scr: &mut Screen) {
    // Draw points
    for (x, y) in self.pts.iter() {
      // println!(
      //   "x: {}; y: {}",
      //   x as i32,
      //   (height - *y.get((x - max_height_digits) as usize).unwrap()) as i32,
      // );
      scr.set_pxl(
        *x as i32 + self.widths.height_legend as i32,
        (self.graph_height as f64 - y).round() as i32, // TODO Allow selecting approximation method: round, ceil or cast (as)
        // Can also put a space (or empty box or something) and color bg
        pixel::pxl_fg(
          unsafe { std::char::from_u32_unchecked(self.options.character as u32) },
          self.options.color.into(),
        ),
      )
    }
  }
}

impl<F: Fn(f64) -> f64> fmt::Display for Graph<F> {
  fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
    self.draw();
    Ok(())
  }
}
