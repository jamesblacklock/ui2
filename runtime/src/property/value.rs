use core::fmt;
use std::rc::Rc;
use crate::{println, eprintln};

// any value that can be put into a Property must implement this trait
// a `Value` has an associated `Item` type which is the actual type that is stored
// the `Item` may be the same as the `Value`, or it may be a different
// type. Strings are stored as `Rc<String>`, for example.
pub trait Value: fmt::Debug {
	type Item: ValueItem;
	fn default() -> Self;
	fn item(value: Self) -> Self::Item;
	fn wrapped(value: Self::Item) -> WrappedValue;
}

// Values need to be "wrappable" so that they can be passed uniformly through
// dynamic trait objects
#[derive(Debug, Clone)]
pub enum WrappedValue {
	Boolean(bool),
	Int(i64),
	Float(f64),
	String(Rc<String>),
}

impl WrappedValue {
	pub fn unwrap_boolean(&self) -> bool {
		match self {
			WrappedValue::Boolean(boolean) => *boolean,
			_ => false,
		}
	}
	pub fn unwrap_int(&self) -> i64 {
		match self {
			WrappedValue::Int(int) => *int,
			_ => 0,
		}
	}
	pub fn unwrap_string(&self) -> Rc<String> {
		match self {
			WrappedValue::String(string) => string.clone(),
			_ => Rc::new(String::new()),
		}
	}
	pub fn unwrap_float(&self) -> f64 {
		match self {
			WrappedValue::Float(float) => *float,
			_ => 0.0,
		}
	}
}

pub trait ValueItem: Clone + fmt::Debug + PartialEq {
	fn unwrapped(value: WrappedValue) -> Self;
}

// the implementation of `Value` & `ValueItem` for `f64`
impl Value for f64 {
	type Item = f64;
	fn default() -> Self { 0.0 }
	fn item(value: Self) -> Self::Item { value }
	fn wrapped(value: Self::Item) -> WrappedValue { WrappedValue::Float(value) }
}
impl ValueItem for f64 {
	fn unwrapped(value: WrappedValue) -> Self {
		value.unwrap_float()
	}
}

// the implementation of `Value` & `ValueItem` for `f64`
impl Value for bool {
	type Item = bool;
	fn default() -> Self { false }
	fn item(value: Self) -> Self::Item { value }
	fn wrapped(value: Self::Item) -> WrappedValue { WrappedValue::Boolean(value) }
}
impl ValueItem for bool {
	fn unwrapped(value: WrappedValue) -> Self {
		value.unwrap_boolean()
	}
}

// the implementation of `Value` & `ValueItem` for `i64`
impl Value for i64 {
	type Item = i64;
	fn default() -> Self { 0 }
	fn item(value: Self) -> Self::Item { value }
	fn wrapped(value: Self::Item) -> WrappedValue { WrappedValue::Int(value) }
}
impl ValueItem for i64 {
	fn unwrapped(value: WrappedValue) -> Self {
		value.unwrap_int()
	}
}

// the implementation of `Value` & `ValueItem` for `String`
impl Value for String {
	type Item = Rc<String>;
	fn default() -> Self { String::new() }
	fn item(value: Self) -> Self::Item { Rc::new(value)	}
	fn wrapped(value: Self::Item) -> WrappedValue { WrappedValue::String(value) }
}
impl ValueItem for Rc<String> {
	fn unwrapped(value: WrappedValue) -> Self {
		value.unwrap_string()
	}
}
