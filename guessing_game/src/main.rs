use std::io;

use rand::seq::IndexedRandom;

fn main() {
    let colors = ["orange", "banana", "grapes", "apple", "kiwi"];

    let mut rng = rand::rng();

    let random_val = colors.choose(&mut rng).unwrap().to_string();

    println!("Welcome to the guessing game");

    loop {
        let mut input = String::new();
        println!("Please enter your guess:");
        io::stdin().read_line(&mut input).expect("Invalid input");

        if input.trim() == random_val {
            println!("Your guess is correct: {}", input);
            break;
        } else {
            println!("Your guess is wrong");
        }
    }
}
