use std::fmt::Debug;

use super::items::equipment::Equipment;

pub trait Item: Debug {
  fn id(&self) -> u32;
  fn name(&self) -> String;
  fn description(&self) -> String;

  fn as_equipment(&self) -> Option<&dyn Equipment>;

  fn clone_box(&self) -> Box<dyn Item>;
}

impl Clone for Box<dyn Item> {
  fn clone(&self) -> Box<dyn Item> {
    self.clone_box()
  }
}
