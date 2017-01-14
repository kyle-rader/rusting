use std::io;

fn main() {
    println!("Welcome to Guess the number in Rust!");

    // let secret_number = 23;

    let mut guess = String::new();

    println!("Please input your guess.");

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
