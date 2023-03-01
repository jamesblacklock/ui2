// MISSING FEATURES FROM ORIGINAL API:
// - transitions
// - optional interpolation on `get()` using `transitionStartTime` and `prevValue`

pub mod value;
mod transform;

use core::fmt;
use std::{rc::{Rc, Weak}, cell::{RefCell, RefMut}, mem, collections::HashSet, hash::Hash, ptr};

use transform::{ChildTransformTrait, ChildTransform, Parents};

use value::{Value};

pub use value::WrappedValue;

use crate::{println, eprintln};

use self::{transform::DynChildTransform, value::{length::Length, brush::Brush}};

#[derive(Debug)]
pub struct PropertyFactory(Rc<RefCell<PropertyFactoryImpl>>);

#[derive(Debug)]
pub struct PropertyFactoryImpl {
	count: usize,
	change_set: HashSet<Box<dyn DynProperty>>,
}

impl PropertyFactory {
	pub fn new_factory() -> Self {
		PropertyFactory(Rc::new(RefCell::new(PropertyFactoryImpl {
			count: 0,
			change_set: HashSet::new(),
		})))
	}

	pub fn new<V: Value>(&self, listener: Option<Box<dyn Listener>>) -> Property<V> {
		let value = V::item(V::default());
		Property::new(Rc::downgrade(&self.0), value, listener)
	}

	pub fn int<I: Into<i32>>(&self, value: I, listener: Option<Box<dyn Listener>>) -> Property<i32> {
		let value = Value::item(value.into());
		Property::new(Rc::downgrade(&self.0), value, listener)
	}

	pub fn string<S: Into<String>>(&self, value: S, listener: Option<Box<dyn Listener>>) -> Property<String> {
		let value = Value::item(value.into());
		Property::new(Rc::downgrade(&self.0), value, listener)
	}

	pub fn boolean<B: Into<bool>>(&self, value: B, listener: Option<Box<dyn Listener>>) -> Property<bool> {
		let value = Value::item(value.into());
		Property::new(Rc::downgrade(&self.0), value, listener)
	}

	pub fn float<F: Into<f64>>(&self, value: F, listener: Option<Box<dyn Listener>>) -> Property<f64> {
		let value = Value::item(value.into());
		Property::new(Rc::downgrade(&self.0), value, listener)
	}

	pub fn length<L: Into<Length>>(&self, value: L, listener: Option<Box<dyn Listener>>) -> Property<Length> {
		let value = Value::item(value.into());
		Property::new(Rc::downgrade(&self.0), value, listener)
	}

	pub fn brush<B: Into<Brush>>(&self, value: B, listener: Option<Box<dyn Listener>>) -> Property<Brush> {
		let value = Value::item(value.into());
		Property::new(Rc::downgrade(&self.0), value, listener)
	}

	pub fn bind<
		V: Value,
		P: Parents,
		T: Fn(P::Values) -> V + 'static
	>(&self, parents: P, transform: T, listener: Option<Box<dyn Listener>>) -> Property<V> {
		let p = Property::new(Rc::downgrade(&self.0), V::item(V::default()), listener);
		p.bind(parents, transform);
		p
	}

	pub fn commit_changes(&self) {
		while self.0.borrow().change_set.len() > 0 {
			let change_set = mem::replace(&mut self.0.borrow_mut().change_set, HashSet::new());
			for prop in change_set {
				prop.commit_changes();
			}
		}
	}

	pub fn count(&self) -> usize {
		self.0.borrow().count
	}
}

impl Drop for PropertyFactory {
	fn drop(&mut self) {}
}

type PropertyCell<V> = RefCell<PropertyImpl<V>>;

pub struct Property<V: Value + 'static>(Rc<PropertyCell<V>>);

pub trait Listener {
	fn notify(&self);
	fn fmt_debug(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "<Listener>")
	}
}

impl fmt::Debug for Box<dyn Listener> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.fmt_debug(f)
	}
}

#[derive(Debug)]
pub struct PropertyImpl<V: Value + 'static> {
	value: V::Item,
	readonly: bool,
	transform: Option<Rc<dyn ChildTransformTrait>>,
	children: Vec<(Weak<dyn ChildTransformTrait>, usize)>,
	factory: Weak<RefCell<PropertyFactoryImpl>>,
	listener: Option<Box<dyn Listener>>,
}

impl <V: Value> PropertyImpl<V> {
	fn set_value(&mut self, value: V::Item) -> bool {
		if self.value != value {
			self.value = value;
			if let Some(listener) = &self.listener {
				listener.notify();
			}
			true
		} else {
			false
		}
	}
}

pub trait DynProperty {
	fn fmt_debug(&self, f: &mut fmt::Formatter) -> fmt::Result;
	fn commit_changes(&self);
	fn clone(&self) -> Box<dyn DynProperty>;
	fn get_wrapped_value(&self) -> WrappedValue;
	fn add_child(&self, transform: Weak<dyn ChildTransformTrait>, index: usize);
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
				}
			}
			cell.borrow_mut().children = new_children;
		}
	}
	fn clone(&self) -> Box<dyn DynProperty> {
		Box::new(Clone::clone(self))
	}
	fn get_wrapped_value(&self) -> WrappedValue {
		if let Some(cell) = self.upgrade() {
			V::wrapped(cell.borrow().value.clone())
		} else {
			V::wrapped(V::item(V::default()))
		}
	}
	fn add_child(&self, transform: Weak<dyn ChildTransformTrait>, index: usize) {
		if let Some(cell) = self.upgrade() {
			cell.borrow_mut().children.push((transform, index))
		}
	}
}

impl <V: Value> Property<V> {
	pub fn get(&self) -> V::Item {
		self.0.borrow().value.clone()
	}

	pub fn freeze(&self) {
		self.get_impl().readonly = true;
	}

	pub fn try_set<U: Into<V>>(&self, value: U) -> Result<(), String> {
		self.check_is_settable(false)?;
		let mut self_impl = self.get_impl();
		if self_impl.set_value(V::item(value.into())) {
			let factory = self_impl.factory.upgrade();
			factory.unwrap().borrow_mut().change_set.insert(self.dynamic());
		}
		Ok(())
	}

	pub fn set<U: Into<V>>(&self, value: U) {
		self.try_set(value).unwrap_or_else(|m| panic!("{}", m));
	}

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

		let factory = self.get_impl().factory.upgrade();
		factory.unwrap().borrow_mut().change_set.insert(self.dynamic());
		Ok(())
	}

	pub fn bind<P: Parents, T: Fn(P::Values) -> V + 'static>(&self, parents: P, transform: T) {
		self.try_bind(parents, transform).unwrap_or_else(|m| panic!("{}", m));
	}

	pub fn try_bind_dynamic<T: Fn(Vec<WrappedValue>) -> V + 'static>(&self, parents: &[Box<dyn DynProperty>], transform: T) -> Result<(), String> {
		self.check_is_settable(false)?;
		let values: Vec<_> = parents.iter().map(|e| e.get_wrapped_value()).collect();
		let transform: Rc<dyn ChildTransformTrait> = Rc::new(DynChildTransform {
			values: RefCell::new(values),
			transform,
			child: Rc::downgrade(&self.0)
		});
		self.get_impl().transform = Some(transform.clone());
		transform.update_value();
		for (index, parent) in parents.into_iter().enumerate() {
			parent.add_child(Rc::downgrade(&transform), index);
		}
		Ok(())
	}

	pub fn try_unbind(&self) -> Result<(), String> {
		self.check_is_settable(true)?;
		let mut self_impl = self.get_impl();
		self_impl.transform = None;
		if self_impl.set_value(V::item(V::default())) {
			let factory = self_impl.factory.upgrade();
			factory.unwrap().borrow_mut().change_set.insert(self.dynamic());
		}
		Ok(())
	}

	pub fn unbind(&self) {
		self.try_unbind().unwrap_or_else(|m| panic!("{}", m));
	}

	pub fn dynamic(&self) -> Box<dyn DynProperty> {
		Box::new(Rc::downgrade(&self.0))
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

	fn new(factory: Weak<RefCell<PropertyFactoryImpl>>, value: V::Item, listener: Option<Box<dyn Listener>>) -> Self {
		factory.upgrade().unwrap().borrow_mut().count += 1;
		Property(Rc::new(RefCell::new(PropertyImpl {
			value,
			readonly: false,
			transform: None,
			children: Vec::new(),
			factory,
			listener,
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

#[test]
fn tests() {
	let factory = PropertyFactory::new_factory();
	let a = factory.int(7, None);
	let b = factory.bind((&a,), |(a,)| a == 7, None);

	assert!(a.get() == 7);
	assert!(b.get() == true);

	let c = factory.string("a", None);
	a.bind((&c,), |(c,)| (c.as_bytes()[0] - 91) as _);

	assert!(a.get() == 6);     // `bind()` sets the value immediately
	assert!(b.get() == true);  // children are not updated yet
	assert!(&*c.get() == "a");

	factory.commit_changes();

	assert!(a.get() == 6);     // nothing changed here
	assert!(b.get() == false); // `b` was added to the `change_set` and updated
	assert!(&*c.get() == "a");

	let d = factory.string("B", None);
	c.bind((&d,), |(d,)| d.to_lowercase());
	factory.commit_changes();

	assert!(a.get() == 7);     // "B".to_lowercase() => "b" - 91 => 7
	assert!(b.get() == true);  // a == 7
	assert!(&*c.get() == "b"); // "B".to_lowercase()
	assert!(&*d.get() == "B");
}
