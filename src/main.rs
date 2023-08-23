use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    //create mutable(can change) variable that bount to empty string
    let mut guess = String::new();

    //handle user inputs
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");

}
