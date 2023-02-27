use core::fmt;
use std::cell::RefCell;
use std::rc::Weak;
use crate::{Property, DynProperty, PropertyCell};
use crate::value::{Value, ValueItem, WrappedValue};

pub trait Parents {
	type Values: Values + 'static;
	fn get_values(&self) -> Self::Values;
	fn add_child(&self, transform: Weak<dyn ChildTransformTrait>);
}
impl <Z: Value> Parents for (&Property<Z>,) {
	type Values = (Z::Item,);
	fn get_values(&self) -> Self::Values {
		(self.0.get().clone(),)
	}
	fn add_child(&self, transform: Weak<dyn ChildTransformTrait>){
		self.0.add_child(transform, 0);
	}
}
impl <Z: Value, Y: Value> Parents for (&Property<Z>, &Property<Y>) {
	type Values = (Z::Item, Y::Item);
	fn get_values(&self) -> Self::Values {
		(self.0.get().clone(), self.1.get().clone())
	}
	fn add_child(&self, transform: Weak<dyn ChildTransformTrait>){
		self.0.add_child(transform.clone(), 0);
		self.1.add_child(transform, 1);
	}
}
impl <Z: Value, Y: Value, X: Value> Parents for (&Property<Z>, &Property<Y>, &Property<X>) {
	type Values = (Z::Item, Y::Item, X::Item);
	fn get_values(&self) -> Self::Values {
		(self.0.get().clone(), self.1.get().clone(), self.2.get().clone())
	}
	fn add_child(&self, transform: Weak<dyn ChildTransformTrait>){
		self.0.add_child(transform.clone(), 0);
		self.1.add_child(transform.clone(), 1);
		self.2.add_child(transform, 2);
	}
}
impl <Z: Value, Y: Value, X: Value, W: Value> Parents for (&Property<Z>, &Property<Y>, &Property<X>, &Property<W>) {
	type Values = (Z::Item, Y::Item, X::Item, W::Item);
	fn get_values(&self) -> Self::Values {
		(self.0.get().clone(), self.1.get().clone(), self.2.get().clone(), self.3.get().clone())
	}
	fn add_child(&self, transform: Weak<dyn ChildTransformTrait>){
		self.0.add_child(transform.clone(), 0);
		self.1.add_child(transform.clone(), 1);
		self.2.add_child(transform.clone(), 2);
		self.3.add_child(transform, 3);
	}
}
impl <Z: Value, Y: Value, X: Value, W: Value, V: Value> Parents for (&Property<Z>, &Property<Y>, &Property<X>, &Property<W>, &Property<V>) {
	type Values = (Z::Item, Y::Item, X::Item, W::Item, V::Item);
	fn get_values(&self) -> Self::Values {
		(self.0.get().clone(), self.1.get().clone(), self.2.get().clone(), self.3.get().clone(), self.4.get().clone())
	}
	fn add_child(&self, transform: Weak<dyn ChildTransformTrait>){
		self.0.add_child(transform.clone(), 0);
		self.1.add_child(transform.clone(), 1);
		self.2.add_child(transform.clone(), 2);
		self.3.add_child(transform.clone(), 3);
		self.4.add_child(transform, 4);
	}
}
impl <Z: Value, Y: Value, X: Value, W: Value, V: Value, U: Value> Parents for (&Property<Z>, &Property<Y>, &Property<X>, &Property<W>, &Property<V>, &Property<U>) {
	type Values = (Z::Item, Y::Item, X::Item, W::Item, V::Item, U::Item);
	fn get_values(&self) -> Self::Values {
		(self.0.get().clone(), self.1.get().clone(), self.2.get().clone(), self.3.get().clone(), self.4.get().clone(), self.5.get().clone())
	}
	fn add_child(&self, transform: Weak<dyn ChildTransformTrait>){
		self.0.add_child(transform.clone(), 0);
		self.1.add_child(transform.clone(), 1);
		self.2.add_child(transform.clone(), 2);
		self.3.add_child(transform.clone(), 3);
		self.4.add_child(transform.clone(), 4);
		self.5.add_child(transform, 5);
	}
}
impl <Z: Value, Y: Value, X: Value, W: Value, V: Value, U: Value, T: Value> Parents for (&Property<Z>, &Property<Y>, &Property<X>, &Property<W>, &Property<V>, &Property<U>, &Property<T>) {
	type Values = (Z::Item, Y::Item, X::Item, W::Item, V::Item, U::Item, T::Item);
	fn get_values(&self) -> Self::Values {
		(self.0.get().clone(), self.1.get().clone(), self.2.get().clone(), self.3.get().clone(), self.4.get().clone(), self.5.get().clone(), self.6.get().clone())
	}
	fn add_child(&self, transform: Weak<dyn ChildTransformTrait>){
		self.0.add_child(transform.clone(), 0);
		self.1.add_child(transform.clone(), 1);
		self.2.add_child(transform.clone(), 2);
		self.3.add_child(transform.clone(), 3);
		self.4.add_child(transform.clone(), 4);
		self.5.add_child(transform.clone(), 5);
		self.6.add_child(transform, 6);
	}
}
pub trait Values: Clone + fmt::Debug {
	fn update(&mut self, value: WrappedValue, index: usize);
}
impl <Z: ValueItem,> Values for (Z,) {
	fn update(&mut self, value: WrappedValue, index: usize) {
		match index {
			0 => self.0 = Z::unwrapped(value),
			_ => unreachable!(),
		}
	}
}
impl <Z: ValueItem, Y: ValueItem> Values for (Z, Y) {
	fn update(&mut self, value: WrappedValue, index: usize) {
		match index {
			0 => self.0 = Z::unwrapped(value),
			1 => self.1 = Y::unwrapped(value),
			_ => unreachable!(),
		}
	}
}
impl <Z: ValueItem, Y: ValueItem, X: ValueItem> Values for (Z, Y, X) {
	fn update(&mut self, value: WrappedValue, index: usize) {
		match index {
			0 => self.0 = Z::unwrapped(value),
			1 => self.1 = Y::unwrapped(value),
			2 => self.2 = X::unwrapped(value),
			_ => unreachable!(),
		}
	}
}
impl <Z: ValueItem, Y: ValueItem, X: ValueItem, W: ValueItem> Values for (Z, Y, X, W) {
	fn update(&mut self, value: WrappedValue, index: usize) {
		match index {
			0 => self.0 = Z::unwrapped(value),
			1 => self.1 = Y::unwrapped(value),
			2 => self.2 = X::unwrapped(value),
			3 => self.3 = W::unwrapped(value),
			_ => unreachable!(),
		}
	}
}
impl <Z: ValueItem, Y: ValueItem, X: ValueItem, W: ValueItem, V: ValueItem> Values for (Z, Y, X, W, V) {
	fn update(&mut self, value: WrappedValue, index: usize) {
		match index {
			0 => self.0 = Z::unwrapped(value),
			1 => self.1 = Y::unwrapped(value),
			2 => self.2 = X::unwrapped(value),
			3 => self.3 = W::unwrapped(value),
			4 => self.4 = V::unwrapped(value),
			_ => unreachable!(),
		}
	}
}
impl <Z: ValueItem, Y: ValueItem, X: ValueItem, W: ValueItem, V: ValueItem, U: ValueItem> Values for (Z, Y, X, W, V, U) {
	fn update(&mut self, value: WrappedValue, index: usize) {
		match index {
			0 => self.0 = Z::unwrapped(value),
			1 => self.1 = Y::unwrapped(value),
			2 => self.2 = X::unwrapped(value),
			3 => self.3 = W::unwrapped(value),
			4 => self.4 = V::unwrapped(value),
			5 => self.5 = U::unwrapped(value),
			_ => unreachable!(),
		}
	}
}
impl <Z: ValueItem, Y: ValueItem, X: ValueItem, W: ValueItem, V: ValueItem, U: ValueItem, T: ValueItem> Values for (Z, Y, X, W, V, U, T) {
	fn update(&mut self, value: WrappedValue, index: usize) {
		match index {
			0 => self.0 = Z::unwrapped(value),
			1 => self.1 = Y::unwrapped(value),
			2 => self.2 = X::unwrapped(value),
			3 => self.3 = W::unwrapped(value),
			4 => self.4 = V::unwrapped(value),
			5 => self.5 = U::unwrapped(value),
			6 => self.6 = T::unwrapped(value),
			_ => unreachable!(),
		}
	}
}

pub struct ChildTransform<V: Value + 'static, T: Values, F: Fn(T) -> V> {
	pub values: RefCell<T>,
	pub transform: F,
	pub child: Weak<PropertyCell<V>>,
}

pub trait ChildTransformTrait {
	fn fmt_debug(&self, f: &mut fmt::Formatter) -> fmt::Result;
	fn parent_value_changed(&self, value: WrappedValue, index: usize);
	fn get_child(&self) -> Box<dyn DynProperty>;
	fn update_value(&self);
}

impl fmt::Debug for dyn ChildTransformTrait {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.fmt_debug(f)
	}
}

impl <V: Value, T: Values, F: Fn(T) -> V> ChildTransformTrait for ChildTransform<V, T, F> {
	fn fmt_debug(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self.values)
	}
	fn parent_value_changed(&self, value: WrappedValue, index: usize) {
		self.values.borrow_mut().update(value, index);
	}
	fn get_child(&self) -> Box<dyn DynProperty> {
		DynProperty::clone(&self.child)
	}
	fn update_value(&self) {
		self.child.upgrade().unwrap().borrow_mut().value = V::item((self.transform)(self.values.borrow().clone()));
	}
}
