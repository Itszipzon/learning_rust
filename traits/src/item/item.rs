use std::fmt::Debug;

use super::{active::ActiveItem, item_tags::ItemTag, passive::PassiveItem, trinket::Trinket};

pub trait Item: Debug {
  fn id(&self) -> u32;
  fn name(&self) -> String;
  fn description(&self) -> String;
  fn tags(&self) -> Vec<ItemTag>;

  fn as_active_item(&self) -> Option<&dyn ActiveItem>;
  fn as_active_item_mut(&mut self) -> Option<&mut dyn ActiveItem>;

  fn as_passive_item(&self) -> Option<&dyn PassiveItem>;
  fn as_passive_item_mut(&mut self) -> Option<&mut dyn PassiveItem>;

  fn as_trinket(&self) -> Option<&dyn Trinket>;
  fn as_trinket_mut(&mut self) -> Option<&mut dyn Trinket>;

  fn clone_box(&self) -> Box<dyn Item>;
}

impl Clone for Box<dyn Item> {
  fn clone(&self) -> Box<dyn Item> {
    self.clone_box()
  }
}
