use std::fmt;

use super::{Value, ValueItem, WrappedValue};

#[derive(Debug, Clone, PartialEq)]
pub enum Brush {
  Color(f64, f64, f64, f64),
}

impl Brush {
  pub fn transparent() -> Self {
    Brush::Color(0.0, 0.0, 0.0, 0.0)
  }
  pub fn __to_css(&self) -> String {
    match self {
      Brush::Color(r, g, b, a) => {
        let r = r * 255.0;
        let g = g * 255.0;
        let b = b * 255.0;
        let a = a * 255.0;
        format!("rgba({r}, {g}, {b}, {a})")
      },
    }
  }
}

impl Value for Brush {
  type Item = Brush;
  fn default() -> Self { Brush::transparent() }
  fn item(value: Self) -> Self::Item { value }
  fn from_item(value: Self::Item) -> Self { value }
  fn wrapped(brush: Self::Item) -> WrappedValue { WrappedValue::Brush(brush) }
}

impl ValueItem for Brush {
  fn unwrapped(value: WrappedValue) -> Self {
    match value {
      WrappedValue::Brush(brush) => brush,
      _ => Brush::default(),
    }
  }
}

impl fmt::Display for Brush {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}
