use crate::item::{active::ActiveItem, item::Item, item_tags::ItemTag, passive::PassiveItem, trinket::Trinket};

#[derive(Debug, Clone)]
pub struct SacredHeart {}

impl SacredHeart {
  pub fn new() -> Self {
    SacredHeart {}
  }
}

impl Item for SacredHeart {
  fn id(&self) -> u32 {
    2
  }

  fn name(&self) -> String {
    "Sacred Heart".to_string()
  }

  fn description(&self) -> String {
    "A heart that is sacred.".to_string()
  }

  fn tags(&self) -> Vec<crate::item::item_tags::ItemTag> {
    vec![
      ItemTag::Summonable,
      ItemTag::Offensive,
      ItemTag::Angel
    ]
  }

  fn as_active_item(&self) -> Option<&dyn ActiveItem> {
    None
  }

  fn as_active_item_mut(&mut self) -> Option<&mut dyn ActiveItem> {
    None
  }

  fn as_passive_item(&self) -> Option<&dyn PassiveItem> {
    Some(self)
  }

  fn as_passive_item_mut(&mut self) -> Option<&mut dyn PassiveItem> {
    Some(self)
  }

  fn as_trinket(&self) -> Option<&dyn Trinket> {
    None
  }

  fn as_trinket_mut(&mut self) -> Option<&mut dyn Trinket> {
    None
  }

  fn clone_box(&self) -> Box<dyn Item> {
    Box::new(self.clone())
  }
}

impl PassiveItem for SacredHeart {
  fn as_item(&self) -> &dyn Item {
    self as &dyn Item
  }

  fn as_item_mut(&mut self) -> &mut dyn Item {
    self as &mut dyn Item
  }
}
