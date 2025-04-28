use crate::item::item::Item;

#[derive(Debug)]
pub struct Player {
  name: String,
  inventory: Vec<Box<dyn Item>>,
}

impl Player {
  pub fn new(name: String) -> Self {
    Player {
      name,
      inventory: Vec::new(),
    }
  }

  pub fn add_item(&mut self, item: Box<dyn Item>) {
    self.inventory.push(item);
  }

  pub fn remove_item(&mut self, item_id: u32) -> Option<Box<dyn Item>> {
    if let Some(pos) = self.inventory.iter().position(|x| x.id() == item_id) {
      Some(self.inventory.remove(pos))
    } else {
      None
    }
  }
}
