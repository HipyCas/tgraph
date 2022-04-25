use console_engine::Color;
use derivative::Derivative;

#[derive(Copy, Clone, Derivative, Debug)]
#[derivative(Default)]
pub enum Character {
  CrossCircle = '¤' as isize,
  Registered = '®' as isize,
  Times = '×' as isize,
  CyrillicZhe = 'ж' as isize,
  ///   ҉
  CyrillicMillions = '\u{0489}' as isize,
  ///   ҈        
  CyrillicHundredThousands = '\u{0488}' as isize,
  Bullet = '\u{2022}' as isize,        // •
  EmptyBullet = '\u{2218}' as isize,   // ∘
  SmallTriangle = '\u{2023}' as isize, // ‣
  CrossPoint = '\u{203B}' as isize,    // ※
  Asterisk = '\u{2217}' as isize,      // ∗
  #[derivative(Default)]
  Star = '\u{2055}' as isize, // ⁕
  Function = '\u{2061}' as isize,
}

#[derive(Derivative, Copy, Clone)]
#[derivative(Debug = "transparent")]
pub struct ColorWrapper(Color);

impl Default for ColorWrapper {
  fn default() -> ColorWrapper {
    ColorWrapper(Color::Blue)
  }
}

impl From<Color> for ColorWrapper {
  fn from(c: Color) -> ColorWrapper {
    ColorWrapper(c)
  }
}

impl Into<Color> for ColorWrapper {
  fn into(self) -> Color {
    self.0
  }
}
