// MISSING FEATURES FROM ORIGINAL API:
// - transitions
// - (optional) interpolation on `get()` using `transitionStartTime` and `prevValue`
// - `notify` & `onChange`

mod value;
mod transform;

use core::fmt;
use std::{rc::{Rc, Weak}, cell::{RefCell, RefMut}, mem, collections::HashSet, hash::Hash, ptr};

use transform::{ChildTransformTrait, ChildTransform, Parents};

use crate::value::{Value};

#[derive(Debug)]
struct PropertyFactory(Rc<RefCell<PropertyFactoryImpl>>);

#[derive(Debug)]
pub struct PropertyFactoryImpl {
	count: usize,
	change_set: HashSet<Box<dyn DynProperty>>,
}

impl PropertyFactory {
	#[allow(unused)]
	pub fn new_factory() -> Self {
		PropertyFactory(Rc::new(RefCell::new(PropertyFactoryImpl {
			count: 0,
			change_set: HashSet::new(),
		})))
	}

	#[allow(unused)]
	pub fn new<V: Value>(&self) -> Property<V> {
		let value = V::item(V::default());
		Property::new(Rc::downgrade(&self.0), value)
	}

	#[allow(unused)]
	pub fn int<I: Into<i64>>(&self, value: I) -> Property<i64> {
		let value = Value::item(value.into());
		Property::new(Rc::downgrade(&self.0), value)
	}

	#[allow(unused)]
	pub fn string<S: Into<String>>(&self, value: S) -> Property<String> {
		let value = Value::item(value.into());
		Property::new(Rc::downgrade(&self.0), value)
	}

	#[allow(unused)]
	pub fn bind<V: Value, P: Parents, T: Fn(P::Values) -> V + 'static>(&self, parents: P, transform: T) -> Property<V> {
		let p = Property::new(Rc::downgrade(&self.0), V::item(V::default()));
		p.bind(parents, transform);
		p
	}

	#[allow(unused)]
	pub fn commit_changes(&self) {
		while self.0.borrow().change_set.len() > 0 {
			let change_set = mem::replace(&mut self.0.borrow_mut().change_set, HashSet::new());
			for prop in change_set {
				prop.commit_changes();
			}
		}
	}

	#[allow(unused)]
	pub fn count(&self) -> usize {
		self.0.borrow().count
	}
}

type PropertyCell<V> = RefCell<PropertyImpl<V>>;

pub struct Property<V: Value + 'static>(Rc<PropertyCell<V>>);

#[derive(Debug)]
pub struct PropertyImpl<V: Value + 'static> {
	value: V::Item,
	readonly: bool,
	transform: Option<Rc<dyn ChildTransformTrait>>,
	children: Vec<(Weak<dyn ChildTransformTrait>, usize)>,
	factory: Weak<RefCell<PropertyFactoryImpl>>,
}

pub trait DynProperty {
	fn fmt_debug(&self, f: &mut fmt::Formatter) -> fmt::Result;
	fn commit_changes(&self);
	fn clone(&self) -> Box<dyn DynProperty>;
}

impl Hash for Box<dyn DynProperty> {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		ptr::hash(self, state)
	}
}
impl PartialEq for Box<dyn DynProperty> {
	fn eq(&self, other: &Self) -> bool {
    let left: *const dyn DynProperty = self.as_ref();
    let right: *const dyn DynProperty = other.as_ref();
    left == right
	}
}
impl Eq for Box<dyn DynProperty> {}

impl fmt::Debug for dyn DynProperty {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.fmt_debug(f)
	}
}

impl <V: Value> DynProperty for Weak<RefCell<PropertyImpl<V>>> {
	fn fmt_debug(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if let Some(cell) = self.upgrade() {
			write!(f, "{:#?}", cell)
		} else {
			write!(f, "<dead ref>")
		}
	}
	fn commit_changes(&self) {
		if let Some(cell) = self.upgrade() {
			let transform = cell.borrow().transform.clone();
			if let Some(transform) = transform {
				transform.update_value();
			}

			let mut children = mem::replace(&mut cell.borrow_mut().children, Vec::new());
			let mut new_children = Vec::new();
			let value = &mut cell.borrow().value.clone();
			while let Some((transform, index)) = children.pop() {
				if let Some(transform) = transform.upgrade() {
					transform.parent_value_changed(V::wrapped(value.clone()), index);
					new_children.push((Rc::downgrade(&transform), index));
					cell.borrow().factory.upgrade().unwrap().borrow_mut().change_set.insert(transform.get_child());
				} else {
					println!("child is gone!");
				}
			}
			cell.borrow_mut().children = new_children;
		}
	}
	fn clone(&self) -> Box<dyn DynProperty> {
		Box::new(Clone::clone(self))
	}
}

impl <V: Value> Property<V> {
	#[allow(unused)]
	pub fn get(&self) -> V::Item {
		self.0.borrow().value.clone()
	}

	#[allow(unused)]
	pub fn freeze(&self) {
		self.get_impl().readonly = true;
	}

	#[allow(unused)]
	pub fn try_set<U: Into<V>>(&self, value: U) -> Result<(), String> {
		self.check_is_settable(false)?;
		let mut self_impl = self.get_impl();
		self_impl.value = V::item(value.into());
		self_impl.factory.upgrade().unwrap().borrow_mut().change_set.insert(Box::new(Rc::downgrade(&self.0)));
		Ok(())
	}

	#[allow(unused)]
	pub fn set<U: Into<V>>(&self, value: U) {
		self.try_set(value).unwrap_or_else(|m| panic!("{}", m));
	}

	#[allow(unused)]
	pub fn try_bind<P: Parents, T: Fn(P::Values) -> V + 'static>(&self, parents: P, transform: T) -> Result<(), String> {
		self.check_is_settable(false)?;
		let transform: Rc<dyn ChildTransformTrait> = Rc::new(ChildTransform {
			values: RefCell::new(parents.get_values()),
			transform,
			child: Rc::downgrade(&self.0)
		});
		self.get_impl().transform = Some(transform.clone());
		transform.update_value();
		parents.add_child(Rc::downgrade(&transform));
		Ok(())
	}

	#[allow(unused)]
	pub fn bind<P: Parents, T: Fn(P::Values) -> V + 'static>(&self, parents: P, transform: T) {
		self.try_bind(parents, transform).unwrap_or_else(|m| panic!("{}", m));
	}

	#[allow(unused)]
	pub fn try_unbind(&self) -> Result<(), String> {
		self.check_is_settable(true)?;
		let mut self_impl = self.get_impl();
		self_impl.transform = None;
		self_impl.value = V::item(V::default());
		Ok(())
	}

	#[allow(unused)]
	pub fn unbind(&self) {
		self.try_unbind().unwrap_or_else(|m| panic!("{}", m));
	}

	fn check_is_settable(&self, unbind: bool) -> Result<(), String> {
		let self_impl = self.get_impl();
		if !unbind && self_impl.transform.is_some() {
			Err("cannot rebind child binding without unbinding it first".to_owned())
		} else if self_impl.readonly {
			Err("cannot rebind readonly binding".to_owned())
		} else {
			Ok(())
		}
	}

	fn get_impl(&self) -> RefMut<PropertyImpl<V>> {
		self.0.borrow_mut()
	}

	fn add_child(&self, transform: Weak<dyn ChildTransformTrait>, index: usize) {
		let mut self_impl = self.get_impl();
		self_impl.children.push((transform, index))
	}

	fn new(factory: Weak<RefCell<PropertyFactoryImpl>>, value: V::Item) -> Self {
		factory.upgrade().unwrap().borrow_mut().count += 1;
		Property(Rc::new(RefCell::new(PropertyImpl {
			value,
			readonly: false,
			transform: None,
			children: Vec::new(),
			factory,
		})))
	}
}

impl <V: Value> fmt::Debug for Property<V> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self.get_impl().value)
	}
}

impl <V: Value> Drop for Property<V> {
	fn drop(&mut self) {
			let self_impl = self.get_impl();
			self_impl.factory.upgrade().unwrap().borrow_mut().count -= 1;
			for (transform, index) in &self_impl.children {
				if let Some(transform) = transform.upgrade() {
					transform.parent_value_changed(V::wrapped(V::item(V::default())), *index);
					self_impl.factory.upgrade().unwrap().borrow_mut().change_set.insert(transform.get_child());
				}
			}
	}
}

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
