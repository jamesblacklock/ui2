#![allow(unused)]

use std::fmt;

use runtime::{property, property::{value::*, value::iter::Iter, PropertyFactory, Property}};
use runtime::println;

struct Listener<T: fmt::Debug>(T, usize);

impl <T: fmt::Debug + Clone + 'static> Listener<T> {
	fn from(value: T) -> Option<Box<dyn property::Listener>> {
		let id = &value as *const T as usize;
		Some(Box::new(Listener(value, id)))
	}
}

impl <T: fmt::Debug + Clone + 'static> property::Listener for Listener<T> {
	fn notify(&self) {
		println!("value changed: {:?}", self.0);
	}
	fn clone(&self) -> Box<dyn property::Listener> {
		Box::new(Listener(self.0.clone(), self.1))
	}
	fn id(&self) -> usize {
		self.1
	}
}

fn main() {
	let factory = PropertyFactory::new_factory();

	let i: Property<Iter<i32>> = factory.new(Iter::empty(), None);

	for n in i.get() {
		println!("{n}");
	}
	println!("{:?}", i);

	i.set(Iter::from(8));
	for n in i.get() {
		println!("{n}");
	}
	println!("{:?}", i);


	i.set(Iter::from(&[420, 69]));
	for n in i.get() {
		println!("{n}");
	}
	println!("{:?}", i);
}
