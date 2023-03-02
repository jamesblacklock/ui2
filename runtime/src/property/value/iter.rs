use std::{marker::PhantomData, fmt, rc::Rc, iter};

use super::{Value, ValueItem, WrappedValue};


// typed iterable (iterates over some `Value`, is *itself* a `Value`)
pub struct Iter<V: Value>(WrappedIter, PhantomData<V>);

impl <V: Value> Iter<V> {
  pub fn empty() -> Self {
    Iter(WrappedIter(Box::new(EmptyIterator)), PhantomData)
  }
  pub fn from<I: Iterable<V>>(value: I) -> Self {
    value.iter()
  }
  pub fn next_wrapped(&mut self) -> Option<WrappedValue> {
    self.0.next()
  }
}

impl <V: Value> Iterator for Iter<V> {
  type Item = V;
  fn next(&mut self) -> Option<Self::Item> {
    self.0.next().map(|e| V::from_item(V::Item::unwrapped(e)))
  }
}

impl <V: Value> fmt::Debug for Iter<V> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    fmt::Debug::fmt(&self.0, f)
  }
}

impl <V: Value> PartialEq for Iter<V> {
  fn eq(&self, rhs: &Self) -> bool {
    self.0 == rhs.0
  }
}

impl <V: Value> Clone for Iter<V> {
  fn clone(&self) -> Self {
    Self(self.0.clone(), PhantomData)
  }
}

impl <V: Value> Value for Iter<V> {
  type Item = Self;
  fn default() -> Self {
    Iter::empty()
  }
  fn item(value: Self) -> Self::Item {
    value
  }
  fn from_item(value: Self::Item) -> Self {
    value
  }
  fn wrapped(value: Self::Item) -> WrappedValue {
    WrappedValue::Iter(value.0)
  }
}
impl <V: Value> ValueItem for Iter<V> {
  fn unwrapped(value: WrappedValue) -> Self {
    value.unwrap_iter()
  }
}


// converts stuff to Iter
pub trait Iterable<V: Value> {
  fn iter(self) -> Iter<V>;
}



// the "guts" of Iter. iterates over wrapped values. all iterables must implement.
trait DynIterator {
  fn clone_iterator(&self) -> Box<dyn DynIterator>;
  fn next(&mut self) -> Option<WrappedValue>;
  fn fmt_debug(&self, f: &mut fmt::Formatter) -> fmt::Result;
}


// a DynIterator that is always empty
#[derive(Debug, Clone, PartialEq)]
struct EmptyIterator;

impl DynIterator for EmptyIterator {
  fn next(&mut self) -> Option<WrappedValue> {
    None
  }
  fn clone_iterator(&self) -> Box<dyn DynIterator> {
    Box::new(EmptyIterator)
  }
  fn fmt_debug(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "<empty>")
  }
}


// simple public wrapper around DynIterator; implements std::iter::Iterator
pub struct WrappedIter(Box<dyn DynIterator>);

impl fmt::Debug for WrappedIter {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Iter(")?;
    self.0.fmt_debug(f)?;
    write!(f, ")")
  }
}

impl PartialEq for WrappedIter {
  fn eq(&self, rhs: &Self) -> bool {
    let left: *const _ = &*self.0;
    let right: *const _ = &*rhs.0;
    left == right
  }
}

impl Clone for WrappedIter {
  fn clone(&self) -> Self {
    WrappedIter(self.0.clone_iterator())
  }
}

impl Iterator for WrappedIter {
  type Item = WrappedValue;
  fn next(&mut self) -> Option<Self::Item> {
    DynIterator::next(&mut *self.0)
  }
}

impl <V: Value> Iterable<V> for WrappedIter {
  fn iter(self) -> Iter<V> {
    Iter(self, PhantomData)
  }
}


// a DynIterator for Int
#[derive(Debug, Clone, PartialEq)]
struct IntIterator(i32, i32);

impl DynIterator for IntIterator {
  fn next(&mut self) -> Option<WrappedValue> {
    if self.1 <= self.0 {
      let value = Some(WrappedValue::Int(self.1));
      self.1 += 1;
      value
    } else {
      None
    }
  }
  fn clone_iterator(&self) -> Box<dyn DynIterator> {
    Box::new(Clone::clone(self))
  }
  fn fmt_debug(&self, f: &mut fmt::Formatter) -> fmt::Result {
    if self.1 < self.0 {
      write!(f, "[{},{}]", self.1, self.0)
    } else if self.1 == self.0 {
      write!(f, "{{{}}}", self.1)
    } else {
      write!(f, "<empty>")
    }
  }
}

impl Iterable<i32> for i32 {
  fn iter(self) -> Iter<i32> {
    Iter(WrappedIter(Box::new(IntIterator(self, 1))), PhantomData)
  }
}


// a DynIterator for Vec/slice
pub struct VecIterator<V: Value>(Rc<Vec<V::Item>>, usize, PhantomData<V>);

impl <V: Value + 'static> DynIterator for VecIterator<V> {
  fn clone_iterator(&self) -> Box<dyn DynIterator> {
    Box::new(VecIterator::<V>(self.0.clone(), self.1, PhantomData))
  }
  fn fmt_debug(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", &self.0[self.1..])
  }
  fn next(&mut self) -> Option<WrappedValue> {
    if self.1 < self.0.len() {
      let value = Clone::clone(&self.0[self.1]);
      self.1 += 1;
      Some(V::wrapped(value))
    } else {
      None
    }
  }
}

impl <V: Value + 'static> Iterable<V> for Vec<V::Item> {
  fn iter(self) -> Iter<V> {
    Iter(WrappedIter(Box::new(VecIterator::<V>(Rc::new(self), 0, PhantomData))), PhantomData)
  }
}

impl <V: Value + Clone + 'static> Iterable<V> for &[V] {
  fn iter(self) -> Iter<V> {
    Iterable::iter(self.iter().map(|e| V::item(e.clone())).collect::<Vec<_>>())
  }
}

impl <const N: usize, V: Value + Clone + 'static> Iterable<V> for &[V; N] {
  fn iter(self) -> Iter<V> {
    Iterable::iter(&self[..])
  }
}
