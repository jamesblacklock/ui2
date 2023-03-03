use std::{
	alloc::{alloc, dealloc, Layout},
	mem,
	marker::PhantomData, ptr, rc::Rc, ops::Deref, fmt,
};

use crate::{println, eprintln};

pub mod property;

#[macro_export]
#[cfg(target_arch = "wasm32")]
#[link(wasm_import_module = "runtime")]
extern "C" {
	fn __console_log(buf: Abi<AbiBuffer>, is_error: bool);
  fn __dispatch_function(fnptr: usize, argsptr: usize) -> usize;
  fn __drop_function(fnptr: usize);
  fn __random() -> f64;
}

#[cfg(not(target_arch = "wasm32"))] #[no_mangle]
unsafe extern "C" fn __dispatch_function(fnptr: usize, argsptr: usize) -> usize {
  let f: extern "C" fn(usize) -> usize = mem::transmute(fnptr);
  f(argsptr)
}

#[cfg(not(target_arch = "wasm32"))] #[no_mangle]
extern "C" fn __drop_function(fnptr: usize) {}

#[cfg(target_arch = "wasm32")]
pub fn console_log<S: Into<String>>(message: S, is_error: bool) {
  let buf = AbiBuffer::from_string(message.into());
  unsafe { __console_log(Abi::into_abi(buf), is_error); }
}

#[cfg(target_arch = "wasm32")]
pub fn random() -> f64 {
  unsafe { __random() }
}

#[repr(transparent)]
pub struct Abi<T>(usize, PhantomData<T>);

impl <T> Abi<T> {
  fn null() -> Abi<T> {
    Abi(0, PhantomData)
  }

  fn into_abi(object: T) -> Abi<T> {
    unsafe { Abi(mem::transmute(Box::leak(Box::new(object))), PhantomData) }
  }

  fn into_runtime(self) -> Box<T> {
    unsafe { Box::from_raw(std::mem::transmute(self.0)) }
  }

  fn into_runtime_temporary(self) -> &'static mut T {
    Box::leak(self.into_runtime())
  }
}

trait IntoRuntime<T> {
  fn into_runtime(item: Self) -> T;
}
impl <T: Clone + 'static> IntoRuntime<T> for Abi<T> {
  fn into_runtime(item: Self) -> T {
    item.into_runtime_temporary().clone()
  }
}
impl IntoRuntime<bool> for bool {
  fn into_runtime(item: bool) -> bool { item }
}
impl IntoRuntime<i32> for i32 {
  fn into_runtime(item: i32) -> i32 { item }
}
impl IntoRuntime<f64> for f64 {
  fn into_runtime(item: f64) -> f64 { item }
}
impl IntoRuntime<String> for Abi<AbiBuffer> {
  fn into_runtime(item: Abi<AbiBuffer>) -> String {
    item.into_runtime_temporary().to_string()
  }
}

trait IntoAbi<T> {
  fn into_abi(item: T) -> Self;
}
impl <T> IntoAbi<T> for Abi<T> {
  fn into_abi(item: T) -> Self { Abi::into_abi(item) }
}
impl IntoAbi<bool> for bool {
  fn into_abi(item: bool) -> bool { item }
}
impl IntoAbi<i32> for i32 {
  fn into_abi(item: i32) -> i32 { item }
}
impl IntoAbi<f64> for f64 {
  fn into_abi(item: f64) -> f64 { item }
}
impl IntoAbi<Rc<String>> for Abi<AbiBuffer> {
  fn into_abi(item: Rc<String>) -> Abi<AbiBuffer> {
    Abi::into_abi(AbiBuffer::from_string(item.deref().clone()))
  }
}

pub struct AbiBuffer {
  size: usize,
  ptr: *mut u8,
  layout: Option<Layout>,
}

impl AbiBuffer {
  fn new(size: usize) -> Self {
    if size == 0 {
      return AbiBuffer { ptr: 0 as *mut u8, size: 0, layout: None };
    }
    let layout = Layout::array::<u8>(size).unwrap();
    let ptr;
    unsafe {
      ptr = alloc(layout);
    }
    AbiBuffer { ptr, size, layout: Some(layout) }
  }
  fn from_string<S: Into<String>>(s: S) -> Self {
    let s = s.into();
    let size = s.len();
    let mut buf = AbiBuffer::new(size);
    if size == 0 {
      return buf;
    }
    unsafe {
      std::intrinsics::copy_nonoverlapping(&s.as_bytes()[0], buf.ptr, size);
    }
    buf
  }
  fn to_string(&self) -> String {
    let s = unsafe { &*ptr::slice_from_raw_parts(self.ptr, self.size) };
    String::from_utf8(s.to_vec()).unwrap()
  }
  pub fn len(&self) -> usize {
    self.size
  }
  pub fn ptr(&self) -> *mut u8 {
    self.ptr
  }
}

impl Drop for AbiBuffer {
  fn drop(&mut self) {
    if let Some(layout) = self.layout {
      unsafe { dealloc(self.ptr, layout) }
    }
  }
}

#[no_mangle] #[allow(non_snake_case)]
pub fn abi_buffer__len(abi_buffer: Abi<AbiBuffer>) -> usize {
  abi_buffer.into_runtime_temporary().len()
}

#[no_mangle] #[allow(non_snake_case)]
pub fn abi_buffer__new(size: usize) -> Abi<AbiBuffer> {
  Abi::into_abi(AbiBuffer::new(size))
}

#[no_mangle] #[allow(non_snake_case)]
pub fn abi_buffer__ptr(abi_buffer: Abi<AbiBuffer>) -> *mut u8 {
  abi_buffer.into_runtime_temporary().ptr()
}

#[no_mangle] #[allow(non_snake_case)]
pub fn abi_buffer__drop(abi_buffer: Abi<AbiBuffer>) {
  mem::drop(abi_buffer.into_runtime())
}

pub struct AbiResult<T = (), E = String>(Box<Result<T, E>>);

impl <T: Default, E> AbiResult<T, E> {
  pub fn new(value: T) -> Self {
    AbiResult(Box::new(Ok(value)))
  }

  pub fn ok(&mut self, value: T) {
    self.0 = Box::new(Ok(value));
  }

  pub fn err<Q: Into<E>>(&mut self, err: Q) {
    self.0 = Box::new(Err(err.into()));
  }

  pub fn is_ok(&self) -> bool {
    self.0.is_ok()
  }

  pub fn is_err(&self) -> bool {
    self.0.is_err()
  }
}

impl AbiResult<(), String> {
  pub fn message(&self) -> String {
    if self.0.is_err() {
      self.0.clone().unwrap_err()
    } else {
      String::new()
    }
  }
}

impl <T: Default, E> Default for AbiResult<T, E> {
  fn default() -> Self {
    AbiResult::new(Default::default())
  }
}

#[no_mangle] #[allow(non_snake_case)]
pub fn abi_result__new() -> Abi<AbiResult> {
  Abi::into_abi(AbiResult::default())
}

#[no_mangle] #[allow(non_snake_case)]
pub fn abi_result__is_ok(abi_result: Abi<AbiResult>) -> bool {
  abi_result.into_runtime_temporary().is_ok()
}

#[no_mangle] #[allow(non_snake_case)]
pub fn abi_result__is_err(abi_result: Abi<AbiResult>) -> bool {
  abi_result.into_runtime_temporary().is_err()
}

#[no_mangle] #[allow(non_snake_case)]
pub fn abi_result__message(abi_result: Abi<AbiResult>) -> Abi<AbiBuffer> {
  let message = abi_result.into_runtime_temporary().message();
  Abi::into_abi(AbiBuffer::from_string(message))
}

#[no_mangle] #[allow(non_snake_case)]
pub fn abi_result__drop(abi_result: Abi<AbiResult>) {
  mem::drop(abi_result.into_runtime())
}

#[derive(Debug)]
pub struct AbiFunction(usize);

impl AbiFunction {
  pub fn dispatch_usize<T: fmt::Debug>(&self, vec: Vec<T>) -> usize {
    let args = Abi::into_abi(vec);
    unsafe { __dispatch_function(self.0, args.0) }
  }
  pub fn dispatch_box<T: fmt::Debug, U: Clone + 'static>(&self, vec: Vec<T>) -> Option<U> {
    let p = self.dispatch_usize(vec);
    if p == 0 {
      None
    } else {
      let value: &mut U = Abi::into_runtime_temporary(Abi(p, PhantomData));
      Some(value.clone())
    }
  }
  pub fn dispatch_void<T: fmt::Debug>(&self, vec: Vec<T>) {
    self.dispatch_usize(vec);
  }
  pub fn is_null(&self) -> bool {
    self.0 == 0
  }
}

impl Drop for AbiFunction {
	fn drop(&mut self) {
		if self.0 != 0 {
			unsafe { __drop_function(self.0) }
		}
	}
}
