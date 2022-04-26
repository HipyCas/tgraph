use console_engine::pixel;
use console_engine::screen::Screen;
use console_engine::Color;
use std::fmt;
use std::iter::successors;

use crate::function::Function;
use crate::graph::{GraphOptions, GraphWidths};
use crate::traits::AsF64;

pub struct MultiGraph<X: AsF64, Y: AsF64, F: Fn(X) -> Y> {
  functions: Vec<Function<X, Y, F>>,
  widths: GraphWidths,
  height: u32,
  graph_height: u32,
  options: MultiGraphOptions,
}

pub struct MultiGraphOptions(Vec<GraphOptions>);

impl Default for MultiGraphOptions {
  fn default() -> MultiGraphOptions {
    let colors = [
      Color::Red,
      Color::Blue,
      Color::Green,
      Color::Magenta,
      Color::Cyan,
      Color::Yellow,
      Color::DarkRed,
      Color::DarkBlue,
      Color::DarkGreen,
      Color::DarkMagenta,
      Color::DarkCyan,
      Color::DarkYellow,
    ];
    MultiGraphOptions(
      (0..12)
        .map(|i| GraphOptions {
          color: colors[i].into(),
          ..GraphOptions::default()
        })
        .collect(),
    )
  }
}

impl<X: AsF64, Y: AsF64, F: Fn(X) -> Y> MultiGraph<X, Y, F> {
  pub fn new(
    f: Vec<Function<X, Y, F>>,
    width: u32,
    set_height: Option<u32>,
  ) -> MultiGraph<X, Y, F> {
    MultiGraph::with_options(f, width, set_height, MultiGraphOptions::default())
  }

  pub fn new_screen(f: Vec<Function<X, Y, F>>) -> MultiGraph<X, Y, F> {
    MultiGraph::with_options_screen(f, MultiGraphOptions::default())
  }

  pub fn with_options(
    fs: Vec<Function<X, Y, F>>,
    width: u32,
    set_height: Option<u32>,
    options: MultiGraphOptions,
  ) -> MultiGraph<X, Y, F> {
    // Get max y
    let max = fs
      .iter()
      .map(|f| {
        f.rng(0, width)
          .into_iter()
          .reduce(f64::max)
          .unwrap_or_default()
      })
      .reduce(f64::max)
      .unwrap_or_default()
      .round() as u32;
    // Get digits of maximum number
    let max_height_digits = successors(Some(max), |&n| (n >= 10).then(|| n / 10)).count() as u32;

    let height = match set_height {
      Some(h) => h,
      None => max + 1,
    };
    MultiGraph {
      functions: fs,
      widths: GraphWidths {
        total: width,
        graph: width - max_height_digits,
        height_legend: max_height_digits,
      },
      height,
      graph_height: height - 1,
      options,
    }
  }

  pub fn with_options_screen(
    f: Vec<Function<X, Y, F>>,
    options: MultiGraphOptions,
  ) -> MultiGraph<X, Y, F> {
    let w_screen = console_engine::crossterm::terminal::size().unwrap().0;
    MultiGraph::with_options(f, w_screen as u32, None, options)
  }

  pub fn draw(&self) {
    let mut scr = Screen::new(self.widths.total, self.height);

    self.draw_axis(&mut scr);
    if self.options.0.get(0).unwrap().height_legend {
      self.draw_height_legend(&mut scr);
    }
    self.draw_functions(&mut scr);

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

  fn draw_functions(&self, scr: &mut Screen) {
    for (i, f) in self.functions.iter().enumerate() {
      // Draw points
      for (x, y) in f
        .rng_x(0, self.widths.graph)
        .into_iter()
        .map(|(x, y)| (x, y.round() as u32))
      {
        // println!(
        //   "x: {}; y: {}",
        //   x as i32,
        //   (height - *y.get((x - max_height_digits) as usize).unwrap()) as i32,
        // );
        scr.set_pxl(
          (x + self.widths.height_legend) as i32,
          (self.graph_height - y) as i32, // TODO Allow selecting approximation method: round, ceil or cast (as)
          // Can also put a space (or empty box or something) and color bg
          pixel::pxl_fg(
            self.options.0.get(i).unwrap().character.as_char(),
            self.options.0.get(i).unwrap().color.into(),
          ),
        )
      }
    }
  }
}

impl<X: AsF64, Y: AsF64, F: Fn(X) -> Y> fmt::Display for MultiGraph<X, Y, F> {
  fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
    self.draw();
    Ok(())
  }
}
