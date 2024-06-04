// i/o library
use std::io;

// random library, the one we added manually to Cargo.toml
use rand::Rng;

// main function (kind of like Java)
fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // mut = mutable
    let mut guess = String::new();

    // this is not mutable
    let example = 0;

    // also not mutable
    // 1..=100 translates to "1 to 100, inclusive"
    let secret_number = rand::thread_rng().gen_range(1..=100);

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    let intGuess: Result<i32, _> = guess.parse();

    println!("You guessed: {guess}, and that's {}", intGuess==Ok(secret_number));
    println!("Here's another example of printing something {example}, {}", 2*3)
}