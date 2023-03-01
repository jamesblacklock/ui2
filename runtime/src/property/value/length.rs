use std::fmt;
use std::ops;

use crate::{println, eprintln};

use super::{Value, ValueItem, WrappedValue};

#[derive(Debug, Clone, PartialEq)]
pub enum Length {
  In(f64),
  Px(f64),
  Cm(f64),
  __HtmlVw(f64),
  __HtmlVh(f64),
  Add(Box<Length>, Box<Length>),
  Sub(Box<Length>, Box<Length>),
  Mul(Box<Length>, f64),
  Div(Box<Length>, f64),
}

impl Length {
  fn try_convert_units(&self, other: &Length) -> Option<f64> {
    match (self, other) {
      (Length::Px(n), Length::Px(_)) => Some(*n),
      (Length::In(n), Length::In(_)) => Some(*n),
      (Length::Cm(n), Length::Cm(_)) => Some(*n),
      (Length::In(n), Length::Cm(_)) => Some(*n * 2.54),
      (Length::Cm(n), Length::In(_)) => Some(*n / 2.54),
      (Length::__HtmlVw(n), Length::__HtmlVw(_)) => Some(*n),
      (Length::__HtmlVh(n), Length::__HtmlVh(_)) => Some(*n),
      _ => None,
    }
  }
}

impl Value for Length {
  type Item = Length;
  fn default() -> Self {
    Length::Px(0.0)
  }
  fn item(value: Self) -> Self::Item {
    value
  }
  fn wrapped(value: Self::Item) -> WrappedValue {
    WrappedValue::Length(value)
  }
}

impl ValueItem for Length {
  fn unwrapped(value: WrappedValue) -> Self {
    match value {
      WrappedValue::Length(value) => value,
      _ => Length::default(),
    }
  }
}

impl ops::Add for Length {
  type Output = Length;
  fn add(self, rhs: Self) -> Self::Output {
    if let Some(rhs) = rhs.try_convert_units(&self) {
      match self {
        Length::In(lhs) => Length::In(lhs + rhs),
        Length::Px(lhs) => Length::Px(lhs + rhs),
        Length::Cm(lhs) => Length::Cm(lhs + rhs),
        Length::__HtmlVw(lhs) => Length::__HtmlVw(lhs + rhs),
        Length::__HtmlVh(lhs) => Length::__HtmlVh(lhs + rhs),
        _ => unreachable!(),
      }
    } else {
      Length::Add(Box::new(self), Box::new(rhs))
    }
  }
}

impl ops::Sub for Length {
  type Output = Length;
  fn sub(self, rhs: Self) -> Self::Output {
    if let Some(rhs) = rhs.try_convert_units(&self) {
      match self {
        Length::In(lhs) => Length::In(lhs - rhs),
        Length::Px(lhs) => Length::Px(lhs - rhs),
        Length::Cm(lhs) => Length::Cm(lhs - rhs),
        Length::__HtmlVw(lhs) => Length::__HtmlVw(lhs - rhs),
        Length::__HtmlVh(lhs) => Length::__HtmlVh(lhs - rhs),
        _ => unreachable!(),
      }
    } else {
      Length::Sub(Box::new(self), Box::new(rhs))
    }
  }
}

impl ops::Mul<f64> for Length {
  type Output = Length;
  fn mul(self, rhs: f64) -> Self::Output {
    match self {
      Length::In(lhs) => Length::In(lhs * rhs),
      Length::Px(lhs) => Length::Px(lhs * rhs),
      Length::Cm(lhs) => Length::Cm(lhs * rhs),
      Length::__HtmlVw(lhs) => Length::__HtmlVw(lhs * rhs),
      Length::__HtmlVh(lhs) => Length::__HtmlVh(lhs * rhs),
      _ => Length::Mul(Box::new(self), rhs),
    }
  }
}

impl ops::Div<f64> for Length {
  type Output = Length;
  fn div(self, rhs: f64) -> Self::Output {
    if rhs == 0.0 || rhs.is_nan() {
      eprintln!("attempted to divide Length by zero");
      return Length::Px(0.0);
    }
    match self {
      Length::In(lhs) => Length::In(lhs / rhs),
      Length::Px(lhs) => Length::Px(lhs / rhs),
      Length::Cm(lhs) => Length::Cm(lhs / rhs),
      Length::__HtmlVw(lhs) => Length::__HtmlVw(lhs / rhs),
      Length::__HtmlVh(lhs) => Length::__HtmlVh(lhs / rhs),
      _ => Length::Mul(Box::new(self), rhs),
    }
  }
}

impl ops::Neg for Length {
  type Output = Length;
  fn neg(self) -> Self::Output {
    match self {
      Length::In(n) => Length::In(-n),
      Length::Px(n) => Length::Px(-n),
      Length::Cm(n) => Length::Cm(-n),
      Length::__HtmlVw(n) => Length::__HtmlVw(-n),
      Length::__HtmlVh(n) => Length::__HtmlVh(-n),
      Length::Add(l, r) => -*l + -*r,
      Length::Sub(l, r) => -*l - -*r,
      Length::Mul(l, r) => *l * -r,
      Length::Div(l, r) => *l / -r,
    }
  }
}

impl fmt::Display for Length {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Length::In(n) => write!(f, "{n}in"),
      Length::Px(n) => write!(f, "{n}px"),
      Length::Cm(n) => write!(f, "{n}cm"),
      Length::__HtmlVw(n) => write!(f, "{n}vw"),
      Length::__HtmlVh(n) => write!(f, "{n}vh"),
      Length::Add(l, r) => write!(f, "({} + {})", l, r),
      Length::Sub(l, r) => write!(f, "({} - {})", l, r),
      Length::Mul(l, r) => write!(f, "({} * {})", l, r),
      Length::Div(l, r) => write!(f, "({} / {})", l, r),
    }
  }
}
