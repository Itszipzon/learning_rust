use crate::item::item::Item;

pub trait ActiveItem: Item {
  fn on_use(&mut self) -> bool;

  fn as_item(&self) -> &dyn Item;
  fn as_item_mut(&mut self) -> &mut dyn Item;
}