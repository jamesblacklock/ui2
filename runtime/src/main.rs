use core::fmt;

use runtime::{property, property::PropertyFactory};
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
	let a = factory.int(0, Listener::from("a"));
	let b = factory.bind((&a,), |(a,)| a * 2, Listener::from("b"));
	let c = factory.bind((&b,), |(b,)| b * 2, Listener::from("c"));
	let d = factory.bind((&c,), |(c,)| c * 2, Listener::from("d"));
	let s = factory.bind((&a, &b, &c, &d), |(a, b, c, d)| format!("your final numbers are {a}, {b}, {c}, and {d}!"), Listener::from("s"));
	let t = factory.bind((&a, &s), |(a, s)| format!("{a}{s}{a}"), Listener::from("t"));
	a.set(123);
	// a.set(2);
	// factory.commit_changes();
	// println!("{}", t.get());
	// // s.unbind();
	// a.set(4);
	// factory.commit_changes();
	// println!("{}", t.get());
}
