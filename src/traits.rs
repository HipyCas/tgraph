pub trait AsF64 {
  fn as_f64(self) -> f64;
  fn from_f64(v: f64) -> Self;
}

macro_rules! impl_asf64 {
  ($type:ty) => {
    impl AsF64 for $type {
      fn as_f64(self) -> f64 {
        self as f64
      }
      fn from_f64(v: f64) -> $type {
        v.round() as $type
      }
    }
  };
}

impl_asf64!(u8);
impl_asf64!(u16);
impl_asf64!(u32);
impl_asf64!(u64);
impl_asf64!(u128);
impl_asf64!(i8);
impl_asf64!(i16);
impl_asf64!(i32);
impl_asf64!(i64);
impl_asf64!(i128);

impl AsF64 for f32 {
  fn as_f64(self) -> f64 {
    self as f64
  }

  fn from_f64(v: f64) -> f32 {
    v as f32
  }
}

impl AsF64 for f64 {
  fn as_f64(self) -> f64 {
    self
  }

  fn from_f64(v: f64) -> f64 {
    v
  }
}

pub trait MaybeAsF64 {
  fn maybe_as_f64(self) -> Option<f64>;
}

impl<T> MaybeAsF64 for T
where
  T: AsF64,
{
  fn maybe_as_f64(self) -> Option<f64> {
    Some(self.as_f64())
  }
}

impl<T> MaybeAsF64 for Option<T>
where
  T: AsF64,
{
  fn maybe_as_f64(self) -> Option<f64> {
    self.map(|v| v.as_f64())
  }
}
