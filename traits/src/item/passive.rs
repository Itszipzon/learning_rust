use crate::item::item::Item;

pub trait PassiveItem: Item {
    fn as_item(&self) -> &dyn Item;
    fn as_item_mut(&mut self) -> &mut dyn Item;
}