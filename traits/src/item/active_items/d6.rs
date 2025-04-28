use crate::item::{active::ActiveItem, item::Item, item_tags::ItemTag};

#[derive(Debug, Clone)]
pub struct D6 {}

impl D6 {
  pub fn new() -> Self {
    D6 {}
  }
}

impl ActiveItem for D6 {
  fn on_use(&mut self) -> bool {
    true
  }

  fn as_item(&self) -> &dyn Item {
    self as &dyn Item
  }

  fn as_item_mut(&mut self) -> &mut dyn Item {
    self as &mut dyn Item
  }
}

impl Item for D6 {
  fn id(&self) -> u32 {
    1
  }

  fn name(&self) -> String {
    "D6".to_string()
  }

  fn description(&self) -> String {
    "Rerolls all pickups on the ground.".to_string()
  }

  fn tags(&self) -> Vec<ItemTag> {
    vec![
      ItemTag::Offensive,
      ItemTag::NoCanTrip
    ]
  
  }

  fn as_active_item(&self) -> Option<&dyn ActiveItem> {
    Some(self)
  }

  fn as_active_item_mut(&mut self) -> Option<&mut dyn ActiveItem> {
    Some(self)
  }

  fn as_passive_item(&self) -> Option<&dyn crate::item::passive::PassiveItem> {
    None
  }

  fn as_passive_item_mut(&mut self) -> Option<&mut dyn crate::item::passive::PassiveItem> {
    None
  }

  fn as_trinket(&self) -> Option<&dyn crate::item::trinket::Trinket> {
    None
  }

  fn as_trinket_mut(&mut self) -> Option<&mut dyn crate::item::trinket::Trinket> {
    None
  }

  fn clone_box(&self) -> Box<dyn Item> {
    Box::new(self.clone())
  }
}
