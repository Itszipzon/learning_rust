use crate::item::item::Item;
use crate::item::items::equipment::Equipment;

#[derive(Debug, Clone)]
pub struct Sword {
  id: u32,
  name: String,
  display_name: String,
  description: String,
}

impl Sword {
  pub fn new(id: u32, name: String, display_name: String, description: String) -> Self {
    Sword {
      id,
      name,
      display_name,
      description,
    }
  }
}

impl Equipment for Sword {

  fn name(&self) -> String {
    self.display_name.clone()
  }

  fn as_item(&self) -> Option<&dyn Item> {
    Some(self)
  }
}

impl Item for Sword {
  fn id(&self) -> u32 {
    self.id
  }

  fn name(&self) -> String {
    self.name.clone()
  }

  fn description(&self) -> String {
    self.description.clone()
  }

  fn clone_box(&self) -> Box<dyn Item> {
    Box::new(self.clone())
  }

  fn as_equipment(&self) -> Option<&dyn Equipment> {
    Some(self)
  }
}
