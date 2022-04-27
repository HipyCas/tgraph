use derivative::Derivative;
use std::marker::PhantomData;

use crate::AsF64;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Function<X: AsF64, Y: AsF64, F: Fn(X) -> Y> {
  #[derivative(Debug = "ignore")]
  f: F,
  #[derivative(Debug = "ignore")]
  _x: PhantomData<X>,
  #[derivative(Debug = "ignore")]
  _y: PhantomData<Y>,
}

impl<X: AsF64, Y: AsF64, F: Fn(X) -> Y> Function<X, Y, F> {
  pub fn new(f: F) -> Function<X, Y, F> {
    Function {
      f,
      _x: PhantomData,
      _y: PhantomData,
    }
  }

  pub fn at(&self, x: f64) -> f64 {
    (self.f)(X::from_f64(x)).as_f64()
  }

  pub fn pt(&self, x: f64) -> (f64, f64) {
    let y = self.at(x);
    (x, y)
  }

  pub fn rng(&self, x_i: u32, x_f: u32) -> Vec<f64> {
    (x_i..=x_f)
      .map(|x| (self.f)(X::from_f64(x.as_f64())).as_f64())
      .collect()
  }

  pub fn rng_x(&self, x_i: u32, x_f: u32) -> Vec<(u32, f64)> {
    (x_i..=x_f)
      .map(|x| (x, (self.f)(X::from_f64(x.as_f64())).as_f64()))
      .collect()
  }
}

impl<X: AsF64 + Copy, Y: AsF64, F: Fn(X) -> Y> IntoIterator for Function<X, Y, F> {
  type Item = Y;
  type IntoIter = FunctionIntoIterator<X, Y, F>;

  fn into_iter(self) -> Self::IntoIter {
    FunctionIntoIterator {
      func: self,
      x: X::from_f64(0_f64),
    }
  }
}

pub struct FunctionIntoIterator<X: AsF64, Y: AsF64, F: Fn(X) -> Y> {
  func: Function<X, Y, F>,
  x: X,
}

impl<X: AsF64 + Copy, Y: AsF64, F: Fn(X) -> Y> Iterator for FunctionIntoIterator<X, Y, F> {
  type Item = Y;
  fn next(&mut self) -> Option<Y> {
    let res = self.func.at(self.x.as_f64());
    self.x = X::from_f64(self.x.as_f64() + 1.0);
    Some(Y::from_f64(res))
  }
}

#[macro_export]
macro_rules! func {
  (|$x:ident| $code:expr) => {
    Function::new(|$x: f64| -> f64 { $code } as fn(f64) -> f64)
  };
  ($e:expr) => {
    Function::new($e)
  };
  ($x1:ident -> $code1:expr) => {
    Function::new(|$x1: f64| -> f64 { $code1 } as fn(f64) -> f64)
  };
  ($x2:ident [$xt:ty] -> $code2:expr) => {
    Function::new(|$x2: $xt| -> f64 { $code2 } as fn($xt) -> f64)
  };
  ($x3:ident [$xt1:ty] -> $code3:expr => [$yt:ty]) => {
    Function::new(|$x3: $xt1| -> $yt { $code3 } as fn($xt1) -> $yt)
  };
  ($x4:ident -> $code4:expr => [$yt1:ty]) => {
    Function::new(|$x4: f64| -> $yt1 { $code4 } as fn(f64) -> $yt1)
  };
}
