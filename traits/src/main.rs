mod item;
mod entities;

use entities::player::Player;
use item::active_items::d6::D6;
use item::passive_items::sacred_heart::SacredHeart;
use item::trinkets::curved_horn::CurvedHorn;

fn main() {
    let mut p = Player::new("Player1".to_string());

    let d6 = D6::new();
    let sacred_heart = SacredHeart::new();
    let curved_horn = CurvedHorn::new();
    println!("{:#?}", p);

    p.add_item(Box::new(sacred_heart));

    println!("{:#?}", p);

    p.add_active_item(Box::new(d6));
    println!("{:#?}", p);

    p.add_trinket(Box::new(curved_horn));
    println!("{:#?}", p);
}
