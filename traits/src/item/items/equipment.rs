use crate::item::item::Item;

pub trait Equipment {
    fn name(&self) -> String;

    fn as_item(&self) -> Option<&dyn Item>;
}