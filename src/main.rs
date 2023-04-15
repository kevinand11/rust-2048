use crate::data::GameData;

mod data;

fn main() {
    println!("Hello, world!");

    let mut data = GameData {
        arr: [[0, 4, 0, 2], [8, 2, 0, 8], [0, 0, 0, 2], [2, 0, 2, 2]],
    };

    println!("Left\n{:?}", data);
    data.left();
    println!("{:?}", data);

    data = GameData {
        arr: [[2, 0, 4, 0], [8, 0, 2, 8], [2, 0, 0, 0], [2, 2, 0, 2]],
    };

    println!("\n\nRight\n{:?}", data);
    data.right();
    println!("{:?}", data);

    data = GameData {
        arr: [[2, 8, 2, 2], [0, 0, 0, 2], [4, 2, 0, 0], [0, 8, 0, 2]],
    };

    println!("\n\nUp\n{:?}", data);
    data.up();
    println!("{:?}", data);

    data = GameData {
        arr: [[2, 8, 2, 2], [0, 0, 0, 2], [4, 2, 0, 0], [0, 8, 0, 2]],
    };

    println!("\n\nDown\n{:?}", data);
    data.down();
    println!("{:?}", data);
}
