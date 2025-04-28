mod item;
mod entities;

use entities::player::Player;
use item::items::equipments::sword::Sword;

fn main() {
    let mut p = Player::new("Player1".to_string());

    let sword = Sword::new(
        1,
        "Sword".to_string(),
        "A sharp sword".to_string(),
        "A sword that is sharp and shiny.".to_string(),
    );

    p.add_item(Box::new(sword));
    println!("{:#?}", p);
}
