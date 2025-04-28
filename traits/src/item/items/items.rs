use crate::item::item::Item;

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

impl Item for Sword {
  fn id(&self) -> u32 {
    todo!()
  }

  fn name(&self) -> String {
    todo!()
  }

  fn description(&self) -> String {
    todo!()
  }

  fn clone_box(&self) -> Box<dyn Item> {
    Box::new(self.clone())
  }
}
