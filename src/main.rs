use crate::data::GameData;
use std::io;

mod data;

fn main() {
    println!("Welcome to 2048");

    let mut game = GameData::new();

    loop {
        game.print();

        if game.is_busted() {
            println!("Game over!");
            break;
        }

        if game.is_won() {
            println!("Game won!");
            break;
        }

        println!("Enter w to move up, s to move down, a to move left, d to move right, q to quit");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input = input.to_lowercase();
        let input = input.trim();

        match input {
            "w" => game.up(),
            "a" => game.left(),
            "s" => game.down(),
            "d" => game.right(),
            "q" => break,
            _ => {
                println!("Unrecognized input");
                continue;
            }
        };
    }
}
