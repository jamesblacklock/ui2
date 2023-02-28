use runtime::property::PropertyFactory;
use runtime::println;

fn main() {
	let factory = PropertyFactory::new_factory();
	let a = factory.int(0);
	let b = factory.bind((&a,), |(a,)| a * 2);
	let c = factory.bind((&b,), |(b,)| b * 2);
	let d = factory.bind((&c,), |(c,)| c * 2);
	let s = factory.bind((&a, &b, &c, &d), |(a, b, c, d)| format!("your final numbers are {a}, {b}, {c}, and {d}!"));
	let t = factory.bind((&a, &s), |(a, s)| format!("{a}{s}{a}"));
	a.set(2);
	factory.commit_changes();
	println!("{}", t.get());
	// s.unbind();
	a.set(4);
	factory.commit_changes();
	println!("{}", t.get());
}
