use crate::data::GameData;

mod data;

fn main() {
    println!("Hello, world!");

    let mut data = GameData::new();
    println!("Left\n{:?}", data);
    data.left();
    println!("{:?}", data);

    data = GameData::new();
    println!("\n\nRight\n{:?}", data);
    data.right();
    println!("{:?}", data);

    data = GameData::new();
    println!("\n\nUp\n{:?}", data);
    data.up();
    println!("{:?}", data);

    data = GameData::new();
    println!("\n\nDown\n{:?}", data);
    data.down();
    println!("{:?}", data);
}
