use std::io;

fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");

    println!("guess: {guess}");

    operations_examples();
    booleans();
    chars();
    tuple();
    array();
}

fn operations_examples() {

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    println!("sum: {sum}, difference: {difference}, product: {product}, quotient: {quotient}, truncated: {truncated}, remainder: {remainder}")
}

fn booleans() {
    let t = true;

    let f: bool = false; // with explicit type annotation

    println!("For booleans : t: {t} and f: {f}")
}

fn chars() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("For chars c: {c}, z: {z}, heat_eyed_cat: {heart_eyed_cat}")
}

fn tuple() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x,y,z) = tup;
    let x = tup.0;

    println!("The value of y is {y}");
    println!("The value of x is {x}");

    let unit:() = ();
}

fn array() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    
    let a = [3; 5];
    
    let first = a[0];
    let second = a[1];
    
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}