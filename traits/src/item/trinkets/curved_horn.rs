use crate::item::{
  active::ActiveItem, item::Item, item_tags::ItemTag, passive::PassiveItem, trinket::Trinket,
};

#[derive(Debug, Clone)]
pub struct CurvedHorn {}

impl CurvedHorn {
  pub fn new() -> Self {
    CurvedHorn {}
  }
}

impl Item for CurvedHorn {
  fn id(&self) -> u32 {
    3
  }

  fn name(&self) -> String {
    "Curved Horn".to_string()
  }

  fn description(&self) -> String {
    "Curved Horn".to_string()
  }

  fn tags(&self) -> Vec<ItemTag> {
    vec![]
  }

  fn as_active_item(&self) -> Option<&dyn ActiveItem> {
    None
  }

  fn as_active_item_mut(&mut self) -> Option<&mut dyn ActiveItem> {
    None
  }

  fn as_passive_item(&self) -> Option<&dyn PassiveItem> {
    None
  }

  fn as_passive_item_mut(&mut self) -> Option<&mut dyn PassiveItem> {
    None
  }

  fn as_trinket(&self) -> Option<&dyn Trinket> {
    Some(self)
  }

  fn as_trinket_mut(&mut self) -> Option<&mut dyn Trinket> {
    Some(self)
  }

  fn clone_box(&self) -> Box<dyn Item> {
    Box::new(self.clone())
  }
}

impl Trinket for CurvedHorn {
  fn as_item(&self) -> &dyn Item {
    self
  }

  fn as_item_mut(&mut self) -> &mut dyn Item {
    self
  }
}
