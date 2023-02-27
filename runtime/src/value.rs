use core::fmt;
use std::rc::Rc;

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
pub enum WrappedValue {
	Int(i64),
	String(Rc<String>),
}

pub trait ValueItem: Clone + fmt::Debug {
	fn unwrapped(value: WrappedValue) -> Self;
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
		match value {
			WrappedValue::Int(int) => int,
			_ => unreachable!(),
		}
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
		match value {
			WrappedValue::String(string) => string,
			_ => unreachable!(),
		}
	}
}
