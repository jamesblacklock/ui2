use super::{Value, ValueItem, WrappedValue};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Layout {
  Row = 0,
  Column = 1,
}

impl Value for Layout {
  type Item = Layout;
  fn default() -> Self { Layout::Row }
  fn item(value: Self) -> Self::Item { value }
  fn from_item(value: Self::Item) -> Self { value }
  fn wrapped(value: Self::Item) -> WrappedValue { WrappedValue::EnumLayout(value) }
}

impl ValueItem for Layout {
  fn unwrapped(value: WrappedValue) -> Self {
    value.unwrap_enum_layout()
  }
}
