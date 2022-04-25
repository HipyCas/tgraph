pub trait AsF64 {
  fn as_f64(self) -> f64;
  fn from_f64(v: f64) -> Self;
}

impl AsF64 for u8 {
  fn as_f64(self) -> f64 {
    self as f64
  }

  fn from_f64(v: f64) -> u8 {
    v.round() as u8
  }
}

impl AsF64 for u16 {
  fn as_f64(self) -> f64 {
    self as f64
  }

  fn from_f64(v: f64) -> u16 {
    v.round() as u16
  }
}

impl AsF64 for u32 {
  fn as_f64(self) -> f64 {
    self as f64
  }

  fn from_f64(v: f64) -> u32 {
    v.round() as u32
  }
}

impl AsF64 for u64 {
  fn as_f64(self) -> f64 {
    self as f64
  }

  fn from_f64(v: f64) -> u64 {
    v.round() as u64
  }
}

impl AsF64 for u128 {
  fn as_f64(self) -> f64 {
    self as f64
  }

  fn from_f64(v: f64) -> u128 {
    v.round() as u128
  }
}

impl AsF64 for i8 {
  fn as_f64(self) -> f64 {
    self as f64
  }

  fn from_f64(v: f64) -> i8 {
    v.round() as i8
  }
}

impl AsF64 for i16 {
  fn as_f64(self) -> f64 {
    self as f64
  }

  fn from_f64(v: f64) -> i16 {
    v.round() as i16
  }
}

impl AsF64 for i32 {
  fn as_f64(self) -> f64 {
    self as f64
  }

  fn from_f64(v: f64) -> i32 {
    v.round() as i32
  }
}

impl AsF64 for i64 {
  fn as_f64(self) -> f64 {
    self as f64
  }

  fn from_f64(v: f64) -> i64 {
    v.round() as i64
  }
}

impl AsF64 for i128 {
  fn as_f64(self) -> f64 {
    self as f64
  }

  fn from_f64(v: f64) -> i128 {
    v.round() as i128
  }
}

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