use console_engine::Color;
use derivative::Derivative;
use std::fmt;
use std::fmt::Write;

/// Enum used in `tgraph` to represent painting character.
#[derive(Copy, Clone, Derivative, Debug)]
#[derivative(Default)]
pub enum Character {
    /// Unicode circle with cross character (¤)
    CrossCircle,
    /// Copyright character (®)
    Copyright,
    /// Backwards compatibility with `Character::Copyright`
    Registered,
    /// Unicode times/multiplication symbol (×)
    Times,
    /// Unicode Cyrillic Zhe letter character (ж)
    CyrillicZhe,
    /// Unicode Cyrillic millions character (    ҉ )
    CyrillicMillions,
    /// Unicode Cyrillic hundred thousands character (    ҈ )
    CyrillicHundredThousands,
    /// Unicode bullet character (•)
    Bullet,
    /// Unicode empty bullet character (∘)
    EmptyBullet,
    /// Unicode small right-pointing triangle (‣)
    SmallTriangle,
    /// Unicode cross with points character (※)
    CrossPoint,
    /// ASCII and Unicode asterisk character (∗)
    Asterisk,
    #[derivative(Default)]
    /// **(Default)** Unicode star character (⁕)
    Star,
    /// Tuple wrapper for rust `char` type, allows to use
    Custom(char),
}

impl Character {
    /// Transforms the `Character` variant to the corresponding Rust `char` character. For the `Character::Custom(char)` variant, it returns the `char` that the variant wraps.
    pub fn as_char(&self) -> char {
        match self {
            Character::CrossCircle => '¤',
            Character::Copyright | Character::Registered => '®',
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
    /// Wraps the `char` in a `Character::Custom(char)` variant.
    fn from(c: char) -> Character {
        Character::Custom(c)
    }
}

impl From<Character> for char {
    /// Calls `Character::as_char(&self)`
    fn from(val: Character) -> Self {
        val.as_char()
    }
}

impl fmt::Display for Character {
    /// Prints the character obtained from calling `Character::as_char()` on `self`
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

impl From<ColorWrapper> for Color {
    fn from(val: ColorWrapper) -> Self {
        val.0
    }
}

#[derive(Derivative)]
#[derivative(Default, Debug)]
pub struct Scales {
    #[derivative(Default(value = "1f64"))]
    pub x: f64,
    #[derivative(Default(value = "1f64"))]
    pub y: f64,
}

impl From<(f64, f64)> for Scales {
    fn from((x, y): (f64, f64)) -> Scales {
        Scales { x, y }
    }
}

impl From<(Option<f64>, Option<f64>)> for Scales {
    fn from((maybe_x, maybe_y): (Option<f64>, Option<f64>)) -> Scales {
        Scales {
            x: maybe_x.unwrap_or(1f64),
            y: maybe_y.unwrap_or(1f64),
        }
    }
}
