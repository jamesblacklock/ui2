#![allow(unused)]

use core::fmt;

use runtime::{property, property::{value::*, PropertyFactory}};
use runtime::println;

struct Listener<T: fmt::Debug>(T);

impl <T: fmt::Debug + 'static> Listener<T> {
	fn from(value: T) -> Option<Box<dyn property::Listener>> {
		Some(Box::new(Listener(value)))
	}
}

impl <T: fmt::Debug> property::Listener for Listener<T> {
	fn notify(&self) {
		println!("value changed: {:?}", self.0);
	}
}

fn main() {
	let factory = PropertyFactory::new_factory();
}
