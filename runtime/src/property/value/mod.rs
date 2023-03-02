use std::{fmt, rc::Rc, ops::Deref};
use crate::{println, eprintln};

pub mod length;
pub mod brush;
pub mod layout;
pub mod iter;

use length::Length;
use brush::Brush;
use layout::Layout;

use self::iter::{Iter, WrappedIter, Iterable};

// NOT IMPLEMENTED:
// Component
// Iter
// List

// any value that can be put into a Property must implement this trait
// a `Value` has an associated `Item` type which is the actual type that is stored
// the `Item` may be the same as the `Value`, or it may be a different
// type. Strings are stored as `Rc<String>`, for example.
pub trait Value: fmt::Debug {
	type Item: ValueItem;
	fn default() -> Self;
	fn item(value: Self) -> Self::Item;
	fn from_item(value: Self::Item) -> Self;
	fn wrapped(value: Self::Item) -> WrappedValue;
}

pub trait ToValue {
	type Value: Value;
	fn to_value(self) -> Self::Value;
}

impl <T: Value> ToValue for T {
	type Value = Self;
	fn to_value(self) -> T { self }
}

impl ToValue for &'static str {
	type Value = String;
	fn to_value(self) -> String { self.to_owned() }
}

// Values need to be "wrappable" so that they can be passed uniformly through
// dynamic trait objects
#[derive(Debug, Clone)]
pub enum WrappedValue {
	Boolean(bool),
	Int(i32),
	Float(f64),
	String(Rc<String>),
	Length(Length),
	Brush(Brush),
	EnumLayout(Layout),
	Iter(WrappedIter),
}

impl WrappedValue {
	pub fn unwrap_boolean(&self) -> bool {
		match self {
			WrappedValue::Boolean(boolean) => *boolean,
			WrappedValue::Float(float) => *float != 0.0,
			WrappedValue::Int(int) => *int != 0,
			_ => false,
		}
	}
	pub fn unwrap_int(&self) -> i32 {
		match self {
			WrappedValue::Boolean(boolean) => *boolean as i32,
			WrappedValue::Float(float) => *float as i32,
			WrappedValue::Int(int) => *int,
			_ => 0,
		}
	}
	pub fn unwrap_string(&self) -> Rc<String> {
		match self {
			WrappedValue::Boolean(boolean) => boolean.to_string().into(),
			WrappedValue::Float(float) => float.to_string().into(),
			WrappedValue::Int(int) => int.to_string().into(),
			WrappedValue::String(string) => string.clone(),
			_ => Rc::new(String::new()),
		}
	}
	pub fn unwrap_float(&self) -> f64 {
		match self {
			WrappedValue::Boolean(boolean) => *boolean as i32 as f64,
			WrappedValue::Float(float) => *float,
			WrappedValue::Int(int) => *int as f64,
			_ => 0.0,
		}
	}
	pub fn unwrap_length(&self) -> Length {
		match self {
			WrappedValue::Boolean(boolean) => Length::Px(*boolean as i32 as f64),
			WrappedValue::Float(float) => Length::Px(*float as f64),
			WrappedValue::Int(int) => Length::Px(*int as f64),
			WrappedValue::Length(length) => length.clone(),
			_ => Length::default(),
		}
	}
	pub fn unwrap_brush(&self) -> Brush {
		match self {
			WrappedValue::Brush(brush) => brush.clone(),
			_ => Brush::default(),
		}
	}
	pub fn unwrap_enum_layout(&self) -> Layout {
		match self {
			WrappedValue::EnumLayout(layout) => *layout,
			_ => Layout::default(),
		}
	}
	pub fn unwrap_iter<V: Value>(&self) -> Iter<V> {
		match self {
			WrappedValue::Iter(iter) => iter.clone().iter(),
			_ => Iter::empty(),
		}
	}
}

pub trait ValueItem: Clone + fmt::Debug + PartialEq {
	fn unwrapped(value: WrappedValue) -> Self;
}

// the implementation of `Value` for `f64`
impl Value for f64 {
	type Item = f64;
	fn default() -> Self { 0.0 }
	fn item(value: Self) -> Self::Item { value }
	fn from_item(value: Self::Item) -> Self { value }
	fn wrapped(value: Self::Item) -> WrappedValue { WrappedValue::Float(value) }
}
impl ValueItem for f64 {
	fn unwrapped(value: WrappedValue) -> Self {
		value.unwrap_float()
	}
}

// the implementation of `Value` for `f64`
impl Value for bool {
	type Item = bool;
	fn default() -> Self { false }
	fn item(value: Self) -> Self::Item { value }
	fn from_item(value: Self::Item) -> Self { value }
	fn wrapped(value: Self::Item) -> WrappedValue { WrappedValue::Boolean(value) }
}
impl ValueItem for bool {
	fn unwrapped(value: WrappedValue) -> Self {
		value.unwrap_boolean()
	}
}

// the implementation of `Value` for `i32`
impl Value for i32 {
	type Item = i32;
	fn default() -> Self { 0 }
	fn item(value: Self) -> Self::Item { value }
	fn from_item(value: Self::Item) -> Self { value }
	fn wrapped(value: Self::Item) -> WrappedValue { WrappedValue::Int(value) }
}
impl ValueItem for i32 {
	fn unwrapped(value: WrappedValue) -> Self {
		value.unwrap_int()
	}
}

// the implementation of `Value` for `String`
impl Value for String {
	type Item = Rc<String>;
	fn default() -> Self { String::new() }
	fn item(value: Self) -> Self::Item { Rc::new(value)	}
	fn from_item(value: Self::Item) -> Self { value.deref().clone() }
	fn wrapped(value: Self::Item) -> WrappedValue { WrappedValue::String(value) }
}
impl ValueItem for Rc<String> {
	fn unwrapped(value: WrappedValue) -> Self {
		value.unwrap_string()
	}
}
