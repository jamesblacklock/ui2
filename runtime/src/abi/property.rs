use std::ops::Deref;
use std::rc::Rc;
use std::{mem, fmt};
use crate::abi::{Abi, AbiResult, AbiFunction, IntoRuntime, IntoAbi};
use crate::property::{
  Property,
  PropertyFactory,
  DynProperty,
  WrappedValue,
  Listener,
  value::{
    Value,
    ValueItem,
    length::Length,
    brush::Brush,
    layout::Layout,
    iter::{Iter, Iterable},
  },
};
use crate::{println, eprintln};

use super::AbiBuffer;

impl Listener for AbiFunction {
  fn notify(&self) {
    self.dispatch_void(Vec::new() as Vec<()>)
  }
  fn fmt_debug(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

macro_rules! define_drop {
  ($t:ty, $drop:ident) => {
    #[no_mangle] #[allow(non_snake_case)]
    pub fn $drop(value: Abi<$t>) {
      mem::drop(value.into_runtime());
    }
  }
}

macro_rules! define_property_type {
  (
    $prop_type:ty,
    $abi_type:ty,
    $property_factory_new:ident,
    $property_get:ident,
    $property_set:ident,
    $property_freeze:ident,
    $property_weakref:ident,
    $property_bind:ident,
    $property_unbind:ident,
    $property_drop:ident,
    $value_wrap:ident,
    $value_unwrap:ident,
  ) => {
    #[no_mangle] #[allow(non_snake_case)]
    pub fn $property_factory_new(factory: Abi<PropertyFactory>, notify: AbiFunction) -> Abi<Property<$prop_type>> {
      let listener = if notify.is_null() { None } else {
        Some(Box::new(notify) as Box<dyn Listener>)
      };
      let factory = factory.into_runtime_temporary();
      Abi::into_abi(factory.new(<$prop_type as Value>::default(), listener))
    }
    #[no_mangle] #[allow(non_snake_case)]
    pub fn $property_get(property: Abi<Property<$prop_type>>) -> $abi_type {
      IntoAbi::into_abi(property.into_runtime_temporary().get())
    }
    #[no_mangle] #[allow(non_snake_case)]
    pub fn $property_set(property: Abi<Property<$prop_type>>, value: $abi_type, result: Abi<AbiResult>) {
      let value = IntoRuntime::into_runtime(value);
      if let Err(err) = property.into_runtime_temporary().try_set(value.clone()) {
        result.into_runtime_temporary().err(err);
      }
    }
    #[no_mangle] #[allow(non_snake_case)]
    pub fn $property_freeze(property: Abi<Property<$prop_type>>) {
      property.into_runtime_temporary().freeze();
    }
    #[no_mangle] #[allow(non_snake_case)]
    pub fn $property_weakref(property: Abi<Property<$prop_type>>) -> Abi<Box<dyn DynProperty>> {
      Abi::into_abi(property.into_runtime_temporary().dynamic())
    }
    #[no_mangle] #[allow(non_snake_case)]
    pub fn $property_bind(property: Abi<Property<$prop_type>>, parents: Abi<Vec<Box<dyn DynProperty>>>, callback: AbiFunction, result: Abi<AbiResult>) {
      let callback = move |q| Value::from_item(ValueItem::unwrapped(callback.dispatch_box(q)));
      let property = property.into_runtime_temporary();
    
      if let Err(err) = property.try_bind_dynamic(&parents.into_runtime_temporary(), callback) {
        result.into_runtime_temporary().err(err);
      }
    }
    #[no_mangle] #[allow(non_snake_case)]
    pub fn $property_unbind(property: Abi<Property<$prop_type>>, result: Abi<AbiResult>) {
      if let Err(err) = property.into_runtime_temporary().try_unbind() {
        result.into_runtime_temporary().err(err);
      }
    }
    #[no_mangle] #[allow(non_snake_case)]
    fn $value_wrap(value: $abi_type) -> Abi<WrappedValue> {
      let value = IntoRuntime::into_runtime(value);
      Abi::into_abi(<$prop_type as Value>::wrapped(<$prop_type as Value>::item(value)))
    }
    #[no_mangle] #[allow(non_snake_case)]
    fn $value_unwrap(value: Abi<WrappedValue>) -> $abi_type {
      IntoAbi::into_abi(<$prop_type as Value>::Item::unwrapped(value.into_runtime_temporary().clone()))
    }
    define_drop! { Property<$prop_type>, $property_drop }
  };
}

macro_rules! define_vec_type {
  (
    $element_type:ty,
    $vec_new:ident,
    $vec_push:ident,
    $vec_len:ident,
    $vec_get:ident,
    $vec_drop:ident,
  ) => {
    #[no_mangle] #[allow(non_snake_case)]
    pub fn $vec_new() -> Abi<Vec<$element_type>> {
      Abi::into_abi(Vec::new())
    }
    #[no_mangle] #[allow(non_snake_case)]
    pub fn $vec_push(vec: Abi<Vec<$element_type>>, property: Abi<$element_type>) {
      let vec = vec.into_runtime_temporary();
      vec.push(property.into_runtime_temporary().clone());
    }
    #[no_mangle] #[allow(non_snake_case)]
    pub fn $vec_len(vec: Abi<Vec<$element_type>>) -> usize {
      vec.into_runtime_temporary().len()
    }
    #[no_mangle] #[allow(non_snake_case)]
    pub fn $vec_get(vec: Abi<Vec<$element_type>>, index: usize) -> Abi<$element_type> {
      Abi::into_abi(vec.into_runtime_temporary()[index].clone())
    }
    define_drop! { Vec<$element_type>, $vec_drop }
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
pub fn property_factory__commit_changes(factory: Abi<PropertyFactory>) {
  factory.into_runtime_temporary().commit_changes()
}

#[no_mangle] #[allow(non_snake_case)]
pub fn property_factory__count(factory: Abi<PropertyFactory>) -> usize {
  factory.into_runtime_temporary().count()
}

define_drop! { PropertyFactory, property_factory__drop }

/*************************************************************************
 * 
 *                              Property
 * 
 *************************************************************************/

define_property_type! {
  bool,
  bool,
  property_factory__new_property__boolean,
  property__boolean__get,
  property__boolean__set,
  property__boolean__freeze,
  property__boolean__weakref,
  property__boolean__bind,
  property__boolean__unbind,
  property__boolean__drop,
  wrapped_value__wrap_boolean,
  wrapped_value__unwrap_boolean,
}

define_property_type! {
  i32,
  i32,
  property_factory__new_property__int,
  property__int__get,
  property__int__set,
  property__int__freeze,
  property__int__weakref,
  property__int__bind,
  property__int__unbind,
  property__int__drop,
  wrapped_value__wrap_int,
  wrapped_value__unwrap_int,
}

define_property_type! {
  f64,
  f64,
  property_factory__new_property__float,
  property__float__get,
  property__float__set,
  property__float__freeze,
  property__float__weakref,
  property__float__bind,
  property__float__unbind,
  property__float__drop,
  wrapped_value__wrap_float,
  wrapped_value__unwrap_float,
}

define_property_type! {
  String,
  Abi<AbiBuffer>,
  property_factory__new_property__string,
  property__string__get,
  property__string__set,
  property__string__freeze,
  property__string__weakref,
  property__string__bind,
  property__string__unbind,
  property__string__drop,
  wrapped_value__wrap_string,
  wrapped_value__unwrap_string,
}

define_property_type! {
  Length,
  Abi<Length>,
  property_factory__new_property__length,
  property__length__get,
  property__length__set,
  property__length__freeze,
  property__length__weakref,
  property__length__bind,
  property__length__unbind,
  property__length__drop,
  wrapped_value__wrap_length,
  wrapped_value__unwrap_length,
}

define_property_type! {
  Brush,
  Abi<Brush>,
  property_factory__new_property__brush,
  property__brush__get,
  property__brush__set,
  property__brush__freeze,
  property__brush__weakref,
  property__brush__bind,
  property__brush__unbind,
  property__brush__drop,
  wrapped_value__wrap_brush,
  wrapped_value__unwrap_brush,
}

impl IntoAbi<Layout> for u8 {
  fn into_abi(item: Layout) -> Self { item as u8 }
}

impl IntoRuntime<Layout> for u8 {
  fn into_runtime(item: Self) -> Layout {
    const LAYOUT_COLUMN: u8 = Layout::Column as u8;
    match item {
      LAYOUT_COLUMN => Layout::Column,
      _ => Layout::Row,
    }
  }
}

define_property_type! {
  Layout,
  u8,
  property_factory__new_property__enum_layout,
  property__enum_layout__get,
  property__enum_layout__set,
  property__enum_layout__freeze,
  property__enum_layout__weakref,
  property__enum_layout__bind,
  property__enum_layout__unbind,
  property__enum_layout__drop,
  wrapped_value__wrap_enum_layout,
  wrapped_value__unwrap_enum_layout,
}

define_property_type! {
  Iter<i32>,
  Abi<Iter<i32>>,
  property_factory__new_property__iter,
  property__iter__get,
  property__iter__set,
  property__iter__freeze,
  property__iter__weakref,
  property__iter__bind,
  property__iter__unbind,
  property__iter__drop,
  wrapped_value__wrap_iter,
  wrapped_value__unwrap_iter,
}

/*************************************************************************
 * 
 *                  Box<dyn DynProperty> (PropertyWeakRef)
 * 
 *************************************************************************/

define_drop! { Box<dyn DynProperty>, property__weakref__drop }

/*************************************************************************
 * 
 *                               Vec<_>
 * 
 *************************************************************************/

define_vec_type! {
  Box<dyn DynProperty>,
  vec__weakref_property__new,
  vec__weakref_property__push,
  vec__weakref_property__len,
  vec__weakref_property__get,
  vec__weakref_property__drop,
}

define_vec_type! {
  WrappedValue,
  vec__wrapped_value__new,
  vec__wrapped_value__push,
  vec__wrapped_value__len,
  vec__wrapped_value__get,
  vec__wrapped_value__drop,
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
 
define_drop! { Length, length__drop }

/*************************************************************************
 * 
 *                                Brush
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
 
define_drop! { Brush, brush__drop }

/*************************************************************************
 * 
 *                                Iter
 * 
 *************************************************************************/

#[no_mangle] #[allow(non_snake_case)]
pub fn iter__from_int(value: i32) -> Abi<Iter<i32>> {
  Abi::into_abi(value.iter())
}

#[no_mangle] #[allow(non_snake_case)]
pub fn iter__next(iter: Abi<Iter<i32>>) -> Abi<WrappedValue> {
  let item = iter.into_runtime_temporary().next_wrapped();
  if let Some(item) = item {
    Abi::into_abi(item)
  } else {
    Abi::null()
  }
}

#[no_mangle] #[allow(non_snake_case)]
pub fn iter__to_string(iter: Abi<Iter<i32>>) -> Abi<AbiBuffer> {
  Abi::into_abi(AbiBuffer::from_string(format!("{:?}", iter.into_runtime_temporary())))
}
 
define_drop! { Iter<i32>, iter__drop }

/*************************************************************************
 * 
 *                             WrappedValue
 * 
 *************************************************************************/

#[no_mangle] #[allow(non_snake_case)]
fn wrapped_value__tag(value: Abi<WrappedValue>) -> u32 {
  const BOOLEAN: u32 = 0;
  const INT: u32 = 1;
  const FLOAT: u32 = 2;
  const STRING: u32 = 3;
  const LENGTH: u32 = 4;
  const BRUSH: u32 = 5;
  const ENUM_LAYOUT: u32 = 6;
  const ITER: u32 = 7;

  match value.into_runtime_temporary() {
    WrappedValue::Int(_) => INT,
    WrappedValue::String(_) => STRING,
    WrappedValue::Boolean(_) => BOOLEAN,
    WrappedValue::Float(_) => FLOAT,
    WrappedValue::Length(_) => LENGTH,
    WrappedValue::Brush(_) => BRUSH,
    WrappedValue::EnumLayout(_) => ENUM_LAYOUT,
    WrappedValue::Iter(_) => ITER,
  }
}

define_drop! { WrappedValue, wrapped_value__drop }
