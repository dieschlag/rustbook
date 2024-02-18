use std::io;

fn main() {

    const CONVERSION_COEFF: f64 = 5.0 / 9.0 ;
    const CONVERSION_CST: f64 = 32.0 ;

    println!("Please type the Farenheit value: ");

    let mut farenheit_value = String::new();

    io::stdin()
        .read_line(&mut farenheit_value)
        .expect("Failed to read line");

    let farenheit_value: f64 = farenheit_value.trim().parse().expect("Please enter a float number"); 

    let celsius_value = (farenheit_value - CONVERSION_CST) * CONVERSION_COEFF;

    println!("The corresponding Celsius value is: {celsius_value}");

}
