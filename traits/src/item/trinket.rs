use crate::item::item::Item;

pub trait Trinket: Item {
  fn as_item(&self) -> &dyn Item;
  fn as_item_mut(&mut self) -> &mut dyn Item;
}