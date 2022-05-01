use derivative::Derivative;
use std::marker::PhantomData;

use crate::traits::AsF64;
use crate::types::Scales;

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

    pub fn rng_x_scale(&self, x_i: u32, x_f: u32, scales: &Scales) -> Vec<(f64, f64)> {
        (x_i..=x_f)
            .map(|x| {
                (
                    x as f64 * scales.x,
                    (self.f)(X::from_f64(x.as_f64() * scales.x)).as_f64() * scales.y,
                )
            })
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

/// A handy macro for creating [`Function`] instances easier.
///
/// This macro has three possible syntaxes for defining functions: closure-like, raw and arrow. It casts the created closure to `fn($x_type) -> $y_type` to ensure that they have the same type signature (needed for usage in [`MultiGraph`](struct.MultiGraph.html) ).
///
/// The first of them is just the same syntax as a simple closure, without possible type annotations, as the type is to be set to `f64`. On the other hand, you have the raw syntax, which allows basically anything, included a typed closure, but casting to an `fn(T) -> U` is required when using [`MultiGraph`](struct.MultiGraph.html) so all functions have the same signature. An example usage is the following:
///
/// ```
/// use tgraph::{MultiGraph, func};
///
/// MultiGraph::new_screen(vec![
///   func!(|x| f64::sin(x/2f64).abs() * 4f64),
///   func!(|x: f64| -> f64 {x.ln()} as fn(f64) -> f64),
/// ]).draw();
/// ```
///
#[macro_export]
macro_rules! func {
    (|$x:ident| $code:expr) => {
        tgraph::Function::new(|$x: f64| -> f64 { $code } as fn(f64) -> f64)
    };
    ($e:expr) => {
        tgraph::Function::new($e)
    };
    ($x:ident -> $code:expr) => {
        tgraph::Function::new(|$x: f64| -> f64 { $code } as fn(f64) -> f64)
    };
    ($x:ident [$xt:ty] -> $code:expr) => {
        tgraph::Function::new(|$x: $xt| -> f64 { $code } as fn($xt) -> f64)
    };
    ($x:ident [$xt:ty] -> $code:expr => [$yt:ty]) => {
        tgraph::Function::new(|$x: $xt| -> $yt { $code } as fn($xt) -> $yt)
    };
    ($x:ident -> $code:expr => [$yt:ty]) => {
        tgraph::Function::new(|$x: f64| -> $yt { $code } as fn(f64) -> $yt)
    };
}
