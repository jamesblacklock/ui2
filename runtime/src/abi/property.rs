use std::ops::Deref;
use std::rc::Rc;
use std::{mem};
use crate::abi::{Abi, AbiResult, AbiFunction};
use crate::property::{Property, PropertyFactory, DynProperty, WrappedValue};
use crate::{println, eprintln};

use super::AbiBuffer;

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
pub fn property_factory__new_property__int(factory: Abi<PropertyFactory>) -> Abi<Property<i64>> {
  let factory = factory.into_runtime_temporary();
  Abi::into_abi(factory.int(0))
}

#[no_mangle] #[allow(non_snake_case)]
pub fn property_factory__new_property__string(factory: Abi<PropertyFactory>) -> Abi<Property<String>> {
  let factory = factory.into_runtime_temporary();
  Abi::into_abi(factory.string(""))
}

#[no_mangle] #[allow(non_snake_case)]
pub fn property_factory__new_property__float(factory: Abi<PropertyFactory>) -> Abi<Property<f64>> {
  let factory = factory.into_runtime_temporary();
  Abi::into_abi(factory.float(0))
}

#[no_mangle] #[allow(non_snake_case)]
pub fn property_factory__new_property__boolean(factory: Abi<PropertyFactory>) -> Abi<Property<bool>> {
  let factory = factory.into_runtime_temporary();
  Abi::into_abi(factory.boolean(false))
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
    let value: Box<WrappedValue> = callback.dispatch(q);
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
 *                            Property<i64>
 * 
 *************************************************************************/
#[no_mangle] #[allow(non_snake_case)]
pub fn property__int__get(property: Abi<Property<i64>>) -> i64 {
  property.into_runtime_temporary().get()
}

#[no_mangle] #[allow(non_snake_case)]
pub fn property__int__freeze(property: Abi<Property<i64>>) {
  property.into_runtime_temporary().freeze();
}

#[no_mangle] #[allow(non_snake_case)]
pub fn property__int__set(property: Abi<Property<i64>>, value: i64, result: Abi<AbiResult>) {
  if let Err(err) = property.into_runtime_temporary().try_set(value) {
    result.into_runtime_temporary().err(err);
  }
}

#[no_mangle] #[allow(non_snake_case)]
pub fn property__int__weakref(property: Abi<Property<i64>>) -> Abi<Box<dyn DynProperty>> {
  Abi::into_abi(property.into_runtime_temporary().dynamic())
}

#[no_mangle] #[allow(non_snake_case)]
pub fn property__int__bind(property: Abi<Property<i64>>, parents: Abi<Vec<Box<dyn DynProperty>>>, callback: AbiFunction, result: Abi<AbiResult>) {
  let callback = move |q| {
    let value: Box<WrappedValue> = callback.dispatch(q);
    value.unwrap_int()
  };
  
  let property = property.into_runtime_temporary();

  if let Err(err) = property.try_bind_dynamic(&parents.into_runtime_temporary(), callback) {
    result.into_runtime_temporary().err(err);
  }
}

#[no_mangle] #[allow(non_snake_case)]
pub fn property__int__unbind(property: Abi<Property<i64>>, result: Abi<AbiResult>) {
  if let Err(err) = property.into_runtime_temporary().try_unbind() {
    result.into_runtime_temporary().err(err);
  }
}

#[no_mangle] #[allow(non_snake_case)]
pub fn property__int__drop(property: Abi<Property<i64>>) {
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
    let value: Box<WrappedValue> = callback.dispatch(q);
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
    let value: Box<WrappedValue> = callback.dispatch(q);
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
fn wrapped_value__wrap_int(value: i64) -> Abi<WrappedValue> {
  Abi::into_abi(WrappedValue::Int(value))
}

#[no_mangle] #[allow(non_snake_case)]
fn wrapped_value__unwrap_int(value: Abi<WrappedValue>) -> i64 {
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
fn wrapped_value__tag(value: Abi<WrappedValue>) -> u32 {
  const BOOLEAN: u32 = 0;
  const INT: u32 = 1;
  const FLOAT: u32 = 2;
  const STRING: u32 = 1;

  match value.into_runtime_temporary() {
    WrappedValue::Int(_) => INT,
    WrappedValue::String(_) => STRING,
    WrappedValue::Boolean(_) => BOOLEAN,
    WrappedValue::Float(_) => FLOAT,
  }
}

#[no_mangle] #[allow(non_snake_case)]
pub fn wrapped_value__drop(value: Abi<WrappedValue>) {
  mem::drop(value.into_runtime())
}
