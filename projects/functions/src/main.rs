fn main() {

    //hello this is a comment

    //if the comment is too long we need to add // 
    //at the begginig of each line

    println!("Hello, world!");
    another_function();
    another_function_with_argument(5);
    print_labeled_measurement(5, 'h');
    expression();
    let x = five();
    println!("The value of x is {x}");
    let x = plus_one(x);
    println!("The value of x is {x}")
}

fn another_function() {
    println!("Another function.");
}

fn another_function_with_argument(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn expression() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}