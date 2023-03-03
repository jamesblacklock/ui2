#![allow(unused)]

#[macro_export]
#[cfg(target_arch = "wasm32")]
macro_rules! println {
  ($($t:tt)*) => (crate::abi::console_log(&format_args!($($t)*).to_string(), false))
}

#[macro_export]
#[cfg(target_arch = "wasm32")]
macro_rules! eprintln {
  ($($t:tt)*) => (crate::abi::console_log(&format_args!($($t)*).to_string(), true))
}

#[macro_export]
#[cfg(not(target_arch = "wasm32"))]
macro_rules! println {
  ($($t:tt)*) => (std::println!($($t)*))
}

#[macro_export]
#[cfg(not(target_arch = "wasm32"))]
macro_rules! eprintln {
  ($($t:tt)*) => (std::eprintln!($($t)*))
}

#[cfg(not(target_arch = "wasm32"))]
fn random() -> usize {
  use rand::Rng;
  rand::thread_rng().gen()
}

#[cfg(target_arch = "wasm32")]
fn random() -> usize {
  let n = crate::abi::random();
  (n * usize::MAX as f64) as usize
}

pub mod property;

pub mod abi;
