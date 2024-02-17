use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
        

    println!("You guessed: {guess}");

    let x = 2;
    let y = 4;

    println!("x = {x} and y + x = {} and y + 2 = {}", x + y, y + 2)


}