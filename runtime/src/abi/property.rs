use std::ops::Deref;
use std::rc::Rc;
use std::{mem};
use crate::abi::{Abi, AbiResult, AbiFunction};
use crate::property::value::Value;
use crate::property::{Property, PropertyFactory, DynProperty, WrappedValue, Listener};
use crate::property::value::{length::Length, brush::Brush};
use crate::{println, eprintln};

use super::AbiBuffer;

impl Listener for AbiFunction {
  fn notify(&self) {
    self.dispatch_void(Vec::new() as Vec<()>)
  }
  fn fmt_debug(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "{:?}", self)
  }
}

/*************************************************************************
 * 
 *                           PropertyFactory
 * 
 *************************************************************************/

#[no_mangle] #[allow(non_snake_case)]
pub fn property_factory__new_factory() -> Abi<PropertyFactory> {
  Abi::into_abi(PropertyFactory::new_factory())
}

#[no_mangle] #[allow(non_snake_case)]
pub fn property_factory__new_property__int(factory: Abi<PropertyFactory>, notify: AbiFunction) -> Abi<Property<i32>> {
  let listener = if notify.is_null() { None } else {
    Some(Box::new(notify) as Box<dyn Listener>)
  };
  let factory = factory.into_runtime_temporary();
  Abi::into_abi(factory.int(0, listener))
}

#[no_mangle] #[allow(non_snake_case)]
pub fn property_factory__new_property__string(factory: Abi<PropertyFactory>, notify: AbiFunction) -> Abi<Property<String>> {
  let listener = if notify.is_null() { None } else {
    Some(Box::new(notify) as Box<dyn Listener>)
  };
  let factory = factory.into_runtime_temporary();
  Abi::into_abi(factory.string("", listener))
}

#[no_mangle] #[allow(non_snake_case)]
pub fn property_factory__new_property__float(factory: Abi<PropertyFactory>, notify: AbiFunction) -> Abi<Property<f64>> {
  let listener = if notify.is_null() { None } else {
    Some(Box::new(notify) as Box<dyn Listener>)
  };
  let factory = factory.into_runtime_temporary();
  Abi::into_abi(factory.float(0, listener))
}

#[no_mangle] #[allow(non_snake_case)]
pub fn property_factory__new_property__boolean(factory: Abi<PropertyFactory>, notify: AbiFunction) -> Abi<Property<bool>> {
  let listener = if notify.is_null() { None } else {
    Some(Box::new(notify) as Box<dyn Listener>)
  };
  let factory = factory.into_runtime_temporary();
  Abi::into_abi(factory.boolean(false, listener))
}

#[no_mangle] #[allow(non_snake_case)]
pub fn property_factory__new_property__length(factory: Abi<PropertyFactory>, notify: AbiFunction) -> Abi<Property<Length>> {
  let listener = if notify.is_null() { None } else {
    Some(Box::new(notify) as Box<dyn Listener>)
  };
  let factory = factory.into_runtime_temporary();
  Abi::into_abi(factory.length(Length::default(), listener))
}

#[no_mangle] #[allow(non_snake_case)]
pub fn property_factory__new_property__brush(factory: Abi<PropertyFactory>, notify: AbiFunction) -> Abi<Property<Brush>> {
  let listener = if notify.is_null() { None } else {
    Some(Box::new(notify) as Box<dyn Listener>)
  };
  let factory = factory.into_runtime_temporary();
  Abi::into_abi(factory.brush(Brush::default(), listener))
}

#[no_mangle] #[allow(non_snake_case)]
pub fn property_factory__commit_changes(factory: Abi<PropertyFactory>) {
  factory.into_runtime_temporary().commit_changes()
}

#[no_mangle] #[allow(non_snake_case)]
pub fn property_factory__count(factory: Abi<PropertyFactory>) -> usize {
  factory.into_runtime_temporary().count()
}

#[no_mangle] #[allow(non_snake_case)]
pub fn property_factory__drop(factory: Abi<PropertyFactory>) {
  mem::drop(factory.into_runtime());
}

/*************************************************************************
 * 
 *                            Property<bool>
 * 
 *************************************************************************/
#[no_mangle] #[allow(non_snake_case)]
pub fn property__boolean__get(property: Abi<Property<bool>>) -> bool {
  property.into_runtime_temporary().get()
}

#[no_mangle] #[allow(non_snake_case)]
pub fn property__boolean__freeze(property: Abi<Property<bool>>) {
  property.into_runtime_temporary().freeze();
}

#[no_mangle] #[allow(non_snake_case)]
pub fn property__boolean__set(property: Abi<Property<bool>>, value: bool, result: Abi<AbiResult>) {
  if let Err(err) = property.into_runtime_temporary().try_set(value) {
    result.into_runtime_temporary().err(err);
  }
}

#[no_mangle] #[allow(non_snake_case)]
pub fn property__boolean__weakref(property: Abi<Property<bool>>) -> Abi<Box<dyn DynProperty>> {
  Abi::into_abi(property.into_runtime_temporary().dynamic())
}

#[no_mangle] #[allow(non_snake_case)]
pub fn property__boolean__bind(property: Abi<Property<bool>>, parents: Abi<Vec<Box<dyn DynProperty>>>, callback: AbiFunction, result: Abi<AbiResult>) {
  let callback = move |q| {
    let value: Box<WrappedValue> = callback.dispatch_box(q);
    value.unwrap_boolean()
  };
  
  let property = property.into_runtime_temporary();

  if let Err(err) = property.try_bind_dynamic(&parents.into_runtime_temporary(), callback) {
    result.into_runtime_temporary().err(err);
  }
}

#[no_mangle] #[allow(non_snake_case)]
pub fn property__boolean__unbind(property: Abi<Property<bool>>, result: Abi<AbiResult>) {
  if let Err(err) = property.into_runtime_temporary().try_unbind() {
    result.into_runtime_temporary().err(err);
  }
}

#[no_mangle] #[allow(non_snake_case)]
pub fn property__boolean__drop(property: Abi<Property<bool>>) {
  mem::drop(property.into_runtime());
}

/*************************************************************************
 * 
 *                            Property<i32>
 * 
 *************************************************************************/
#[no_mangle] #[allow(non_snake_case)]
pub fn property__int__get(property: Abi<Property<i32>>) -> i32 {
  property.into_runtime_temporary().get()
}

#[no_mangle] #[allow(non_snake_case)]
pub fn property__int__freeze(property: Abi<Property<i32>>) {
  property.into_runtime_temporary().freeze();
}

#[no_mangle] #[allow(non_snake_case)]
pub fn property__int__set(property: Abi<Property<i32>>, value: i32, result: Abi<AbiResult>) {
  if let Err(err) = property.into_runtime_temporary().try_set(value) {
    result.into_runtime_temporary().err(err);
  }
}

#[no_mangle] #[allow(non_snake_case)]
pub fn property__int__weakref(property: Abi<Property<i32>>) -> Abi<Box<dyn DynProperty>> {
  Abi::into_abi(property.into_runtime_temporary().dynamic())
}

#[no_mangle] #[allow(non_snake_case)]
pub fn property__int__bind(property: Abi<Property<i32>>, parents: Abi<Vec<Box<dyn DynProperty>>>, callback: AbiFunction, result: Abi<AbiResult>) {
  let callback = move |q| {
    let value: Box<WrappedValue> = callback.dispatch_box(q);
    value.unwrap_int()
  };
  
  let property = property.into_runtime_temporary();

  if let Err(err) = property.try_bind_dynamic(&parents.into_runtime_temporary(), callback) {
    result.into_runtime_temporary().err(err);
  }
}

#[no_mangle] #[allow(non_snake_case)]
pub fn property__int__unbind(property: Abi<Property<i32>>, result: Abi<AbiResult>) {
  if let Err(err) = property.into_runtime_temporary().try_unbind() {
    result.into_runtime_temporary().err(err);
  }
}

#[no_mangle] #[allow(non_snake_case)]
pub fn property__int__drop(property: Abi<Property<i32>>) {
  mem::drop(property.into_runtime());
}

/*************************************************************************
 * 
 *                            Property<f64>
 * 
 *************************************************************************/
#[no_mangle] #[allow(non_snake_case)]
pub fn property__float__get(property: Abi<Property<f64>>) -> f64 {
  property.into_runtime_temporary().get()
}

#[no_mangle] #[allow(non_snake_case)]
pub fn property__float__freeze(property: Abi<Property<f64>>) {
  property.into_runtime_temporary().freeze();
}

#[no_mangle] #[allow(non_snake_case)]
pub fn property__float__set(property: Abi<Property<f64>>, value: f64, result: Abi<AbiResult>) {
  if let Err(err) = property.into_runtime_temporary().try_set(value) {
    result.into_runtime_temporary().err(err);
  }
}

#[no_mangle] #[allow(non_snake_case)]
pub fn property__float__weakref(property: Abi<Property<f64>>) -> Abi<Box<dyn DynProperty>> {
  Abi::into_abi(property.into_runtime_temporary().dynamic())
}

#[no_mangle] #[allow(non_snake_case)]
pub fn property__float__bind(property: Abi<Property<f64>>, parents: Abi<Vec<Box<dyn DynProperty>>>, callback: AbiFunction, result: Abi<AbiResult>) {
  let callback = move |q| {
    let value: Box<WrappedValue> = callback.dispatch_box(q);
    value.unwrap_float()
  };
  
  let property = property.into_runtime_temporary();

  if let Err(err) = property.try_bind_dynamic(&parents.into_runtime_temporary(), callback) {
    result.into_runtime_temporary().err(err);
  }
}

#[no_mangle] #[allow(non_snake_case)]
pub fn property__float__unbind(property: Abi<Property<f64>>, result: Abi<AbiResult>) {
  if let Err(err) = property.into_runtime_temporary().try_unbind() {
    result.into_runtime_temporary().err(err);
  }
}

#[no_mangle] #[allow(non_snake_case)]
pub fn property__float__drop(property: Abi<Property<f64>>) {
  mem::drop(property.into_runtime());
}

/*************************************************************************
 * 
 *                            Property<String>
 * 
 *************************************************************************/
#[no_mangle] #[allow(non_snake_case)]
pub fn property__string__get(property: Abi<Property<String>>) -> Abi<AbiBuffer> {
  Abi::into_abi(AbiBuffer::from_string(property.into_runtime_temporary().get().deref().clone()))
}

#[no_mangle] #[allow(non_snake_case)]
pub fn property__string__freeze(property: Abi<Property<String>>) {
  property.into_runtime_temporary().freeze();
}

#[no_mangle] #[allow(non_snake_case)]
pub fn property__string__set(property: Abi<Property<String>>, value: Abi<AbiBuffer>, result: Abi<AbiResult>) {
  let value = value.into_runtime_temporary().to_string();
  if let Err(err) = property.into_runtime_temporary().try_set(value) {
    result.into_runtime_temporary().err(err);
  }
}

#[no_mangle] #[allow(non_snake_case)]
pub fn property__string__weakref(property: Abi<Property<String>>) -> Abi<Box<dyn DynProperty>> {
  Abi::into_abi(property.into_runtime_temporary().dynamic())
}

#[no_mangle] #[allow(non_snake_case)]
pub fn property__string__bind(property: Abi<Property<String>>, parents: Abi<Vec<Box<dyn DynProperty>>>, callback: AbiFunction, result: Abi<AbiResult>) {
  let callback = move |q| {
    let value: Box<WrappedValue> = callback.dispatch_box(q);
    value.unwrap_string().deref().clone()
  };
  
  let property = property.into_runtime_temporary();

  if let Err(err) = property.try_bind_dynamic(&parents.into_runtime_temporary(), callback) {
    result.into_runtime_temporary().err(err);
  }
}

#[no_mangle] #[allow(non_snake_case)]
pub fn property__string__unbind(property: Abi<Property<String>>, result: Abi<AbiResult>) {
  if let Err(err) = property.into_runtime_temporary().try_unbind() {
    result.into_runtime_temporary().err(err);
  }
}

#[no_mangle] #[allow(non_snake_case)]
pub fn property__string__drop(property: Abi<Property<String>>) {
  mem::drop(property.into_runtime());
}

/*************************************************************************
 * 
 *                            Property<Length>
 * 
 *************************************************************************/

#[no_mangle] #[allow(non_snake_case)]
pub fn property__length__get(property: Abi<Property<Length>>) -> Abi<Length> {
  Abi::into_abi(property.into_runtime_temporary().get())
}

#[no_mangle] #[allow(non_snake_case)]
pub fn property__length__freeze(property: Abi<Property<Length>>) {
  property.into_runtime_temporary().freeze();
}

#[no_mangle] #[allow(non_snake_case)]
pub fn property__length__set(property: Abi<Property<Length>>, value: Abi<Length>, result: Abi<AbiResult>) {
  let value = value.into_runtime_temporary();
  if let Err(err) = property.into_runtime_temporary().try_set(value.clone()) {
    result.into_runtime_temporary().err(err);
  }
}

#[no_mangle] #[allow(non_snake_case)]
pub fn property__length__weakref(property: Abi<Property<Length>>) -> Abi<Box<dyn DynProperty>> {
  Abi::into_abi(property.into_runtime_temporary().dynamic())
}

#[no_mangle] #[allow(non_snake_case)]
pub fn property__length__bind(property: Abi<Property<Length>>, parents: Abi<Vec<Box<dyn DynProperty>>>, callback: AbiFunction, result: Abi<AbiResult>) {
  let callback = move |q| {
    let value: Box<WrappedValue> = callback.dispatch_box(q);
    value.unwrap_length()
  };
  
  let property = property.into_runtime_temporary();

  if let Err(err) = property.try_bind_dynamic(&parents.into_runtime_temporary(), callback) {
    result.into_runtime_temporary().err(err);
  }
}

#[no_mangle] #[allow(non_snake_case)]
pub fn property__length__unbind(property: Abi<Property<Length>>, result: Abi<AbiResult>) {
  if let Err(err) = property.into_runtime_temporary().try_unbind() {
    result.into_runtime_temporary().err(err);
  }
}

#[no_mangle] #[allow(non_snake_case)]
pub fn property__length__drop(property: Abi<Property<Length>>) {
  mem::drop(property.into_runtime());
}

/*************************************************************************
 * 
 *                            Property<Brush>
 * 
 *************************************************************************/

#[no_mangle] #[allow(non_snake_case)]
pub fn property__brush__get(property: Abi<Property<Brush>>) -> Abi<Brush> {
  Abi::into_abi(property.into_runtime_temporary().get())
}

#[no_mangle] #[allow(non_snake_case)]
pub fn property__brush__freeze(property: Abi<Property<Brush>>) {
  property.into_runtime_temporary().freeze();
}

#[no_mangle] #[allow(non_snake_case)]
pub fn property__brush__set(property: Abi<Property<Brush>>, value: Abi<Brush>, result: Abi<AbiResult>) {
  let value = value.into_runtime_temporary();
  if let Err(err) = property.into_runtime_temporary().try_set(value.clone()) {
    result.into_runtime_temporary().err(err);
  }
}

#[no_mangle] #[allow(non_snake_case)]
pub fn property__brush__weakref(property: Abi<Property<Brush>>) -> Abi<Box<dyn DynProperty>> {
  Abi::into_abi(property.into_runtime_temporary().dynamic())
}

#[no_mangle] #[allow(non_snake_case)]
pub fn property__brush__bind(property: Abi<Property<Brush>>, parents: Abi<Vec<Box<dyn DynProperty>>>, callback: AbiFunction, result: Abi<AbiResult>) {
  let callback = move |q| {
    let value: Box<WrappedValue> = callback.dispatch_box(q);
    value.unwrap_brush()
  };
  
  let property = property.into_runtime_temporary();

  if let Err(err) = property.try_bind_dynamic(&parents.into_runtime_temporary(), callback) {
    result.into_runtime_temporary().err(err);
  }
}

#[no_mangle] #[allow(non_snake_case)]
pub fn property__brush__unbind(property: Abi<Property<Brush>>, result: Abi<AbiResult>) {
  if let Err(err) = property.into_runtime_temporary().try_unbind() {
    result.into_runtime_temporary().err(err);
  }
}

#[no_mangle] #[allow(non_snake_case)]
pub fn property__brush__drop(property: Abi<Property<Brush>>) {
  mem::drop(property.into_runtime());
}

/*************************************************************************
 * 
 *                  Box<dyn DynProperty> (PropertyWeakRef)
 * 
 *************************************************************************/

#[no_mangle] #[allow(non_snake_case)]
pub fn property__weakref__drop(property: Abi<Box<dyn DynProperty>>) {
  mem::drop(property.into_runtime());
}

/*************************************************************************
 * 
 *                 Vec<Box<dyn DynProperty>> (PropertyVec)
 * 
 *************************************************************************/

#[no_mangle] #[allow(non_snake_case)]
pub fn vec__weakref_property__new() -> Abi<Vec<Box<dyn DynProperty>>> {
  Abi::into_abi(Vec::new())
}

#[no_mangle] #[allow(non_snake_case)]
pub fn vec__weakref_property__push(vec: Abi<Vec<Box<dyn DynProperty>>>, property: Abi<Box<dyn DynProperty>>) {
  let vec = vec.into_runtime_temporary();
  vec.push(property.into_runtime_temporary().clone());
}

#[no_mangle] #[allow(non_snake_case)]
pub fn vec__weakref_property__len(vec: Abi<Vec<Box<dyn DynProperty>>>) -> usize {
  vec.into_runtime_temporary().len()
}

#[no_mangle] #[allow(non_snake_case)]
pub fn vec__weakref_property__get(vec: Abi<Vec<Box<dyn DynProperty>>>, index: usize) -> Abi<Box<dyn DynProperty>> {
  Abi::into_abi(vec.into_runtime_temporary()[index].clone())
}

#[no_mangle] #[allow(non_snake_case)]
pub fn vec__weakref_property__drop(vec: Abi<Vec<Box<dyn DynProperty>>>) {
  mem::drop(vec.into_runtime())
}

/*************************************************************************
 * 
 *                 Vec<WrappedValue> (ValueVec)
 * 
 *************************************************************************/

#[no_mangle] #[allow(non_snake_case)]
pub fn vec__wrapped_value__new() -> Abi<Vec<WrappedValue>> {
  Abi::into_abi(Vec::new())
}

#[no_mangle] #[allow(non_snake_case)]
pub fn vec__wrapped_value__push(vec: Abi<Vec<WrappedValue>>, property: Abi<WrappedValue>) {
  let vec = vec.into_runtime_temporary();
  vec.push(property.into_runtime_temporary().clone());
}

#[no_mangle] #[allow(non_snake_case)]
pub fn vec__wrapped_value__len(vec: Abi<Vec<WrappedValue>>) -> usize {
  vec.into_runtime_temporary().len()
}

#[no_mangle] #[allow(non_snake_case)]
pub fn vec__wrapped_value__get(vec: Abi<Vec<WrappedValue>>, index: usize) -> Abi<WrappedValue> {
  Abi::into_abi(vec.into_runtime_temporary()[index].clone())
}

#[no_mangle] #[allow(non_snake_case)]
pub fn vec__wrapped_value__drop(vec: Abi<Vec<WrappedValue>>) {
  mem::drop(vec.into_runtime())
}

/*************************************************************************
 * 
 *                               Length
 * 
 *************************************************************************/

#[no_mangle] #[allow(non_snake_case)]
pub fn length__px(value: f64) -> Abi<Length> {
  Abi::into_abi(Length::Px(value))
}
 
#[no_mangle] #[allow(non_snake_case)]
pub fn length__cm(value: f64) -> Abi<Length> {
  Abi::into_abi(Length::Cm(value))
}
 
#[no_mangle] #[allow(non_snake_case)]
pub fn length__in(value: f64) -> Abi<Length> {
  Abi::into_abi(Length::In(value))
}
 
#[no_mangle] #[allow(non_snake_case)]
pub fn length__html_vw(value: f64) -> Abi<Length> {
  Abi::into_abi(Length::__HtmlVw(value))
}
 
#[no_mangle] #[allow(non_snake_case)]
pub fn length__html_vh(value: f64) -> Abi<Length> {
  Abi::into_abi(Length::__HtmlVh(value))
}
 
#[no_mangle] #[allow(non_snake_case)]
pub fn length__add(lhs: Abi<Length>, rhs: Abi<Length>) -> Abi<Length> {
  let lhs = lhs.into_runtime_temporary();
  let rhs = rhs.into_runtime_temporary();
  Abi::into_abi(lhs.clone() + rhs.clone())
}
 
#[no_mangle] #[allow(non_snake_case)]
pub fn length__sub(lhs: Abi<Length>, rhs: Abi<Length>) -> Abi<Length> {
  let lhs = lhs.into_runtime_temporary();
  let rhs = rhs.into_runtime_temporary();
  Abi::into_abi(lhs.clone() - rhs.clone())
}
 
#[no_mangle] #[allow(non_snake_case)]
pub fn length__mul(lhs: Abi<Length>, rhs: f64) -> Abi<Length> {
  let lhs = lhs.into_runtime_temporary();
  Abi::into_abi(lhs.clone() * rhs)
}
 
#[no_mangle] #[allow(non_snake_case)]
pub fn length__div(lhs: Abi<Length>, rhs: f64) -> Abi<Length> {
  let lhs = lhs.into_runtime_temporary();
  Abi::into_abi(lhs.clone() / rhs)
}

#[no_mangle] #[allow(non_snake_case)]
pub fn length__neg(value: Abi<Length>) -> Abi<Length> {
  let value = value.into_runtime_temporary();
  Abi::into_abi(-value.clone())
}
 
#[no_mangle] #[allow(non_snake_case)]
pub fn length__to_string(length: Abi<Length>) -> Abi<AbiBuffer> {
  Abi::into_abi(AbiBuffer::from_string(format!("{}", length.into_runtime_temporary())))
}
 
#[no_mangle] #[allow(non_snake_case)]
pub fn length__drop(length: Abi<Length>) {
  mem::drop(length.into_runtime())
}

/*************************************************************************
 * 
 *                                  Brush
 * 
 *************************************************************************/

#[no_mangle] #[allow(non_snake_case)]
pub fn brush__rgba(r: f64, g: f64, b: f64, a: f64) -> Abi<Brush> {
  Abi::into_abi(Brush::Color(r, g, b, a))
}
 
#[no_mangle] #[allow(non_snake_case)]
pub fn brush__to_string(brush: Abi<Brush>) -> Abi<AbiBuffer> {
  Abi::into_abi(AbiBuffer::from_string(format!("{}", brush.into_runtime_temporary())))
}
 
#[no_mangle] #[allow(non_snake_case)]
pub fn brush__drop(brush: Abi<Brush>) {
  mem::drop(brush.into_runtime())
}

/*************************************************************************
 * 
 *                             WrappedValue
 * 
 *************************************************************************/

#[no_mangle] #[allow(non_snake_case)]
fn wrapped_value__wrap_boolean(value: bool) -> Abi<WrappedValue> {
  Abi::into_abi(WrappedValue::Boolean(value))
}

#[no_mangle] #[allow(non_snake_case)]
fn wrapped_value__unwrap_boolean(value: Abi<WrappedValue>) -> bool {
  value.into_runtime_temporary().unwrap_boolean()
}
#[no_mangle] #[allow(non_snake_case)]
fn wrapped_value__wrap_float(value: f64) -> Abi<WrappedValue> {
  Abi::into_abi(WrappedValue::Float(value))
}

#[no_mangle] #[allow(non_snake_case)]
fn wrapped_value__unwrap_float(value: Abi<WrappedValue>) -> f64 {
  value.into_runtime_temporary().unwrap_float()
}
#[no_mangle] #[allow(non_snake_case)]
fn wrapped_value__wrap_int(value: i32) -> Abi<WrappedValue> {
  Abi::into_abi(WrappedValue::Int(value))
}

#[no_mangle] #[allow(non_snake_case)]
fn wrapped_value__unwrap_int(value: Abi<WrappedValue>) -> i32 {
  value.into_runtime_temporary().unwrap_int()
}

#[no_mangle] #[allow(non_snake_case)]
fn wrapped_value__wrap_string(value: Abi<AbiBuffer>) -> Abi<WrappedValue> {
  let string = value.into_runtime_temporary().to_string();
  Abi::into_abi(WrappedValue::String(Rc::new(string)))
}

#[no_mangle] #[allow(non_snake_case)]
fn wrapped_value__unwrap_string(value: Abi<WrappedValue>) -> Abi<AbiBuffer> {
  Abi::into_abi(AbiBuffer::from_string(value.into_runtime_temporary().unwrap_string().deref().clone()))
}

#[no_mangle] #[allow(non_snake_case)]
fn wrapped_value__wrap_length(value: Abi<Length>) -> Abi<WrappedValue> {
  let length = value.into_runtime_temporary().clone();
  Abi::into_abi(WrappedValue::Length(length))
}

#[no_mangle] #[allow(non_snake_case)]
fn wrapped_value__unwrap_length(value: Abi<WrappedValue>) -> Abi<Length> {
  Abi::into_abi(value.into_runtime_temporary().unwrap_length())
}

#[no_mangle] #[allow(non_snake_case)]
fn wrapped_value__wrap_brush(value: Abi<Brush>) -> Abi<WrappedValue> {
  let brush = value.into_runtime_temporary().clone();
  Abi::into_abi(WrappedValue::Brush(brush))
}

#[no_mangle] #[allow(non_snake_case)]
fn wrapped_value__unwrap_brush(value: Abi<WrappedValue>) -> Abi<Brush> {
  Abi::into_abi(value.into_runtime_temporary().unwrap_brush())
}

#[no_mangle] #[allow(non_snake_case)]
fn wrapped_value__tag(value: Abi<WrappedValue>) -> u32 {
  const BOOLEAN: u32 = 0;
  const INT: u32 = 1;
  const FLOAT: u32 = 2;
  const STRING: u32 = 3;
  const LENGTH: u32 = 4;
  const BRUSH: u32 = 5;

  match value.into_runtime_temporary() {
    WrappedValue::Int(_) => INT,
    WrappedValue::String(_) => STRING,
    WrappedValue::Boolean(_) => BOOLEAN,
    WrappedValue::Float(_) => FLOAT,
    WrappedValue::Length(_) => LENGTH,
    WrappedValue::Brush(_) => BRUSH,
  }
}

#[no_mangle] #[allow(non_snake_case)]
pub fn wrapped_value__drop(value: Abi<WrappedValue>) {
  mem::drop(value.into_runtime())
}
