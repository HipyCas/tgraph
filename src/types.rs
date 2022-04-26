use console_engine::Color;
use derivative::Derivative;
use std::fmt;
use std::fmt::Write;

#[derive(Copy, Clone, Derivative, Debug)]
#[derivative(Default)]
pub enum Character {
  CrossCircle,
  Registered,
  Times,
  CyrillicZhe,
  ///   ҉
  CyrillicMillions,
  ///   ҈        
  CyrillicHundredThousands,
  Bullet,        // •
  EmptyBullet,   // ∘
  SmallTriangle, // ‣
  CrossPoint,    // ※
  Asterisk,      // ∗
  #[derivative(Default)]
  Star, // ⁕
  Custom(char),
}

impl Character {
  pub fn as_char(&self) -> char {
    match self {
      Character::CrossCircle => '¤',
      Character::Registered => '®',
      Character::Times => '×',
      Character::CyrillicZhe => 'ж',
      Character::CyrillicMillions => '\u{0489}',
      Character::CyrillicHundredThousands => '\u{0488}',
      Character::Bullet => '\u{2022}',
      Character::EmptyBullet => '\u{2218}',
      Character::SmallTriangle => '\u{2023}',
      Character::CrossPoint => '\u{203B}',
      Character::Asterisk => '\u{2217}',
      Character::Star => '\u{2055}',
      Character::Custom(c) => *c,
    }
  }
}

impl From<char> for Character {
  fn from(c: char) -> Character {
    Character::Custom(c)
  }
}

impl Into<char> for Character {
  fn into(self) -> char {
    self.as_char()
  }
}

impl fmt::Display for Character {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
    f.write_char(self.as_char())
  }
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
