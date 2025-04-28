use crate::item::{active::ActiveItem, item::Item, trinket::Trinket};

#[derive(Debug)]
pub struct Player {
  name: String,
  active_item: Option<Box<dyn ActiveItem>>,
  secondary_active_item: Option<Box<dyn ActiveItem>>,
  trinket: Option<Box<dyn Trinket>>,
  secondary_trinket: Option<Box<dyn Trinket>>,
  allow_secondary_active_item: bool,
  allow_secondary_trinket: bool,
  inventory: Vec<Box<dyn Item>>,

  selected_active_item_is_main: bool,
  selected_trinket_is_main: bool,
}

impl Player {
  pub fn new(name: String) -> Self {
    Player {
      name,
      active_item: None,
      secondary_active_item: None,
      trinket: None,
      secondary_trinket: None,
      inventory: Vec::new(),
      allow_secondary_active_item: false,
      allow_secondary_trinket: false,
      selected_active_item_is_main: true,
      selected_trinket_is_main: true,
    }
  }

  pub fn add_active_item(&mut self, item: Box<dyn ActiveItem>) {
    if !self.allow_secondary_active_item {
      self.active_item = Some(item);
      return;
    }

    if !self.active_item.is_some() {
      self.active_item = Some(item);
      return;
    }

    if !self.secondary_active_item.is_some() {
      self.secondary_active_item = Some(item);
      return;
    }

    if self.selected_active_item_is_main {
      self.active_item = Some(item);
    } else {
      self.secondary_active_item = Some(item);
    }
  }

  pub fn remove_active_item(&mut self) {
    if self.selected_active_item_is_main {
      self.active_item = None;
    } else {
      self.secondary_active_item = None;
    }
  }

  pub fn add_trinket(&mut self, item: Box<dyn Trinket>) {
    if !self.allow_secondary_trinket {
      self.trinket = Some(item);
      return;
    }

    if !self.trinket.is_some() {
      self.trinket = Some(item);
      return;
    }

    if !self.secondary_trinket.is_some() {
      self.secondary_trinket = Some(item);
      return;
    }

    if self.selected_trinket_is_main {
      self.trinket = Some(item);
    } else {
      self.secondary_trinket = Some(item);
    }
  }

  pub fn remove_trinket(&mut self) {
    if self.selected_trinket_is_main {
      self.trinket = None;
    } else {
      self.secondary_trinket = None;
    }
  }

  pub fn add_item(&mut self, item: Box<dyn Item>) {
    if item.as_active_item().is_some() {
      return;
    }

    self.inventory.push(item);
  }

  pub fn get_selected_active_item(&self) -> Option<&dyn ActiveItem> {
    if self.selected_active_item_is_main {
      return self.active_item.as_ref().unwrap().as_active_item();
    }
    self.secondary_active_item.as_ref().unwrap().as_active_item()
  }

  pub fn get_selected_trinket(&self) -> Option<&dyn Trinket> {
    if self.selected_trinket_is_main {
      return self.trinket.as_deref();
    }
    self.secondary_trinket.as_deref()
  }

  pub fn swap_selected_item(&mut self) {
    if self.selected_active_item_is_main && self.allow_secondary_active_item && self.secondary_active_item.is_some() {
      self.selected_active_item_is_main = false;
    } else {
      self.selected_active_item_is_main = true;
    }

    if self.selected_trinket_is_main && self.allow_secondary_trinket && self.secondary_trinket.is_some() {
      self.selected_trinket_is_main = false;
    } else {
      self.selected_trinket_is_main = true;
    }
  }
}
